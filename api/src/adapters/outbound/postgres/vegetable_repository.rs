use async_trait::async_trait;
use deadpool_postgres::Pool;

use crate::application::ports::{vegetable_repository::VegetableRepository, Page, RepositoryError};
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
        v.good_companions,
        v.bad_companions,
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
    let good_companions: Vec<String> = row.try_get("good_companions").unwrap_or_default();
    let bad_companions: Vec<String> = row.try_get("bad_companions").unwrap_or_default();
    Ok(Vegetable {
        id: row.try_get("id")?,
        name: row.try_get("name")?,
        variety_ids,
        good_companions,
        bad_companions,
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

    async fn list_page(
        &self,
        locale: &str,
        page: usize,
        size: usize,
    ) -> Result<Page<Vegetable>, RepositoryError> {
        let client = self.pool.get().await?;
        let limit = size as i64;
        let offset = ((page - 1) * size) as i64;
        let query = format!(
            "SELECT COUNT(*) OVER() AS total_count, id, name,
                good_companions, bad_companions, variety_ids
             FROM (
                 {SELECT_COLUMNS} {GROUP_BY} ORDER BY v.id
             ) sub
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
            .map(row_to_vegetable)
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Page { items, total })
    }
}
