//! Démo RuestDB : `ruest db init` → `ruest migrate dev` → `cargo run -p ruest-db-demo`

#[path = "../generated/ruestdb/mod.rs"]
mod ruestdb_client;

use anyhow::Result;
use ruestdb_client::RuestDbClient;
use ruest_db_runtime::RuestDb;

#[tokio::main]
async fn main() -> Result<()> {
    let _ = dotenvy::dotenv();
    let db = RuestDb::connect_from_env().await?;
    let client = RuestDbClient::new(db);

    let customers = client.customer.find_many().await?;
    println!("customers: {}", customers.len());

    let created = client
        .customer
        .create(ruestdb_client::customer::CreateCustomer {
            email: "new@example.com".into(),
            name: "RuestDB Demo".into(),
        })
        .await?;
    println!("created customer {}", created.id);

    Ok(())
}
