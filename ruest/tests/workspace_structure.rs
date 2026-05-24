//! Vérifie la structure du workspace RUEST (crates et fichiers clés).

use std::path::{Path, PathBuf};

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("ruest crate has a parent directory")
        .to_path_buf()
}

fn ruest_dir() -> PathBuf {
    workspace_root().join("ruest")
}

fn assert_dir_exists(path: impl AsRef<Path>) {
    let path = path.as_ref();
    assert!(path.is_dir(), "répertoire attendu: {}", path.display());
}

fn assert_file_exists(path: impl AsRef<Path>) {
    let path = path.as_ref();
    assert!(path.is_file(), "fichier attendu: {}", path.display());
}

/// Modules internes du crate `ruest` (plus de crates séparés sur crates.io).
const RUEST_INTERNAL_MODULES: &[&str] = &[
    "core",
    "di",
    "router",
    "http",
    "config",
    "validation",
    "logger",
    "security",
    "testing",
];

const PUBLISHED_CRATES: &[(&str, &str)] = &[
    ("ruest", "ruest/Cargo.toml"),
    ("ruest-macros", "ruest/macros/Cargo.toml"),
    ("ruest-db", "ruest-db/Cargo.toml"),
    ("ruest-cli", "ruest/cli/Cargo.toml"),
];

/// Points d'entrée publics (API stable / macros).
const KEY_FILES: &[(&str, &str)] = &[
    ("core", "src/core/module.rs"),
    ("core", "src/core/app.rs"),
    ("macros", "macros/src/lib.rs"),
    ("macros", "macros/src/module.rs"),
    ("di", "src/di/container.rs"),
    ("di", "src/di/inject.rs"),
    ("http", "src/http/result.rs"),
    ("http", "src/http/server.rs"),
    ("router", "src/router/path.rs"),
    ("testing", "src/testing/mod.rs"),
    ("root", "src/lib.rs"),
    ("root", "src/bootstrap.rs"),
];

#[test]
fn ruest_internal_modules_exist() {
    let rf = ruest_dir();
    for name in RUEST_INTERNAL_MODULES {
        assert_dir_exists(rf.join("src").join(name));
        assert_file_exists(rf.join("src").join(name).join("mod.rs"));
    }
}

#[test]
fn published_crates_exist() {
    let root = workspace_root();
    for (_name, rel) in PUBLISHED_CRATES {
        assert_file_exists(root.join(rel));
    }
}

#[test]
fn key_source_files_exist() {
    let rf = ruest_dir();
    for (area, rel) in KEY_FILES {
        let path = if *area == "macros" {
            rf.join(rel)
        } else {
            rf.join(rel)
        };
        assert_file_exists(path);
    }
}

#[test]
fn ruestdb_crate_exists() {
    let root = workspace_root();
    assert_dir_exists(root.join("ruest-db"));
    assert_file_exists(root.join("ruest-db/Cargo.toml"));
    assert_file_exists(root.join("ruest-db/src/lib.rs"));
}

#[test]
fn examples_exist() {
    let root = workspace_root();
    assert_dir_exists(root.join("examples/basic-api"));
    assert_dir_exists(root.join("examples/shop-api"));
    assert_file_exists(root.join("examples/basic-api/src/app_module.rs"));
    assert_file_exists(root.join("examples/shop-api/src/app_module.rs"));
    assert_file_exists(root.join("examples/ruest-db-demo/schema.ruest"));
}

#[test]
fn documentation_layout() {
    let root = workspace_root();
    assert_file_exists(root.join("README.md"));
    assert_file_exists(root.join("ARCHITECTURE.md"));
    assert_file_exists(root.join("docs/DX.md"));
}
