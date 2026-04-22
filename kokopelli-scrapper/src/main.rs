use std::collections::{HashMap, HashSet};
use std::time::Duration;

use anyhow::{Context, Result};
use chrono::Utc;
use reqwest::Client;
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};

const BASE_FR: &str = "https://kokopelli-semences.fr/fr";
const DELAY_MS: u64 = 300;

/// Groups (slugs) under /fr/c/semences/potageres/
const GROUPS: &[(&str, &str)] = &[
    ("bulbes", "Bulbes"),
    ("engrais-verts", "Engrais Verts"),
    ("legumes-feuilles", "Légumes-Feuilles"),
    ("legumes-fruits", "Légumes-Fruits"),
    ("legumes-racines", "Légumes-Racines"),
    ("plantes-a-grains", "Plantes à Grains"),
];

// ---------------------------------------------------------------------------
// Data model
// ---------------------------------------------------------------------------

#[derive(Debug, Serialize, Deserialize)]
struct Variety {
    name_fr: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    name_en: Option<String>,
    url_fr: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    url_en: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    product_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    maturity: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Vegetable {
    name_fr: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    name_en: Option<String>,
    url_fr: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    url_en: Option<String>,
    slug_fr: String,
    varieties: Vec<Variety>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Group {
    name_fr: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    name_en: Option<String>,
    url_fr: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    url_en: Option<String>,
    slug_fr: String,
    vegetables: Vec<Vegetable>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Catalog {
    scraped_at: String,
    groups: Vec<Group>,
}

// ---------------------------------------------------------------------------
// HTTP helpers
// ---------------------------------------------------------------------------

async fn get_html(client: &Client, url: &str) -> Result<Html> {
    tokio::time::sleep(Duration::from_millis(DELAY_MS)).await;
    let resp = client
        .get(url)
        .send()
        .await
        .with_context(|| format!("GET {url}"))?;
    let text = resp
        .text()
        .await
        .with_context(|| format!("reading body of {url}"))?;
    Ok(Html::parse_document(&text))
}

/// Extract hreflang="en" URL from a page.
fn extract_hreflang_en(doc: &Html) -> Option<String> {
    let sel = Selector::parse(r#"link[rel="alternate"][hreflang="en"]"#).ok()?;
    doc.select(&sel)
        .next()
        .and_then(|el| el.value().attr("href"))
        .map(String::from)
}

/// Count pages using the last page-number link in pagination.
fn extract_last_page(doc: &Html) -> u32 {
    let Ok(sel) = Selector::parse("a.page-link, a[href*='?page='], a[href*='&page=']") else {
        return 1;
    };
    let mut max = 1u32;
    for el in doc.select(&sel) {
        if let Some(href) = el.value().attr("href") {
            if let Some(pos) = href.find("page=") {
                let tail = &href[pos + 5..];
                let num_str: String = tail.chars().take_while(|c| c.is_ascii_digit()).collect();
                if let Ok(n) = num_str.parse::<u32>() {
                    if n > max {
                        max = n;
                    }
                }
            }
        }
    }
    max
}

// ---------------------------------------------------------------------------
// Phase 2: Discover vegetable URLs for a group
// ---------------------------------------------------------------------------

/// Returns a list of (slug, name_fr, url_fr) for vegetable sub-category pages
/// found while paginatin a group page.
async fn discover_vegetables(
    client: &Client,
    group_slug: &str,
) -> Result<Vec<(String, String, String)>> {
    let base_url = format!("{BASE_FR}/c/semences/potageres/{group_slug}");
    // pattern we are looking for in anchor hrefs
    let prefix = format!("/fr/c/semences/potageres/{group_slug}/");

    let first_page = get_html(client, &base_url).await?;
    let last = extract_last_page(&first_page);

    let mut seen: HashSet<String> = HashSet::new();
    let mut ordered: Vec<(String, String, String)> = Vec::new();

    let process_page = |doc: Html,
                        seen: &mut HashSet<String>,
                        ordered: &mut Vec<(String, String, String)>| {
        let Ok(sel) = Selector::parse("a[href]") else {
            return;
        };
        for el in doc.select(&sel) {
            let href = match el.value().attr("href") {
                Some(h) => h,
                None => continue,
            };
            if !href.starts_with(&prefix) {
                continue;
            }
            // must be exactly one segment deeper (no further slash after the slug)
            let tail = &href[prefix.len()..];
            if tail.is_empty() || tail.contains('/') {
                continue;
            }
            let slug = tail.split('?').next().unwrap_or(tail).to_string();
            if seen.insert(slug.clone()) {
                // try to grab the visible badge text next to this link
                let name = el
                    .text()
                    .collect::<String>()
                    .trim()
                    .to_uppercase()
                    .to_string();
                let url = format!("https://kokopelli-semences.fr{href}");
                ordered.push((slug, name, url));
            }
        }
    };

    process_page(first_page, &mut seen, &mut ordered);

    for page in 2..=last {
        let url = format!("{base_url}?page={page}");
        let doc = get_html(client, &url).await?;
        process_page(doc, &mut seen, &mut ordered);
    }

    Ok(ordered)
}

// ---------------------------------------------------------------------------
// Phase 3a: Scrape FR varieties for a vegetable page
// ---------------------------------------------------------------------------

struct FrVariety {
    name_fr: String,
    url_fr: String,
    product_code: Option<String>,
    maturity: Option<String>,
}

async fn scrape_fr_varieties(client: &Client, veg_url_fr: &str) -> Result<Vec<FrVariety>> {
    let first_page = get_html(client, veg_url_fr).await?;
    let last = extract_last_page(&first_page);
    let mut varieties = scrape_fr_varieties_from_page(&first_page);

    for page in 2..=last {
        let url = format!("{}?page={page}", veg_url_fr);
        let doc = get_html(client, &url).await?;
        varieties.extend(scrape_fr_varieties_from_page(&doc));
    }

    Ok(varieties)
}

fn scrape_fr_varieties_from_page(doc: &Html) -> Vec<FrVariety> {
    // Product cards: each is an <article> or similar container, but the most
    // reliable cross-page selector is the short-name link to /fr/p/...
    let Ok(link_sel) = Selector::parse(r#"a[href^="/fr/p/"]"#) else {
        return vec![];
    };
    // Maturity text lives in a sibling span with class like "maturity" or is
    // a text node in a parent element. We use a broad selector and match by
    // proximity: the closest text node containing a known maturity keyword.
    let maturity_keywords = ["Très Précoce", "Précoce", "Mi-Saison", "Tardive"];

    // Collect all product links (short name, no duplicates)
    let mut seen_urls: HashSet<String> = HashSet::new();
    let mut results: Vec<FrVariety> = Vec::new();

    for el in doc.select(&link_sel) {
        let href = match el.value().attr("href") {
            Some(h) => h,
            None => continue,
        };
        let abs_url = format!("https://kokopelli-semences.fr{href}");
        if !seen_urls.insert(abs_url.clone()) {
            continue;
        }

        let name_fr = el.text().collect::<String>().trim().to_string();
        if name_fr.is_empty() {
            continue;
        }

        // Product code: first path segment after /fr/p/ before the first '-'
        // e.g. /fr/p/P2630-Bou-Tozzina → "P2630"
        let product_code = href
            .trim_start_matches("/fr/p/")
            .split('-')
            .next()
            .map(String::from);

        // Maturity: look for known keywords in the text of the parent element tree.
        // We walk up through the fragment using the raw element's parent in the
        // element tree. Because `scraper` doesn't expose parent navigation directly,
        // we search for the closest <span> or text node near this <a>.
        // Pragmatic approach: look for a <span> inside the same product card.
        // Cards contain the maturity as a text node. We search the whole page for
        // the pattern by finding the maturity span immediately preceding this link.
        let maturity = find_maturity_near(doc, href, &maturity_keywords);

        results.push(FrVariety {
            name_fr,
            url_fr: abs_url,
            product_code,
            maturity,
        });
    }

    results
}

/// Crude heuristic: find the maturity text associated with a product by
/// looking for a known keyword within a window of text nodes around the
/// product link href in the raw HTML.
fn find_maturity_near(doc: &Html, href: &str, keywords: &[&str]) -> Option<String> {
    // We serialise the fragment of HTML around the product and scan for keywords.
    let html_str = doc.html();
    let needle = format!("href=\"{href}\"");
    let pos = html_str.find(needle.as_str())?;
    // Look in a window of ~800 bytes before this position for a maturity keyword
    let start = pos.saturating_sub(800);
    let window = &html_str[start..pos];
    // Check longest keyword first (Très Précoce must come before Précoce)
    for &kw in keywords {
        if window.contains(kw) {
            return Some(kw.to_string());
        }
    }
    None
}

// ---------------------------------------------------------------------------
// Phase 3b: Scrape EN variety names for a vegetable page
// ---------------------------------------------------------------------------

async fn scrape_en_variety_names(client: &Client, veg_url_en: &str) -> Result<Vec<(String, String)>> {
    let first_page = get_html(client, veg_url_en).await?;
    let last = extract_last_page(&first_page);
    let mut names = scrape_en_names_from_page(&first_page);

    for page in 2..=last {
        let url = format!("{}?page={page}", veg_url_en);
        let doc = get_html(client, &url).await?;
        names.extend(scrape_en_names_from_page(&doc));
    }

    Ok(names)
}

fn scrape_en_names_from_page(doc: &Html) -> Vec<(String, String)> {
    let Ok(link_sel) = Selector::parse(r#"a[href^="/en/p/"]"#) else {
        return vec![];
    };
    let mut seen: HashSet<String> = HashSet::new();
    let mut results = Vec::new();

    for el in doc.select(&link_sel) {
        let href = match el.value().attr("href") {
            Some(h) => h,
            None => continue,
        };
        let abs_url = format!("https://kokopelli-semences.fr{href}");
        if !seen.insert(abs_url.clone()) {
            continue;
        }
        let name = el.text().collect::<String>().trim().to_string();
        if name.is_empty() {
            continue;
        }
        results.push((name, abs_url));
    }
    results
}

// ---------------------------------------------------------------------------
// Phase 4: Extract vegetable name from EN page h1
// ---------------------------------------------------------------------------

async fn fetch_en_vegetable_name(client: &Client, url_en: &str) -> Result<Option<String>> {
    let doc = get_html(client, url_en).await?;
    let Ok(sel) = Selector::parse("h1") else {
        return Ok(None);
    };
    Ok(doc
        .select(&sel)
        .next()
        .map(|el| el.text().collect::<String>().trim().to_string()))
}

// ---------------------------------------------------------------------------
// Orchestration
// ---------------------------------------------------------------------------

async fn scrape_group(client: &Client, slug: &str, name_fr: &str) -> Result<Group> {
    let url_fr = format!("{BASE_FR}/c/semences/potageres/{slug}");
    eprintln!("[group] {name_fr}");

    // Parallel-friendly: fetch group page for hreflang
    let group_doc = get_html(client, &url_fr).await?;
    let url_en_opt = extract_hreflang_en(&group_doc);
    let name_en = if let Some(ref url_en) = url_en_opt {
        let en_doc = get_html(client, url_en).await?;
        match Selector::parse("h1") {
            Ok(sel) => en_doc
                .select(&sel)
                .next()
                .map(|el| el.text().collect::<String>().trim().to_string()),
            Err(_) => None,
        }
    } else {
        None
    };

    // Discover vegetable sub-categories from FR listing pages
    let veg_entries = discover_vegetables(client, slug).await?;

    let mut vegetables: Vec<Vegetable> = Vec::new();

    for (veg_slug, veg_name_fr, veg_url_fr) in veg_entries {
        // Scrape FR varieties
        let fr_varieties = scrape_fr_varieties(client, &veg_url_fr).await?;

        // Get EN url for this vegetable via hreflang on its FR page
        let veg_page_doc = get_html(client, &veg_url_fr).await?;
        let veg_url_en = extract_hreflang_en(&veg_page_doc);

        // Get EN vegetable name and EN varieties
        let (veg_name_en, en_map) = if let Some(ref url_en) = veg_url_en {
            let name_en = fetch_en_vegetable_name(client, url_en).await?;
            let en_names = scrape_en_variety_names(client, url_en).await?;
            (name_en, en_names)
        } else {
            (None, vec![])
        };

        eprintln!(
            "  [{slug}/{veg_slug}] {} FR varieties | {} EN varieties",
            fr_varieties.len(),
            en_map.len()
        );

        if !en_map.is_empty() && en_map.len() != fr_varieties.len() {
            eprintln!(
                "  WARNING: FR/EN count mismatch for {veg_slug}: {} vs {}",
                fr_varieties.len(),
                en_map.len()
            );
        }

        // Build a position-indexed map for EN
        let en_by_pos: HashMap<usize, &(String, String)> = en_map
            .iter()
            .enumerate()
            .map(|(i, v)| (i, v))
            .collect();

        let varieties: Vec<Variety> = fr_varieties
            .into_iter()
            .enumerate()
            .map(|(i, fr)| {
                let (name_en, url_en) = en_by_pos
                    .get(&i)
                    .map(|(n, u)| (Some(n.clone()), Some(u.clone())))
                    .unwrap_or((None, None));
                Variety {
                    name_fr: fr.name_fr,
                    name_en,
                    url_fr: fr.url_fr,
                    url_en,
                    product_code: fr.product_code,
                    maturity: fr.maturity,
                }
            })
            .collect();

        vegetables.push(Vegetable {
            name_fr: veg_name_fr,
            name_en: veg_name_en,
            url_fr: veg_url_fr,
            url_en: veg_url_en,
            slug_fr: veg_slug,
            varieties,
        });
    }

    Ok(Group {
        name_fr: name_fr.to_string(),
        name_en,
        url_fr,
        url_en: url_en_opt,
        slug_fr: slug.to_string(),
        vegetables,
    })
}

#[tokio::main]
async fn main() -> Result<()> {
    let client = Client::builder()
        .user_agent("kokopelli-scrapper/0.1.0")
        .timeout(Duration::from_secs(30))
        .build()?;

    let mut groups: Vec<Group> = Vec::new();

    for (slug, name_fr) in GROUPS {
        let group = scrape_group(&client, slug, name_fr).await?;
        groups.push(group);
    }

    let catalog = Catalog {
        scraped_at: Utc::now().to_rfc3339(),
        groups,
    };

    let json = serde_json::to_string_pretty(&catalog)?;
    std::fs::write("output.json", &json)?;
    eprintln!("Done → output.json");

    Ok(())
}
