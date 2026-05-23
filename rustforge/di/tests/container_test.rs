use rustforge_di::{Container, Scope};
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
