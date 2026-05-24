# Benchmarks RUEST

Comparaison de charge **RUEST** vs **Axum minimal** (même handler JSON).

## Démarrage rapide

```bash
# 1. Build release
cargo build --release -p basic-api -p axum-baseline

# 2. Terminal A — RUEST (port 3000)
RUST_LOG=off cargo run --release -p basic-api

# 3. Terminal B — Axum baseline (port 3001)
RUST_LOG=off cargo run --release -p axum-baseline

# 4. Mesures
./benchmarks/scripts/load_test.sh http://127.0.0.1:3000/users ruest
./benchmarks/scripts/load_test.sh http://127.0.0.1:3001/users axum-baseline
```

Voir [docs/PERFORMANCE.md](../docs/PERFORMANCE.md) pour la méthodologie complète (NestJS, Actix, profiling, micro-benches).

## Contenu

| Chemin | Rôle |
|--------|------|
| `axum-baseline/` | GET `/users` JSON — sans DI, sans layers RUEST |
| `scripts/load_test.sh` | wrk ou hey, sortie req/s + latence |
