use criterion::{black_box, criterion_group, criterion_main, Criterion};
use ruest_router::join_paths;

fn bench_join_paths(c: &mut Criterion) {
    c.bench_function("join_paths_api_users", |b| {
        b.iter(|| black_box(join_paths("/api", "/users")));
    });
}

criterion_group!(benches, bench_join_paths);
criterion_main!(benches);
