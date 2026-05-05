use async_trait::async_trait;
use deadpool_postgres::Pool;
use serde_json::Value as JsonValue;

use crate::application::ports::{
    variety_response_repository::{VarietyListFilter, VarietyResponse, VarietyResponseRepository},
    Page, RepositoryError,
};
use crate::domain::models::variety::{
    CalendarWindow, Category, Lifecycle, RegionCalendar, SoilType, SunExposure,
};

pub struct PostgresVarietyResponseRepository {
    pool: Pool,
}

impl PostgresVarietyResponseRepository {
    pub fn new(pool: Pool) -> Self {
        Self { pool }
    }
}

// ---------------------------------------------------------------------------
// Row-to-DTO mapping helpers
// ---------------------------------------------------------------------------

fn parse_enum<T: for<'de> serde::Deserialize<'de>>(s: &str) -> Result<T, RepositoryError> {
    serde_json::from_str(&format!(r#""{s}""#)).map_err(RepositoryError::Json)
}

fn parse_enum_vec<T: for<'de> serde::Deserialize<'de>>(
    arr: &[String],
) -> Result<Vec<T>, RepositoryError> {
    arr.iter().map(|s| parse_enum(s.as_str())).collect()
}

fn row_to_variety_response(row: &tokio_postgres::Row) -> Result<VarietyResponse, RepositoryError> {
    let calendars_json: JsonValue = row.try_get("calendars")?;
    let calendars: Vec<RegionCalendar> =
        serde_json::from_value::<Vec<RegionCalendar>>(calendars_json)
            .map_err(RepositoryError::Json)?;

    let soil_types_raw: Vec<String> = row.try_get("soil_types")?;
    let sun_requirement_raw: Vec<String> = row.try_get("sun_requirement")?;
    let category_str: String = row.try_get("category")?;
    let lifecycle_str: String = row.try_get("lifecycle")?;

    Ok(VarietyResponse {
        id: row.try_get("id")?,
        vegetable_id: row.try_get("vegetable_id")?,
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

// ---------------------------------------------------------------------------
// SQL helpers
// ---------------------------------------------------------------------------

/// Simpler SELECT: no JOIN to the vegetables table — we only need `vegetable_id`
/// as a plain string column, so the query is lighter than the full domain query.
const SELECT_COLUMNS: &str = r#"
    SELECT
        v.id,
        v.vegetable_id,
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
        v.calendars
    FROM varieties v
    LEFT JOIN variety_translations t_req
           ON t_req.variety_id = v.id AND t_req.locale = $1
    LEFT JOIN variety_translations t_en
           ON t_en.variety_id = v.id AND t_en.locale = 'en'
"#;

/// Appends WHERE conditions derived from `filter` to `clauses`.
///
/// `param_idx` is the 1-based index of the **next** positional parameter
/// (`$1` is already taken by `locale`). Returns the updated index and a
/// `Vec` of boxed `ToSql` values to append to the query parameters.
fn build_filter_clauses(
    filter: &VarietyListFilter,
    param_idx: usize,
) -> (
    Vec<String>,
    Vec<Box<dyn tokio_postgres::types::ToSql + Send + Sync>>,
    usize,
) {
    let mut clauses: Vec<String> = Vec::new();
    let mut values: Vec<Box<dyn tokio_postgres::types::ToSql + Send + Sync>> = Vec::new();
    let mut idx = param_idx;

    if let Some(ref cat) = filter.category {
        clauses.push(format!("v.category = ${idx}"));
        values.push(Box::new(
            serde_json::to_value(cat)
                .unwrap()
                .as_str()
                .unwrap()
                .to_string(),
        ));
        idx += 1;
    }
    if let Some(ref lc) = filter.lifecycle {
        clauses.push(format!("v.lifecycle = ${idx}"));
        values.push(Box::new(
            serde_json::to_value(lc)
                .unwrap()
                .as_str()
                .unwrap()
                .to_string(),
        ));
        idx += 1;
    }
    if let Some(bf) = filter.beginner_friendly {
        clauses.push(format!("v.beginner_friendly = ${idx}"));
        values.push(Box::new(bf));
        idx += 1;
    }
    if let Some(ref sun) = filter.sun_requirement {
        clauses.push(format!("${idx} = ANY(v.sun_requirement)"));
        values.push(Box::new(
            serde_json::to_value(sun)
                .unwrap()
                .as_str()
                .unwrap()
                .to_string(),
        ));
        idx += 1;
    }
    if let Some(ref soil) = filter.soil_type {
        clauses.push(format!("${idx} = ANY(v.soil_types)"));
        values.push(Box::new(
            serde_json::to_value(soil)
                .unwrap()
                .as_str()
                .unwrap()
                .to_string(),
        ));
        idx += 1;
    }
    if let Some(ref region) = filter.region {
        let region_str = serde_json::to_value(region)
            .unwrap()
            .as_str()
            .unwrap()
            .to_string();
        clauses.push(format!(
            "v.calendars @> jsonb_build_array(jsonb_build_object('region', ${idx}::text))"
        ));
        values.push(Box::new(region_str));
        idx += 1;
    }

    (clauses, values, idx)
}

// ---------------------------------------------------------------------------
// VarietyResponseRepository implementation
// ---------------------------------------------------------------------------

#[async_trait]
impl VarietyResponseRepository for PostgresVarietyResponseRepository {
    async fn get_by_id(
        &self,
        id: &str,
        locale: &str,
    ) -> Result<Option<VarietyResponse>, RepositoryError> {
        let client = self.pool.get().await?;
        let query = format!("{SELECT_COLUMNS} WHERE v.id = $2");
        let rows = client.query(query.as_str(), &[&locale, &id]).await?;
        rows.first().map(row_to_variety_response).transpose()
    }

    async fn list_page(
        &self,
        locale: &str,
        page: usize,
        size: usize,
        filter: &VarietyListFilter,
    ) -> Result<Page<VarietyResponse>, RepositoryError> {
        let client = self.pool.get().await?;

        // $1 = locale; filter params start at $2
        let (clauses, filter_values, next_idx) = build_filter_clauses(filter, 2);

        // vegetable_id filter (optional)
        let (veg_clause, veg_value, next_idx) = if let Some(ref vid) = filter.vegetable_id {
            (
                Some(format!("v.vegetable_id = ${next_idx}")),
                Some(vid.clone()),
                next_idx + 1,
            )
        } else {
            (None, None, next_idx)
        };

        let limit_idx = next_idx;
        let offset_idx = next_idx + 1;

        let all_clauses: Vec<String> = clauses.into_iter().chain(veg_clause).collect();

        let where_sql = if all_clauses.is_empty() {
            String::new()
        } else {
            format!("WHERE {}", all_clauses.join(" AND "))
        };

        let limit = size as i64;
        let offset = ((page - 1) * size) as i64;
        let query = format!(
            "SELECT COUNT(*) OVER() AS total_count, sub.*
             FROM ({SELECT_COLUMNS} {where_sql} ORDER BY v.id) sub
             LIMIT ${limit_idx} OFFSET ${offset_idx}"
        );

        let mut params: Vec<Box<dyn tokio_postgres::types::ToSql + Send + Sync>> =
            vec![Box::new(locale.to_string())];
        for v in filter_values {
            params.push(v);
        }
        if let Some(v) = veg_value {
            params.push(Box::new(v));
        }
        params.push(Box::new(limit));
        params.push(Box::new(offset));

        let param_refs: Vec<&(dyn tokio_postgres::types::ToSql + Sync)> = params
            .iter()
            .map(|p| p.as_ref() as &(dyn tokio_postgres::types::ToSql + Sync))
            .collect();

        let rows = client.query(query.as_str(), &param_refs).await?;
        let total = rows
            .first()
            .map(|r| r.try_get::<_, i64>("total_count").unwrap_or(0) as usize)
            .unwrap_or(0);
        let items = rows
            .iter()
            .map(row_to_variety_response)
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Page { items, total })
    }

    async fn list_page_by_vegetable_id(
        &self,
        vegetable_id: &str,
        locale: &str,
        page: usize,
        size: usize,
        filter: &VarietyListFilter,
    ) -> Result<Page<VarietyResponse>, RepositoryError> {
        let client = self.pool.get().await?;

        // $1 = locale, $2 = vegetable_id; filter params start at $3
        let (clauses, filter_values, next_idx) = build_filter_clauses(filter, 3);

        let limit_idx = next_idx;
        let offset_idx = next_idx + 1;

        let where_sql = if clauses.is_empty() {
            "WHERE v.vegetable_id = $2".to_string()
        } else {
            format!("WHERE v.vegetable_id = $2 AND {}", clauses.join(" AND "))
        };

        let limit = size as i64;
        let offset = ((page - 1) * size) as i64;
        let query = format!(
            "SELECT COUNT(*) OVER() AS total_count, sub.*
             FROM ({SELECT_COLUMNS} {where_sql} ORDER BY v.id) sub
             LIMIT ${limit_idx} OFFSET ${offset_idx}"
        );

        let mut params: Vec<Box<dyn tokio_postgres::types::ToSql + Send + Sync>> = vec![
            Box::new(locale.to_string()),
            Box::new(vegetable_id.to_string()),
        ];
        for v in filter_values {
            params.push(v);
        }
        params.push(Box::new(limit));
        params.push(Box::new(offset));

        let param_refs: Vec<&(dyn tokio_postgres::types::ToSql + Sync)> = params
            .iter()
            .map(|p| p.as_ref() as &(dyn tokio_postgres::types::ToSql + Sync))
            .collect();

        let rows = client.query(query.as_str(), &param_refs).await?;
        let total = rows
            .first()
            .map(|r| r.try_get::<_, i64>("total_count").unwrap_or(0) as usize)
            .unwrap_or(0);
        let items = rows
            .iter()
            .map(row_to_variety_response)
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Page { items, total })
    }
}

// ---------------------------------------------------------------------------
// Deserialise CalendarWindow / RegionCalendar from the JSONB representation.
// ---------------------------------------------------------------------------
const _: fn() = || {
    fn _assert_deserialize<T: for<'de> serde::Deserialize<'de>>() {}
    _assert_deserialize::<CalendarWindow>();
    _assert_deserialize::<RegionCalendar>();
};
