//! Généré par ForgeDB — ne pas modifier.

use forgedb_runtime::{ForgeDb, ForgeDbError};
use forgedb_runtime::serde::{Deserialize, Serialize};
use forgedb_runtime::Row;

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
    db: ForgeDb,
}

impl OrderDelegate {
    pub(crate) fn new(db: ForgeDb) -> Self {
        Self { db }
    }

    fn map_row(row: &forgedb_runtime::sqlx::postgres::PgRow) -> Result<Order, ForgeDbError> {
        Ok(Order {
            id: row.try_get::<String, _>("id")?,
            customerId: row.try_get::<String, _>("customerId")?,
            totalCents: row.try_get::<i32, _>("totalCents")?,
        })
    }

    pub async fn find_many(&self) -> Result<Vec<Order>, ForgeDbError> {
        let sql = "SELECT \"id\", \"customer_id\", \"total_cents\" FROM \"orders\" ORDER BY \"id\"";
        let rows = forgedb_runtime::sqlx::query(sql).fetch_all(self.db.pool()).await?;
        rows.iter().map(Self::map_row).collect()
    }

    pub async fn find_unique(&self, id: String) -> Result<Option<Order>, ForgeDbError> {
        let sql = "SELECT \"id\", \"customer_id\", \"total_cents\" FROM \"orders\" WHERE \"id\" = $1";
        let row = forgedb_runtime::sqlx::query(&sql)
            .bind(id)
            .fetch_optional(self.db.pool())
            .await?;
        row.as_ref().map(Self::map_row).transpose()
    }

    pub async fn create(&self, input: CreateOrder) -> Result<Order, ForgeDbError> {
        let sql = "INSERT INTO \"orders\" (\"customer_id\", \"total_cents\") VALUES ($1, $2) RETURNING \"id\", \"customer_id\", \"total_cents\"";
        let row = forgedb_runtime::sqlx::query(sql)
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
    ) -> Result<Option<Order>, ForgeDbError> {
        let existing = self.find_unique(id.clone()).await?;
        let Some(mut current) = existing else {
            return Ok(None);
        };
        if let Some(v) = input.customerId { current.customerId = v; }
        if let Some(v) = input.totalCents { current.totalCents = v; }

        let sql = "UPDATE \"orders\" SET \"customer_id\" = $2, \"total_cents\" = $3 WHERE \"id\" = $1 RETURNING \"id\", \"customer_id\", \"total_cents\"";
        let row = forgedb_runtime::sqlx::query(sql)
            .bind(id)
            .bind(current.customerId)
            .bind(current.totalCents)

            .fetch_optional(self.db.pool())
            .await?;
        row.as_ref().map(Self::map_row).transpose()
    }

    pub async fn delete(&self, id: String) -> Result<bool, ForgeDbError> {
        let sql = "DELETE FROM \"orders\" WHERE \"id\" = $1";
        let r = forgedb_runtime::sqlx::query(sql).bind(id).execute(self.db.pool()).await?;
        Ok(r.rows_affected() > 0)
    }
}
