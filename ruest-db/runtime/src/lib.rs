//! Runtime RuestDB : pool PostgreSQL async (SQLx).

pub use chrono;
pub use serde;
pub use sqlx;
pub use sqlx::Row;
pub use uuid;

use std::time::Duration;

use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RuestDbError {
    #[error("database error: {0}")]
    Sqlx(#[from] sqlx::Error),

    #[error("configuration error: {0}")]
    Config(String),
}

/// Connexion partagée à PostgreSQL (prepared statements via SQLx).
#[derive(Clone)]
pub struct RuestDb {
    pool: PgPool,
}

impl RuestDb {
    /// `DATABASE_URL` depuis l'environnement (charge `.env` si présent).
    pub async fn connect_from_env() -> Result<Self, RuestDbError> {
        let _ = dotenvy::dotenv();
        let url = std::env::var("DATABASE_URL").map_err(|_| {
            RuestDbError::Config(
                "DATABASE_URL is not set (postgres://user:pass@localhost:5432/db)".into(),
            )
        })?;
        Self::connect(&url).await
    }

    pub async fn connect(url: &str) -> Result<Self, RuestDbError> {
        let pool = PgPoolOptions::new()
            .max_connections(10)
            .acquire_timeout(Duration::from_secs(30))
            .connect(url)
            .await?;
        Ok(Self { pool })
    }

    pub fn pool(&self) -> &PgPool {
        &self.pool
    }
}
