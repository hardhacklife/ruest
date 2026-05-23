//! Généré par ForgeDB — ne pas modifier.

use forgedb_runtime::{ForgeDb, ForgeDbError};
use forgedb_runtime::serde::{Deserialize, Serialize};
use forgedb_runtime::Row;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Customer {
    pub id: String,
    pub email: String,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCustomer {
    pub email: String,
    pub name: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateCustomer {
    pub email: Option<String>,
    pub name: Option<String>,
}

pub struct CustomerDelegate {
    db: ForgeDb,
}

impl CustomerDelegate {
    pub(crate) fn new(db: ForgeDb) -> Self {
        Self { db }
    }

    fn map_row(row: &forgedb_runtime::sqlx::postgres::PgRow) -> Result<Customer, ForgeDbError> {
        Ok(Customer {
            id: row.try_get::<String, _>("id")?,
            email: row.try_get::<String, _>("email")?,
            name: row.try_get::<String, _>("name")?,
        })
    }

    pub async fn find_many(&self) -> Result<Vec<Customer>, ForgeDbError> {
        let sql = "SELECT \"id\", \"email\", \"name\" FROM \"customers\" ORDER BY \"id\"";
        let rows = forgedb_runtime::sqlx::query(sql).fetch_all(self.db.pool()).await?;
        rows.iter().map(Self::map_row).collect()
    }

    pub async fn find_unique(&self, id: String) -> Result<Option<Customer>, ForgeDbError> {
        let sql = "SELECT \"id\", \"email\", \"name\" FROM \"customers\" WHERE \"id\" = $1";
        let row = forgedb_runtime::sqlx::query(&sql)
            .bind(id)
            .fetch_optional(self.db.pool())
            .await?;
        row.as_ref().map(Self::map_row).transpose()
    }

    pub async fn create(&self, input: CreateCustomer) -> Result<Customer, ForgeDbError> {
        let sql = "INSERT INTO \"customers\" (\"email\", \"name\") VALUES ($1, $2) RETURNING \"id\", \"email\", \"name\"";
        let row = forgedb_runtime::sqlx::query(sql)
            .bind(input.email)
            .bind(input.name)
            .fetch_one(self.db.pool())
            .await?;
        Self::map_row(&row)
    }

    pub async fn update(
        &self,
        id: String,
        input: UpdateCustomer,
    ) -> Result<Option<Customer>, ForgeDbError> {
        let existing = self.find_unique(id.clone()).await?;
        let Some(mut current) = existing else {
            return Ok(None);
        };
        if let Some(v) = input.email { current.email = v; }
        if let Some(v) = input.name { current.name = v; }

        let sql = "UPDATE \"customers\" SET \"email\" = $2, \"name\" = $3 WHERE \"id\" = $1 RETURNING \"id\", \"email\", \"name\"";
        let row = forgedb_runtime::sqlx::query(sql)
            .bind(id)
            .bind(current.email)
            .bind(current.name)

            .fetch_optional(self.db.pool())
            .await?;
        row.as_ref().map(Self::map_row).transpose()
    }

    pub async fn delete(&self, id: String) -> Result<bool, ForgeDbError> {
        let sql = "DELETE FROM \"customers\" WHERE \"id\" = $1";
        let r = forgedb_runtime::sqlx::query(sql).bind(id).execute(self.db.pool()).await?;
        Ok(r.rows_affected() > 0)
    }
}
