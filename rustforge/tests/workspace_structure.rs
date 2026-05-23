//! Vérifie la structure du workspace RustForge (crates et fichiers clés).

use std::path::{Path, PathBuf};

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("rustforge crate has a parent directory")
        .to_path_buf()
}

fn rustforge_dir() -> PathBuf {
    workspace_root().join("rustforge")
}

fn assert_dir_exists(path: impl AsRef<Path>) {
    let path = path.as_ref();
    assert!(path.is_dir(), "répertoire attendu: {}", path.display());
}

fn assert_file_exists(path: impl AsRef<Path>) {
    let path = path.as_ref();
    assert!(path.is_file(), "fichier attendu: {}", path.display());
}

/// Crates internes obligatoires du framework.
const EXPECTED_CRATES: &[&str] = &[
    "core",
    "macros",
    "di",
    "router",
    "http",
    "config",
    "validation",
    "logger",
    "testing",
    "security",
    "cli",
];

const FORGEDB_CRATES: &[&str] = &["schema", "parser", "codegen", "runtime", "migrate"];

/// Points d'entrée publics par crate (API stable / macros).
const KEY_FILES: &[(&str, &str)] = &[
    ("core", "src/module.rs"),
    ("core", "src/app.rs"),
    ("macros", "src/lib.rs"),
    ("macros", "src/module.rs"),
    ("macros", "src/controller.rs"),
    ("macros", "src/service.rs"),
    ("macros", "src/route.rs"),
    ("di", "src/container.rs"),
    ("di", "src/inject.rs"),
    ("http", "src/result.rs"),
    ("http", "src/server.rs"),
    ("router", "src/path.rs"),
    ("testing", "src/lib.rs"),
    ("root", "src/lib.rs"),
    ("root", "src/bootstrap.rs"),
];

#[test]
fn workspace_lists_all_internal_crates() {
    let rf = rustforge_dir();
    for name in EXPECTED_CRATES {
        let crate_dir = rf.join(name);
        assert_dir_exists(&crate_dir);
        assert_file_exists(crate_dir.join("Cargo.toml"));
        let lib_rs = crate_dir.join("src/lib.rs");
        let main_rs = crate_dir.join("src/main.rs");
        assert!(
            lib_rs.is_file() || main_rs.is_file(),
            "crate {} doit avoir src/lib.rs ou src/main.rs",
            name
        );
    }
}

#[test]
fn key_source_files_exist() {
    let rf = rustforge_dir();
    for (crate_name, rel) in KEY_FILES {
        let path = if *crate_name == "root" {
            rf.join(rel)
        } else {
            rf.join(crate_name).join(rel)
        };
        assert_file_exists(path);
    }
}

#[test]
fn forgedb_crates_exist() {
    let root = workspace_root();
    for name in FORGEDB_CRATES {
        let dir = root.join("forge-db").join(name);
        assert_dir_exists(&dir);
        assert_file_exists(dir.join("Cargo.toml"));
    }
}

#[test]
fn examples_exist() {
    let root = workspace_root();
    assert_dir_exists(root.join("examples/basic-api"));
    assert_dir_exists(root.join("examples/shop-api"));
    assert_file_exists(root.join("examples/basic-api/src/app_module.rs"));
    assert_file_exists(root.join("examples/shop-api/src/app_module.rs"));
    assert_file_exists(root.join("examples/forgedb-demo/schema.forge"));
}

#[test]
fn documentation_layout() {
    let root = workspace_root();
    assert_file_exists(root.join("README.md"));
    assert_file_exists(root.join("ARCHITECTURE.md"));
    assert_file_exists(root.join("docs/DX.md"));
}
