//! Tests du graphe de modules (imports → providers enfants).

use ruest::core::{bootstrap, Module, ModuleMetadata};
use ruest::di::Container;

#[derive(Default, Debug, PartialEq)]
struct LeafService {
    marker: u32,
}

fn register_leaf(container: &Container) {
    container.register_singleton(std::sync::Arc::new(LeafService { marker: 42 }));
}

struct LeafModule;

impl Module for LeafModule {
    fn metadata(&self) -> ModuleMetadata {
        ModuleMetadata {
            imports: vec![],
            providers: vec![register_leaf],
            exports: vec![],
        }
    }
}

struct RootModule;

impl Module for RootModule {
    fn metadata(&self) -> ModuleMetadata {
        ModuleMetadata {
            imports: vec![Box::new(LeafModule)],
            providers: vec![],
            exports: vec![],
        }
    }
}

#[test]
fn configure_imports_registers_child_providers() {
    let app = bootstrap(RootModule).expect("bootstrap");
    let leaf = app.container.get::<LeafService>().expect("leaf from import");
    assert_eq!(leaf.marker, 42);
}

#[test]
fn configure_local_providers_after_imports() {
    fn register_root(container: &Container) {
        container.register_default::<RootOnlyService>();
    }

    #[derive(Default)]
    struct RootOnlyService;

    struct CombinedModule;

    impl Module for CombinedModule {
        fn metadata(&self) -> ModuleMetadata {
            ModuleMetadata {
                imports: vec![Box::new(LeafModule)],
                providers: vec![register_root],
                exports: vec![],
            }
        }
    }

    let app = bootstrap(CombinedModule).expect("bootstrap");
    assert!(app.container.get::<LeafService>().is_ok());
    assert!(app.container.get::<RootOnlyService>().is_ok());
}
