use std::path::PathBuf;

fn main() {
    let root = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR"));
    println!("cargo:rerun-if-changed=schema.ruest");

    if root.join("schema.ruest").exists() {
        ruest_db_migrate::generate_client(&root).expect("RuestDB generate");
    }
}
