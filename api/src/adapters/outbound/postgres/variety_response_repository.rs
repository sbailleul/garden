use async_trait::async_trait;
use deadpool_postgres::Pool;
use serde_json::Value as JsonValue;

use crate::application::ports::{
    variety_response_repository::{VarietyResponse, VarietyResponseRepository},
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
    ) -> Result<Page<VarietyResponse>, RepositoryError> {
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
    ) -> Result<Page<VarietyResponse>, RepositoryError> {
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
