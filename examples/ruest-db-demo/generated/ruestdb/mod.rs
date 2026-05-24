//! Client RuestDB généré — `client.user.find_many().await?`

pub mod customer;
pub mod order;

use ruest_db::RuestDb;

pub struct RuestDbClient {
    inner: RuestDb,
    pub customer: customer::CustomerDelegate,
    pub order: order::OrderDelegate,
}

impl RuestDbClient {
    pub fn new(db: RuestDb) -> Self {
        Self {
            inner: db.clone(),
            customer: customer::CustomerDelegate::new(db.clone()),
            order: order::OrderDelegate::new(db.clone()),
        }
    }

    pub fn db(&self) -> &RuestDb {
        &self.inner
    }
}
