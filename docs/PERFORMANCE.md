# Tester les performances de RUEST

Comparer RUEST à Axum pur, Actix, NestJS, etc. demande une **méthodologie stricte**. Sinon les chiffres ne veulent rien dire.

## Ce que vous mesurez vraiment

RUEST **n’est pas un serveur HTTP séparé** : c’est Axum + Tower + vos modules. L’écart de perf vient surtout de :

| Source d’overhead | Impact |
|-------------------|--------|
| Layers par défaut (`Trace`, CORS, logger) | Moyen à fort en charge |
| `Arc<Controller>` + DI au bootstrap | Faible par requête (résolu au `wire`) |
| JSON sérialisation | Identique si même handler |
| JWT (`with_jwt_auth`) | Moyen (vérif token par requête) |
| Logs `tracing` par requête | Moyen |

**Attendu réaliste** (voir [ARCHITECTURE.md](../ARCHITECTURE.md)) : proche d’**Axum pur**, un peu au-dessus si tous les middlewares sont actifs.

---

## 1. Préparer les binaires (obligatoire)

Toujours compiler en **release** avec LTO pour comparer :

```bash
export RUSTFLAGS="-C target-cpu=native"
cargo build --release -p basic-api
cargo build --release -p axum-baseline   # baseline du repo benchmarks/
```

Variables utiles pendant les tests :

```bash
export RUST_LOG=off          # désactive le bruit tracing
export RUEST_BENCH=1         # si vous ajoutez un mode bench sans layers (voir ci-dessous)
```

---

## 2. Tests de charge HTTP (comparaison frameworks)

Outils recommandés (installer un seul suffit) :

| Outil | Installation | Usage |
|-------|--------------|-------|
| [wrk](https://github.com/wg/wrk) | `brew install wrk` | Throughput, latence |
| [hey](https://github.com/rakyll/hey) | `go install …` | Simple, cross-platform |
| bombardier | binaire Go | Fort débit |

### Script fourni

```bash
# Terminal 1 — RUEST
cargo run --release -p basic-api

# Terminal 2 — baseline Axum (même route /users)
cargo run --release -p axum-baseline

# Terminal 3 — charge
./benchmarks/scripts/load_test.sh http://127.0.0.1:3000/users
./benchmarks/scripts/load_test.sh http://127.0.0.1:3001/users   # si baseline sur 3001
```

Le script affiche : **req/s**, **latence p50/p99**, erreurs.

### Scénarios à comparer (même machine, même réseau loopback)

1. **GET JSON** — `GET /users` (liste) — scénario principal  
2. **GET minimal** — `GET /health` — overhead routing seul  
3. **POST JSON** — création utilisateur (si vous comparez écriture)  
4. **Avec / sans JWT** — `shop-api` + `with_jwt_auth` vs `basic-api`

### Comparer à d’autres stacks

| Stack | Comment |
|-------|---------|
| **Axum pur** | `benchmarks/axum-baseline` dans ce repo |
| **Actix Web** | Projet minimal équivalent (même JSON, même route) |
| **NestJS** | `nest start` + même `wrk` ; Node en `--max-old-space-size` si besoin |
| **Express** | Idem |

Notez toujours : **CPU, RAM, OS, version Rust/Node**, nombre de cœurs wrk (`-t`), connexions (`-c`), durée (`-d`).

---

## 3. Micro-benchmarks (overhead framework)

Pour isoler la DI et le routing **sans réseau** :

```bash
cargo bench -p ruest-di
```

Mesure typiquement : résolution `Container::get::<T>()` (singleton en cache vs premier accès).

Pour le routing statique :

```bash
cargo bench -p ruest-router
```

---

## 4. Profiling (où part le temps CPU)

Quand wrk montre un écart, profilez :

```bash
# Installer : cargo install flamegraph
cargo flamegraph --release -p basic-api --bin basic-api
# Puis relancer wrk pendant la capture
```

Ou avec `perf` (Linux) / Instruments (macOS) sur le processus `basic-api`.

Cherchez : `tracing`, `serde_json`, middleware Tower, pas seulement RUEST.

---

## 5. Mode bench « layers minimales » (comparaison honnête)

Par défaut, `ruest::serve` ajoute Trace + CORS + Logger. Pour comparer **uniquement** le coût modules/DI/routes :

Option A — baseline Axum sans layers (déjà dans `axum-baseline`).

Option B — lancer avec variables (si implémenté dans votre fork) :

```rust
// Idée : dans finalize_router, sauter les layers si std::env::var("RUEST_BENCH").is_ok()
```

Sans cela, comparer RUEST « out of the box » à Axum minimal **pénalise** RUEST — c’est normal et documenté.

---

## 6. Grille de résultats (template)

Remplissez après chaque campagne sur **la même machine** :

| Scénario | RUEST basic-api | Axum baseline | NestJS | Notes |
|----------|---------------------|---------------|--------|-------|
| GET /users req/s | | | | threads=4, conn=100, 30s |
| GET /users p99 ms | | | | |
| GET /health req/s | | | | |
| RAM idle Mo | | | | `ps` ou Activity Monitor |
| shop-api + JWT req/s | | | | |

---

## 7. Erreurs fréquentes

- Comparer **debug** vs **release**  
- Oublier `RUST_LOG=off` (tracing fausse tout)  
- Tester en local avec d’autres apps qui consomment le CPU  
- Un seul run wrk (refaire 3 fois, prendre la médiane)  
- Comparer `GET /users` RUEST à `Hello World` ailleurs  

---

## 8. Commandes rapides (copier-coller)

```bash
# Build release
cargo build --release -p basic-api -p axum-baseline

# RUEST :3000
RUST_LOG=off cargo run --release -p basic-api &

# Charge 30s
wrk -t4 -c100 -d30s http://127.0.0.1:3000/users

# Arrêt
kill %1
```

---

## Liens

- [PRINCIPES.md](./PRINCIPES.md) — performant = compile-time, pas « le plus rapide du marché »  
- [benchmarks/README.md](../benchmarks/README.md) — baseline et scripts du repo
