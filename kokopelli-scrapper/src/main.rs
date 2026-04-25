use std::collections::HashMap;
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
// Phase 2: Discover vegetable URLs via product-page breadcrumbs
// ---------------------------------------------------------------------------

/// Strips the Kokopelli origin from an href, returning the path.
/// Works for both absolute (`https://kokopelli-semences.fr/fr/...`) and
/// already-relative (`/fr/...`) href values.
fn to_path(href: &str) -> &str {
    href.strip_prefix("https://kokopelli-semences.fr")
        .unwrap_or(href)
}

// ---------------------------------------------------------------------------
// Phase 3a: Scrape FR variety details from a listing page
// ---------------------------------------------------------------------------

struct FrVariety {
    name_fr: String,
    url_fr: String,
    product_code: Option<String>,
    maturity: Option<String>,
}

fn scrape_fr_varieties_from_page(doc: &Html) -> Vec<FrVariety> {
    // Product cards have TWO links to the same URL per product:
    //   1. Full title link:  "Oignon Rouge Long de Florence"  (first occurrence)
    //   2. Buy-button link:  "Rouge Long de Florence"         (second occurrence)
    // We want the SHORT variety name, so we keep-last: each duplicate URL
    // overwrites the name_fr stored previously.
    // hrefs may be absolute (https://kokopelli-semences.fr/fr/p/...) or relative.
    let Ok(link_sel) = Selector::parse("a[href]") else {
        return vec![];
    };
    let maturity_keywords = ["Très Précoce", "Précoce", "Mi-Saison", "Tardive"];

    // url → index into `results`
    let mut url_index: HashMap<String, usize> = HashMap::new();
    let mut results: Vec<FrVariety> = Vec::new();

    for el in doc.select(&link_sel) {
        let href = match el.value().attr("href") {
            Some(h) => h,
            None => continue,
        };
        let path = to_path(href);
        if !path.starts_with("/fr/p/") {
            continue;
        }
        let abs_url = if href.starts_with("http") {
            href.to_string()
        } else {
            format!("https://kokopelli-semences.fr{href}")
        };
        let name_fr = el.text().collect::<String>().trim().to_string();
        if name_fr.is_empty() {
            continue;
        }

        if let Some(&idx) = url_index.get(&abs_url) {
            // Second (or later) occurrence → shorter buy-button text preferred
            if name_fr.len() < results[idx].name_fr.len() {
                results[idx].name_fr = name_fr;
            }
            continue;
        }

        // First occurrence
        let product_code = path
            .trim_start_matches("/fr/p/")
            .split('-')
            .next()
            .map(String::from);
        let maturity = find_maturity_near(doc, href, &maturity_keywords);
        let idx = results.len();
        url_index.insert(abs_url.clone(), idx);
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

/// Collects all FR variety details from all listing pages of a group.
/// Group listing pages are server-side rendered (reqwest can see product links).
async fn collect_fr_varieties_from_group(
    client: &Client,
    group_slug: &str,
) -> Result<Vec<FrVariety>> {
    let base_url = format!("{BASE_FR}/c/semences/potageres/{group_slug}");
    let first_page = get_html(client, &base_url).await?;
    let last = extract_last_page(&first_page);
    eprintln!("  [collect/{group_slug}] {last} listing page(s)");

    let mut varieties = scrape_fr_varieties_from_page(&first_page);
    eprintln!(
        "  [collect/{group_slug}] page 1 → {} varieties so far",
        varieties.len()
    );
    for page in 2..=last {
        let url = format!("{base_url}?page={page}");
        let doc = get_html(client, &url).await?;
        let page_vars = scrape_fr_varieties_from_page(&doc);
        varieties.extend(page_vars);
        eprintln!(
            "  [collect/{group_slug}] page {page} → {} varieties so far",
            varieties.len()
        );
    }

    Ok(varieties)
}

/// Extracts the vegetable sub-category from the breadcrumb of a product page.
/// Returns `(slug, name_fr, url_fr)` for the last breadcrumb link whose path
/// matches `/fr/c/semences/potageres/{group_slug}/{veg_slug}` exactly.
fn extract_vegetable_from_breadcrumb(
    doc: &Html,
    veg_prefix: &str,
) -> Option<(String, String, String)> {
    let Ok(sel) = Selector::parse("a[href]") else {
        return None;
    };
    let mut result: Option<(String, String, String)> = None;
    for el in doc.select(&sel) {
        let Some(href) = el.value().attr("href") else {
            continue;
        };
        let path = to_path(href);
        if !path.starts_with(veg_prefix) {
            continue;
        }
        let tail = &path[veg_prefix.len()..];
        if tail.is_empty() || tail.contains('/') {
            continue;
        }
        let slug = tail.split('?').next().unwrap_or(tail).to_string();
        let name = el.text().collect::<String>().trim().to_string();
        let abs_url = if href.starts_with("http") {
            href.to_string()
        } else {
            format!("https://kokopelli-semences.fr{href}")
        };
        result = Some((slug, name, abs_url));
    }
    result
}

// ---------------------------------------------------------------------------
// Phase 2+3: Discover vegetables and collect variety details in one pass
// ---------------------------------------------------------------------------

/// Visits each product page in the group to extract the vegetable breadcrumb
/// and hreflang EN product URL, then groups the FR varieties by vegetable.
///
/// Returns: ordered list of `(veg_slug, veg_name_fr, veg_url_fr, varieties)`
/// where each `varieties` element is `(FrVariety, Option<en_product_url>)`.
async fn discover_vegetables_with_varieties(
    client: &Client,
    group_slug: &str,
) -> Result<Vec<(String, String, String, Vec<(FrVariety, Option<String>)>)>> {
    let veg_prefix = format!("/fr/c/semences/potageres/{group_slug}/");

    // Step 1: collect all variety details from the group listing pages
    // (these ARE server-side rendered — reqwest can see product links).
    let all_varieties = collect_fr_varieties_from_group(client, group_slug).await?;
    eprintln!(
        "  [discover/{group_slug}] visiting {} product pages for breadcrumbs",
        all_varieties.len()
    );

    // Step 2: visit each product page to get the vegetable breadcrumb and EN URL.
    // veg_slug → index in `ordered`
    let mut veg_index: HashMap<String, usize> = HashMap::new();
    let mut ordered: Vec<(String, String, String, Vec<(FrVariety, Option<String>)>)> = Vec::new();

    for variety in all_varieties {
        let product_doc = get_html(client, &variety.url_fr).await?;

        let veg = extract_vegetable_from_breadcrumb(&product_doc, &veg_prefix);
        let en_url = extract_hreflang_en(&product_doc);

        match veg {
            Some((slug, name, url)) => {
                if let Some(&idx) = veg_index.get(&slug) {
                    ordered[idx].3.push((variety, en_url));
                } else {
                    eprintln!("  [discover/{group_slug}] vegetable: {name} ({slug})");
                    let idx = ordered.len();
                    veg_index.insert(slug.clone(), idx);
                    ordered.push((slug, name, url, vec![(variety, en_url)]));
                }
            }
            None => {
                eprintln!(
                    "  [discover/{group_slug}] no breadcrumb for {}",
                    variety.url_fr
                );
            }
        }
    }

    Ok(ordered)
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

    // Fetch group page for hreflang / EN group name
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

    // Collect all varieties from group listing pages and assign each to its
    // vegetable sub-category via the product-page breadcrumb.
    // Also captures the hreflang EN product URL for each variety.
    let veg_entries = discover_vegetables_with_varieties(client, slug).await?;

    let mut vegetables: Vec<Vegetable> = Vec::new();

    for (veg_slug, veg_name_fr, veg_url_fr, var_entries) in veg_entries {
        // Fetch the vegetable category page only for hreflang + EN name.
        // (Product links on this page are JS-rendered, so we don't scrape them.)
        let veg_page_doc = get_html(client, &veg_url_fr).await?;
        let veg_url_en = extract_hreflang_en(&veg_page_doc);
        let veg_name_en = if let Some(ref url_en) = veg_url_en {
            fetch_en_vegetable_name(client, url_en).await?
        } else {
            None
        };

        eprintln!(
            "  [{slug}/{veg_slug}] {} varieties | EN veg name: {:?}",
            var_entries.len(),
            veg_name_en
        );

        // Build Variety list. EN variety name is derived from the EN product
        // URL slug (no extra request needed).
        let varieties: Vec<Variety> = var_entries
            .into_iter()
            .map(|(fr, en_url)| {
                // /en/p/P4521-Long-Red-Florence  →  "Long Red Florence"
                let name_en = en_url.as_ref().and_then(|url| {
                    url.split("/en/p/")
                        .nth(1)
                        .and_then(|slug| slug.splitn(2, '-').nth(1))
                        .map(|name_part| name_part.replace('-', " "))
                });
                Variety {
                    name_fr: fr.name_fr,
                    name_en,
                    url_fr: fr.url_fr,
                    url_en: en_url,
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
