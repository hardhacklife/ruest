use std::path::PathBuf;

fn main() {
    let root = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR"));
    println!("cargo:rerun-if-changed=schema.forge");

    if root.join("schema.forge").exists() {
        forgedb_migrate::generate_client(&root).expect("ForgeDB generate");
    }
}
