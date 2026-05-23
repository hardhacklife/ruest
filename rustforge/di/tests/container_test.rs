use rustforge_di::{Container, DiError, Inject, Scope};
use std::sync::Arc;

#[derive(Default)]
struct Counter {
    value: u32,
}

#[test]
fn resolves_singleton_twice() {
    let container = Container::new();
    let descriptor = rustforge_di::default_provider::<Counter>("Counter", Scope::Singleton);
    container.register::<Counter>(descriptor);

    let a = container.get::<Counter>().unwrap();
    let b = container.get::<Counter>().unwrap();
    assert!(Arc::ptr_eq(&a, &b));
}

#[test]
fn register_singleton_and_get() {
    let container = Container::new();
    container.register_singleton(Arc::new(Counter { value: 7 }));

    let c = container.get::<Counter>().unwrap();
    assert_eq!(c.value, 7);
}

#[test]
fn register_default_via_service_pattern() {
    let container = Container::new();
    container.register_default::<Counter>();

    let c = container.get::<Counter>().unwrap();
    assert_eq!(c.value, 0);
}

#[test]
fn not_found_includes_type_name() {
    let container = Container::new();
    let err = match container.get::<Counter>() {
        Err(e) => e,
        Ok(_) => panic!("expected missing Counter"),
    };
    match err {
        DiError::NotFound { type_name } => {
            assert!(type_name.contains("Counter"));
        }
        other => panic!("expected NotFound, got {other:?}"),
    }
}

#[test]
fn inject_resolves_from_container() {
    let container = Container::new();
    container.register_default::<Counter>();

    let inject = Inject::<Counter>::resolve(&container).unwrap();
    assert_eq!(inject.value, 0);
}
