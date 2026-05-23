//! Client ForgeDB généré — `client.user.find_many().await?`

pub mod customer;
pub mod order;

use forgedb_runtime::ForgeDb;

pub struct ForgeDbClient {
    inner: ForgeDb,
    pub customer: customer::CustomerDelegate,
    pub order: order::OrderDelegate,
}

impl ForgeDbClient {
    pub fn new(db: ForgeDb) -> Self {
        Self {
            inner: db.clone(),
            customer: customer::CustomerDelegate::new(db.clone()),
            order: order::OrderDelegate::new(db.clone()),
        }
    }

    pub fn db(&self) -> &ForgeDb {
        &self.inner
    }
}
