use async_trait::async_trait;
use deadpool_postgres::Pool;

use crate::application::ports::variety_repository::{VarietyRepository, VarietyRepositoryError};
use crate::domain::models::variety::Variety;

pub struct PostgresVarietyRepository {
    pool: Pool,
}

impl PostgresVarietyRepository {
    pub fn new(pool: Pool) -> Self {
        Self { pool }
    }
}

const SELECT_COLUMNS: &str = r#"
    SELECT
        v.id,
        COALESCE(t_req.name, t_en.name) AS name
    FROM varieties v
    LEFT JOIN variety_translations t_req
           ON t_req.variety_id = v.id AND t_req.locale = $1
    LEFT JOIN variety_translations t_en
           ON t_en.variety_id = v.id AND t_en.locale = 'en'
"#;

fn row_to_variety(row: &tokio_postgres::Row) -> Result<Variety, VarietyRepositoryError> {
    Ok(Variety {
        id: row.try_get("id")?,
        name: row.try_get("name")?,
    })
}

#[async_trait]
impl VarietyRepository for PostgresVarietyRepository {
    async fn get_all(&self, locale: &str) -> Result<Vec<Variety>, VarietyRepositoryError> {
        let client = self.pool.get().await?;
        let query = format!("{SELECT_COLUMNS} ORDER BY v.id");
        let rows = client.query(query.as_str(), &[&locale]).await?;
        rows.iter().map(row_to_variety).collect()
    }

    async fn get_by_id(
        &self,
        id: &str,
        locale: &str,
    ) -> Result<Option<Variety>, VarietyRepositoryError> {
        let client = self.pool.get().await?;
        let query = format!("{SELECT_COLUMNS} WHERE v.id = $2");
        let rows = client.query(query.as_str(), &[&locale, &id]).await?;
        rows.first().map(row_to_variety).transpose()
    }
}
