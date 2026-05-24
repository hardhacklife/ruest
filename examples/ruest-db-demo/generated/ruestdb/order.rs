//! Généré par RuestDB — ne pas modifier.

use ruest_db_runtime::{RuestDb, RuestDbError};
use ruest_db_runtime::serde::{Deserialize, Serialize};
use ruest_db_runtime::Row;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Order {
    pub id: String,
    pub customerId: String,
    pub totalCents: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateOrder {
    pub customerId: String,
    pub totalCents: i32,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateOrder {
    pub customerId: Option<String>,
    pub totalCents: Option<i32>,
}

pub struct OrderDelegate {
    db: RuestDb,
}

impl OrderDelegate {
    pub(crate) fn new(db: RuestDb) -> Self {
        Self { db }
    }

    fn map_row(row: &ruest_db_runtime::sqlx::postgres::PgRow) -> Result<Order, RuestDbError> {
        Ok(Order {
            id: row.try_get::<String, _>("id")?,
            customerId: row.try_get::<String, _>("customerId")?,
            totalCents: row.try_get::<i32, _>("totalCents")?,
        })
    }

    pub async fn find_many(&self) -> Result<Vec<Order>, RuestDbError> {
        let sql = "SELECT \"id\", \"customer_id\", \"total_cents\" FROM \"orders\" ORDER BY \"id\"";
        let rows = ruest_db_runtime::sqlx::query(sql).fetch_all(self.db.pool()).await?;
        rows.iter().map(Self::map_row).collect()
    }

    pub async fn find_unique(&self, id: String) -> Result<Option<Order>, RuestDbError> {
        let sql = "SELECT \"id\", \"customer_id\", \"total_cents\" FROM \"orders\" WHERE \"id\" = $1";
        let row = ruest_db_runtime::sqlx::query(&sql)
            .bind(id)
            .fetch_optional(self.db.pool())
            .await?;
        row.as_ref().map(Self::map_row).transpose()
    }

    pub async fn create(&self, input: CreateOrder) -> Result<Order, RuestDbError> {
        let sql = "INSERT INTO \"orders\" (\"customer_id\", \"total_cents\") VALUES ($1, $2) RETURNING \"id\", \"customer_id\", \"total_cents\"";
        let row = ruest_db_runtime::sqlx::query(sql)
            .bind(input.customerId)
            .bind(input.totalCents)
            .fetch_one(self.db.pool())
            .await?;
        Self::map_row(&row)
    }

    pub async fn update(
        &self,
        id: String,
        input: UpdateOrder,
    ) -> Result<Option<Order>, RuestDbError> {
        let existing = self.find_unique(id.clone()).await?;
        let Some(mut current) = existing else {
            return Ok(None);
        };
        if let Some(v) = input.customerId { current.customerId = v; }
        if let Some(v) = input.totalCents { current.totalCents = v; }

        let sql = "UPDATE \"orders\" SET \"customer_id\" = $2, \"total_cents\" = $3 WHERE \"id\" = $1 RETURNING \"id\", \"customer_id\", \"total_cents\"";
        let row = ruest_db_runtime::sqlx::query(sql)
            .bind(id)
            .bind(current.customerId)
            .bind(current.totalCents)

            .fetch_optional(self.db.pool())
            .await?;
        row.as_ref().map(Self::map_row).transpose()
    }

    pub async fn delete(&self, id: String) -> Result<bool, RuestDbError> {
        let sql = "DELETE FROM \"orders\" WHERE \"id\" = $1";
        let r = ruest_db_runtime::sqlx::query(sql).bind(id).execute(self.db.pool()).await?;
        Ok(r.rows_affected() > 0)
    }
}
