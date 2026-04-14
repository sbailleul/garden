use async_trait::async_trait;
use deadpool_postgres::Pool;
use serde_json::Value as JsonValue;

use crate::application::ports::vegetable_repository::{RepositoryError, VegetableRepository};
use crate::domain::models::vegetable::{
    CalendarWindow, Category, Lifecycle, RegionCalendar, SoilType, SunExposure, Vegetable,
};

pub struct PostgresVegetableRepository {
    pool: Pool,
}

impl PostgresVegetableRepository {
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
    let calendars_json: JsonValue = row.try_get("calendars")?;
    let calendars: Vec<RegionCalendar> =
        serde_json::from_value::<Vec<RegionCalendar>>(calendars_json)
            .map_err(RepositoryError::Json)?;

    let soil_types_raw: Vec<String> = row.try_get("soil_types")?;
    let sun_requirement_raw: Vec<String> = row.try_get("sun_requirement")?;
    let good_companions: Vec<String> = row.try_get("good_companions")?;
    let bad_companions: Vec<String> = row.try_get("bad_companions")?;

    let category_str: String = row.try_get("category")?;
    let lifecycle_str: String = row.try_get("lifecycle")?;

    Ok(Vegetable {
        id: row.try_get("id")?,
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
        good_companions,
        bad_companions,
        calendars,
        variety_id: row.try_get("variety_id")?,
    })
}

// ---------------------------------------------------------------------------
// SQL helpers
// ---------------------------------------------------------------------------

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
        v.good_companions,
        v.bad_companions,
        v.calendars,
        v.variety_id
    FROM vegetables v
    LEFT JOIN vegetable_translations t_req
           ON t_req.vegetable_id = v.id AND t_req.locale = $1
    LEFT JOIN vegetable_translations t_en
           ON t_en.vegetable_id = v.id AND t_en.locale = 'en'
"#;

// ---------------------------------------------------------------------------
// VegetableRepository implementation
// ---------------------------------------------------------------------------

#[async_trait]
impl VegetableRepository for PostgresVegetableRepository {
    async fn get_all(&self, locale: &str) -> Result<Vec<Vegetable>, RepositoryError> {
        let client = self.pool.get().await?;
        let query = format!("{SELECT_COLUMNS} ORDER BY v.id");
        let rows = client.query(query.as_str(), &[&locale]).await?;
        rows.iter().map(row_to_vegetable).collect()
    }

    async fn get_by_id(
        &self,
        id: &str,
        locale: &str,
    ) -> Result<Option<Vegetable>, RepositoryError> {
        let client = self.pool.get().await?;
        let query = format!("{SELECT_COLUMNS} WHERE v.id = $2");
        let rows = client.query(query.as_str(), &[&locale, &id]).await?;
        rows.first().map(row_to_vegetable).transpose()
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
