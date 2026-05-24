//! Micro-benchmark : résolution DI singleton (hot path après enregistrement).

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use ruest_di::{Container, Scope};

#[derive(Default)]
struct BenchService {
    value: u64,
}

fn bench_get_singleton(c: &mut Criterion) {
    let container = Container::new();
    let descriptor = ruest_di::default_provider::<BenchService>("BenchService", Scope::Singleton);
    container.register::<BenchService>(descriptor);
    // Warm cache
    let _ = container.get::<BenchService>().unwrap();

    c.bench_function("container_get_singleton_cached", |b| {
        b.iter(|| {
            let svc = container.get::<BenchService>().unwrap();
            black_box(svc.value);
        });
    });
}

criterion_group!(benches, bench_get_singleton);
criterion_main!(benches);
