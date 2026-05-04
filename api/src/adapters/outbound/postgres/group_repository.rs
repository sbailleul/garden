use async_trait::async_trait;
use deadpool_postgres::Pool;

use crate::application::ports::{group_repository::GroupRepository, Page, RepositoryError};
use crate::domain::models::group::Group;

pub struct PostgresGroupRepository {
    pool: Pool,
}

impl PostgresGroupRepository {
    pub fn new(pool: Pool) -> Self {
        Self { pool }
    }
}

fn row_to_group(row: &tokio_postgres::Row) -> Result<Group, RepositoryError> {
    Ok(Group {
        id: row.try_get("id")?,
        name: row.try_get("name")?,
    })
}

#[async_trait]
impl GroupRepository for PostgresGroupRepository {
    async fn get_all(&self, locale: &str) -> Result<Vec<Group>, RepositoryError> {
        let client = self.pool.get().await?;
        let rows = client
            .query(
                "SELECT g.id, COALESCE(t_req.name, t_en.name) AS name
                 FROM groups g
                 LEFT JOIN group_translations t_req
                        ON t_req.group_id = g.id AND t_req.locale = $1
                 LEFT JOIN group_translations t_en
                        ON t_en.group_id = g.id AND t_en.locale = 'en'
                 ORDER BY g.id",
                &[&locale],
            )
            .await?;
        rows.iter().map(row_to_group).collect()
    }

    async fn get_by_id(&self, id: &str, locale: &str) -> Result<Option<Group>, RepositoryError> {
        let client = self.pool.get().await?;
        let rows = client
            .query(
                "SELECT g.id, COALESCE(t_req.name, t_en.name) AS name
                 FROM groups g
                 LEFT JOIN group_translations t_req
                        ON t_req.group_id = g.id AND t_req.locale = $1
                 LEFT JOIN group_translations t_en
                        ON t_en.group_id = g.id AND t_en.locale = 'en'
                 WHERE g.id = $2",
                &[&locale, &id],
            )
            .await?;
        rows.first().map(row_to_group).transpose()
    }

    async fn list_page(
        &self,
        locale: &str,
        page: usize,
        size: usize,
    ) -> Result<Page<Group>, RepositoryError> {
        let client = self.pool.get().await?;
        let limit = size as i64;
        let offset = ((page - 1) * size) as i64;
        let rows = client
            .query(
                "SELECT COUNT(*) OVER() AS total_count, id, name
                 FROM (
                     SELECT g.id, COALESCE(t_req.name, t_en.name) AS name
                     FROM groups g
                     LEFT JOIN group_translations t_req
                            ON t_req.group_id = g.id AND t_req.locale = $1
                     LEFT JOIN group_translations t_en
                            ON t_en.group_id = g.id AND t_en.locale = 'en'
                     ORDER BY g.id
                 ) sub
                 LIMIT $2 OFFSET $3",
                &[&locale, &limit, &offset],
            )
            .await?;
        let total = rows
            .first()
            .map(|r| r.try_get::<_, i64>("total_count").unwrap_or(0) as usize)
            .unwrap_or(0);
        let items = rows
            .iter()
            .map(row_to_group)
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Page { items, total })
    }
}
