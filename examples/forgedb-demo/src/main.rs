//! Démo ForgeDB : `forge db init` → `forge migrate dev` → `cargo run -p forgedb-demo`

#[path = "../generated/forgedb/mod.rs"]
mod forgedb_client;

use anyhow::Result;
use forgedb_client::ForgeDbClient;
use forgedb_runtime::ForgeDb;

#[tokio::main]
async fn main() -> Result<()> {
    let _ = dotenvy::dotenv();
    let db = ForgeDb::connect_from_env().await?;
    let client = ForgeDbClient::new(db);

    let customers = client.customer.find_many().await?;
    println!("customers: {}", customers.len());

    let created = client
        .customer
        .create(forgedb_client::customer::CreateCustomer {
            email: "new@example.com".into(),
            name: "ForgeDB Demo".into(),
        })
        .await?;
    println!("created customer {}", created.id);

    Ok(())
}
