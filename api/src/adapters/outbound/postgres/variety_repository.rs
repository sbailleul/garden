use async_trait::async_trait;
use deadpool_postgres::Pool;
use serde_json::Value as JsonValue;
use std::collections::HashMap;
use std::sync::Arc;
use tokio_postgres::types::ToSql;

use crate::application::ports::{
    variety_repository::{VarietyFilter, VarietyRepository},
    Page, RepositoryError,
};
use crate::domain::models::variety::{
    CalendarWindow, Category, Lifecycle, RegionCalendar, SoilType, SunExposure, Variety,
};
use crate::domain::models::vegetable::Vegetable;

pub struct PostgresVarietyRepository {
    pool: Pool,
}

impl PostgresVarietyRepository {
    pub fn new(pool: Pool) -> Self {
        Self { pool }
    }
}

// ---------------------------------------------------------------------------
// Row-to-domain mapping helpers
// ---------------------------------------------------------------------------

fn parse_enum<T: for<'de> serde::Deserialize<'de>>(s: &str) -> Result<T, RepositoryError> {
    serde_json::from_str(&format!(r#""{s}""#)).map_err(RepositoryError::Json)
}

fn parse_enum_vec<T: for<'de> serde::Deserialize<'de>>(
    arr: &[String],
) -> Result<Vec<T>, RepositoryError> {
    arr.iter().map(|s| parse_enum(s.as_str())).collect()
}

fn row_to_vegetable(row: &tokio_postgres::Row) -> Result<Vegetable, RepositoryError> {
    let veg_id: String = row.try_get("veg_id")?;
    let veg_name: String = row.try_get("veg_name").unwrap_or_default();
    let veg_good_companions: Vec<String> = row.try_get("veg_good_companions").unwrap_or_default();
    let veg_bad_companions: Vec<String> = row.try_get("veg_bad_companions").unwrap_or_default();
    let veg_variety_ids: Vec<String> = row.try_get("veg_variety_ids").unwrap_or_default();
    Ok(Vegetable {
        id: veg_id,
        name: veg_name,
        variety_ids: veg_variety_ids,
        good_companions: veg_good_companions,
        bad_companions: veg_bad_companions,
    })
}

fn row_to_variety(
    row: &tokio_postgres::Row,
    vegetable: Arc<Vegetable>,
) -> Result<Variety, RepositoryError> {
    let calendars_json: JsonValue = row.try_get("calendars")?;
    let calendars: Vec<RegionCalendar> =
        serde_json::from_value::<Vec<RegionCalendar>>(calendars_json)
            .map_err(RepositoryError::Json)?;

    let soil_types_raw: Vec<String> = row.try_get("soil_types")?;
    let sun_requirement_raw: Vec<String> = row.try_get("sun_requirement")?;
    let category_str: String = row.try_get("category")?;
    let lifecycle_str: String = row.try_get("lifecycle")?;

    Ok(Variety {
        id: row.try_get("id")?,
        vegetable,
        name: row.try_get("name")?,
        latin_name: row.try_get("latin_name")?,
        category: parse_enum::<Category>(&category_str)?,
        lifecycle: parse_enum::<Lifecycle>(&lifecycle_str)?,
        spacing_cm: row.try_get::<_, i32>("spacing_cm")? as u32,
        days_to_harvest: row.try_get::<_, i32>("days_to_harvest")? as u32,
        days_to_plant: row.try_get::<_, i32>("days_to_plant")? as u32,
        beginner_friendly: row.try_get("beginner_friendly")?,
        soil_types: parse_enum_vec::<SoilType>(&soil_types_raw)?,
        sun_requirement: parse_enum_vec::<SunExposure>(&sun_requirement_raw)?,
        calendars,
    })
}

/// Maps a slice of rows to `Variety` values, sharing one `Arc<Vegetable>` per
/// unique vegetable id so that varieties belonging to the same vegetable point
/// to the same heap allocation.
fn rows_to_varieties(rows: &[tokio_postgres::Row]) -> Result<Vec<Variety>, RepositoryError> {
    let mut veg_cache: HashMap<String, Arc<Vegetable>> = HashMap::new();
    let mut items = Vec::with_capacity(rows.len());
    for row in rows {
        let veg_id: String = row.try_get("veg_id")?;
        let vegetable = match veg_cache.get(&veg_id) {
            Some(v) => Arc::clone(v),
            None => {
                let v = Arc::new(row_to_vegetable(row)?);
                veg_cache.insert(veg_id, Arc::clone(&v));
                v
            }
        };
        items.push(row_to_variety(row, vegetable)?);
    }
    Ok(items)
}

// ---------------------------------------------------------------------------
// SQL helpers
// ---------------------------------------------------------------------------

/// Serialises a serde value to its JSON string representation without the
/// surrounding double-quotes (e.g. `Region::Temperate` → `"Temperate"`).
fn serde_json_str<T: serde::Serialize>(val: &T) -> String {
    serde_json::to_string(val)
        .unwrap_or_default()
        .trim_matches('"')
        .to_string()
}

/// Builds the WHERE clause and the ordered parameter list for a
/// [`VarietyFilter`] query.
///
/// `$1` is always the locale (passed in `SELECT_COLUMNS`).
/// Additional parameters start at `$2` and are returned as `Vec<String>`,
/// allowing `tokio_postgres` to bind them uniformly as text.
fn build_filter_query(locale: &str, filter: &VarietyFilter) -> (String, Vec<String>) {
    let mut conditions: Vec<String> = Vec::new();
    let mut params: Vec<String> = vec![locale.to_string()]; // $1 = locale
    let mut idx: usize = 2;

    // Region is always required — check the JSONB calendars array for an
    // entry whose "region" key matches.
    params.push(serde_json_str(&filter.region));
    conditions.push(format!(
        "v.calendars @> jsonb_build_array(jsonb_build_object('region', ${}::text))",
        idx
    ));
    idx += 1;

    // Sun exposure — stored as a text[] column.
    if let Some(ref sun) = filter.sun {
        params.push(serde_json_str(sun));
        conditions.push(format!("${}::text = ANY(v.sun_requirement)", idx));
        idx += 1;
    }

    // Soil type — stored as a text[] column.
    if let Some(ref soil) = filter.soil {
        params.push(serde_json_str(soil));
        conditions.push(format!("${}::text = ANY(v.soil_types)", idx));
        idx += 1;
    }

    // Skill level — plain boolean column, no extra parameter needed.
    if filter.beginner_only {
        conditions.push("v.beginner_friendly = true".to_string());
    }

    // Excluded variety IDs — each id becomes an individual parameter so that
    // the full params vector stays homogeneously typed as String.
    if !filter.exclusions.is_empty() {
        let placeholders: Vec<String> = (0..filter.exclusions.len())
            .map(|i| format!("${}", idx + i))
            .collect();
        conditions.push(format!("v.id NOT IN ({})", placeholders.join(", ")));
        params.extend(filter.exclusions.iter().cloned());
    }

    let where_clause = format!("WHERE {}", conditions.join(" AND "));
    let query = format!("{SELECT_COLUMNS} {where_clause} ORDER BY v.id");
    (query, params)
}

const SELECT_COLUMNS: &str = r#"
    SELECT
        v.id,
        COALESCE(t_req.name, t_en.name) AS name,
        v.latin_name,
        v.category,
        v.lifecycle,
        v.spacing_cm,
        v.days_to_harvest,
        v.days_to_plant,
        v.beginner_friendly,
        v.soil_types,
        v.sun_requirement,
        v.calendars,
        veg.id                                                                     AS veg_id,
        COALESCE(vt_req.name, vt_en.name)                                         AS veg_name,
        veg.good_companions                                                        AS veg_good_companions,
        veg.bad_companions                                                         AS veg_bad_companions,
        (SELECT ARRAY_AGG(v2.id ORDER BY v2.id) FROM varieties v2
          WHERE v2.vegetable_id = veg.id)                                          AS veg_variety_ids
    FROM varieties v
    JOIN vegetables veg
           ON veg.id = v.vegetable_id
    LEFT JOIN variety_translations t_req
           ON t_req.variety_id = v.id AND t_req.locale = $1
    LEFT JOIN variety_translations t_en
           ON t_en.variety_id = v.id AND t_en.locale = 'en'
    LEFT JOIN vegetable_translations vt_req
           ON vt_req.vegetable_id = veg.id AND vt_req.locale = $1
    LEFT JOIN vegetable_translations vt_en
           ON vt_en.vegetable_id = veg.id AND vt_en.locale = 'en'
"#;

// ---------------------------------------------------------------------------
// VarietyRepository implementation
// ---------------------------------------------------------------------------

#[async_trait]
impl VarietyRepository for PostgresVarietyRepository {
    async fn get_all(&self, locale: &str) -> Result<Vec<Variety>, RepositoryError> {
        let client = self.pool.get().await?;
        let query = format!("{SELECT_COLUMNS} ORDER BY v.id");
        let rows = client.query(query.as_str(), &[&locale]).await?;
        rows_to_varieties(&rows)
    }

    async fn get_by_id(&self, id: &str, locale: &str) -> Result<Option<Variety>, RepositoryError> {
        let client = self.pool.get().await?;
        let query = format!("{SELECT_COLUMNS} WHERE v.id = $2");
        let rows = client.query(query.as_str(), &[&locale, &id]).await?;
        rows.first()
            .map(|row| {
                let vegetable = Arc::new(row_to_vegetable(row)?);
                row_to_variety(row, vegetable)
            })
            .transpose()
    }

    async fn get_by_vegetable_id(
        &self,
        vegetable_id: &str,
        locale: &str,
    ) -> Result<Vec<Variety>, RepositoryError> {
        let client = self.pool.get().await?;
        let query = format!("{SELECT_COLUMNS} WHERE v.vegetable_id = $2 ORDER BY v.id");
        let rows = client
            .query(query.as_str(), &[&locale, &vegetable_id])
            .await?;
        rows_to_varieties(&rows)
    }

    async fn list_page(
        &self,
        locale: &str,
        page: usize,
        size: usize,
    ) -> Result<Page<Variety>, RepositoryError> {
        let client = self.pool.get().await?;
        let limit = size as i64;
        let offset = ((page - 1) * size) as i64;
        let query = format!(
            "SELECT COUNT(*) OVER() AS total_count, sub.*
             FROM ({SELECT_COLUMNS} ORDER BY v.id) sub
             LIMIT $2 OFFSET $3"
        );
        let rows = client
            .query(query.as_str(), &[&locale, &limit, &offset])
            .await?;
        let total = rows
            .first()
            .map(|r| r.try_get::<_, i64>("total_count").unwrap_or(0) as usize)
            .unwrap_or(0);
        let items = rows_to_varieties(&rows)?;
        Ok(Page { items, total })
    }

    async fn list_page_by_vegetable_id(
        &self,
        vegetable_id: &str,
        locale: &str,
        page: usize,
        size: usize,
    ) -> Result<Page<Variety>, RepositoryError> {
        let client = self.pool.get().await?;
        let limit = size as i64;
        let offset = ((page - 1) * size) as i64;
        let query = format!(
            "SELECT COUNT(*) OVER() AS total_count, sub.*
             FROM ({SELECT_COLUMNS} WHERE v.vegetable_id = $2 ORDER BY v.id) sub
             LIMIT $3 OFFSET $4"
        );
        let rows = client
            .query(query.as_str(), &[&locale, &vegetable_id, &limit, &offset])
            .await?;
        let total = rows
            .first()
            .map(|r| r.try_get::<_, i64>("total_count").unwrap_or(0) as usize)
            .unwrap_or(0);
        let items = rows_to_varieties(&rows)?;
        Ok(Page { items, total })
    }

    async fn get_for_planning(
        &self,
        filter: &VarietyFilter,
        locale: &str,
    ) -> Result<Vec<Variety>, RepositoryError> {
        let client = self.pool.get().await?;
        let (query, params) = build_filter_query(locale, filter);
        // All params are strings — collect borrows into a uniform slice.
        let params_refs: Vec<&(dyn ToSql + Sync)> =
            params.iter().map(|s| s as &(dyn ToSql + Sync)).collect();
        let rows = client.query(query.as_str(), &params_refs).await?;
        rows_to_varieties(&rows)
    }
}

// ---------------------------------------------------------------------------
// Deserialise CalendarWindow / RegionCalendar from the JSONB representation
// stored by this crate's serde attributes.  The types already implement
// Deserialize (serde rename_all = "camelCase" / "PascalCase") so we just
// delegate to serde_json.
// ---------------------------------------------------------------------------
const _: fn() = || {
    fn _assert_deserialize<T: for<'de> serde::Deserialize<'de>>() {}
    _assert_deserialize::<CalendarWindow>();
    _assert_deserialize::<RegionCalendar>();
};
