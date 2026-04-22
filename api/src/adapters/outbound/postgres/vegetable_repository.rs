use async_trait::async_trait;
use deadpool_postgres::Pool;

use crate::application::ports::{vegetable_repository::VegetableRepository, RepositoryError};
use crate::domain::models::vegetable::Vegetable;

pub struct PostgresVegetableRepository {
    pool: Pool,
}

impl PostgresVegetableRepository {
    pub fn new(pool: Pool) -> Self {
        Self { pool }
    }
}

const SELECT_COLUMNS: &str = r#"
    SELECT
        v.id,
        COALESCE(t_req.name, t_en.name) AS name,
        ARRAY_AGG(vr.id ORDER BY vr.id) FILTER (WHERE vr.id IS NOT NULL) AS variety_ids
    FROM vegetables v
    LEFT JOIN vegetable_translations t_req
           ON t_req.vegetable_id = v.id AND t_req.locale = $1
    LEFT JOIN vegetable_translations t_en
           ON t_en.vegetable_id = v.id AND t_en.locale = 'en'
    LEFT JOIN varieties vr
           ON vr.vegetable_id = v.id
"#;

const GROUP_BY: &str = "GROUP BY v.id, COALESCE(t_req.name, t_en.name)";

fn row_to_vegetable(row: &tokio_postgres::Row) -> Result<Vegetable, RepositoryError> {
    let variety_ids: Vec<String> = row.try_get("variety_ids").unwrap_or_default();
    Ok(Vegetable {
        id: row.try_get("id")?,
        name: row.try_get("name")?,
        variety_ids,
    })
}

#[async_trait]
impl VegetableRepository for PostgresVegetableRepository {
    async fn get_all(&self, locale: &str) -> Result<Vec<Vegetable>, RepositoryError> {
        let client = self.pool.get().await?;
        let query = format!("{SELECT_COLUMNS} {GROUP_BY} ORDER BY v.id");
        let rows = client.query(query.as_str(), &[&locale]).await?;
        rows.iter().map(row_to_vegetable).collect()
    }

    async fn get_by_id(
        &self,
        id: &str,
        locale: &str,
    ) -> Result<Option<Vegetable>, RepositoryError> {
        let client = self.pool.get().await?;
        let query = format!("{SELECT_COLUMNS} WHERE v.id = $2 {GROUP_BY}");
        let rows = client.query(query.as_str(), &[&locale, &id]).await?;
        rows.first().map(row_to_vegetable).transpose()
    }
}
