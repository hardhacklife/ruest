# RUEST

**Rust + NestJS** — NestJS DX, performance Rust, simplicité Axum.

Framework backend Rust inspiré de NestJS et Spring Boot — voir [BRD&PRD.md](./BRD&PRD.md).

| Document | Contenu |
|----------|---------|
| [docs/PRINCIPES.md](./docs/PRINCIPES.md) | **Modulaire · typé · sécurisé · fonctionnel** (piliers du framework) |
| [ARCHITECTURE.md](./ARCHITECTURE.md) | Performance, routing/DI compile-time |
| [docs/DX.md](./docs/DX.md) | CLI `ruest`, prelude, conventions, roadmap DX |
| [docs/SECURITY.md](./docs/SECURITY.md) | JWT, guards, `with_jwt_auth` |
| [docs/RUESTDB.md](./docs/RUESTDB.md) | Schema DSL, migrations, client généré |
| [docs/HTTP_FEATURES.md](./docs/HTTP_FEATURES.md) | Features Axum activées |
| [docs/PERFORMANCE.md](./docs/PERFORMANCE.md) | Benchmarks, wrk, comparaison Axum/NestJS |

## Capacités HTTP (Axum)

Toutes les features Axum 0.7 sont activées : **HTTP/1**, **HTTP/2**, **WebSocket**, **multipart**, **form**, **query**, `MatchedPath`, `OriginalUri`, tracing des extracteurs, etc.

```rust
use ruest::prelude::*;
// Json, Form, Query, Path, AppResult, Multipart, WebSocketUpgrade, …
```

## Piliers du framework

| Pilier | En bref | Doc |
|--------|---------|-----|
| **Modulaire** | `#[module]` + `imports`, un crate par responsabilité | [PRINCIPES.md](./docs/PRINCIPES.md#1-modulaire) |
| **Typé** | `Inject<T>`, `AppResult`, macros compile-time, RuestDB généré | [PRINCIPES.md](./docs/PRINCIPES.md#2-typé-type-safe) |
| **Sécurisé** | JWT, guards, validation, SQL paramétré (SQLx) | [SECURITY.md](./docs/SECURITY.md) |
| **Fonctionnel** | Bootstrap → routes → tests d’intégration sur chaque brique | [PRINCIPES.md](./docs/PRINCIPES.md#4-fonctionnel) |
| **Performant** | Pas de `dyn` par route, DI monomorphisée | [ARCHITECTURE.md](./ARCHITECTURE.md) |
| **DX** | `prelude`, `ruest_err!`, CLI `ruest` | [DX.md](./docs/DX.md) |

```bash
cargo test -p ruest --test principles   # smoke des piliers
```

## Phase 1 (MVP) ✅

Modules, DI, controllers, routing compile-time, validation, config, logger, CLI `ruest`, `AppResult`, exemples `basic-api` et `shop-api`.

## Installation (crates.io)

Après publication sur [crates.io](https://crates.io/crates/ruest) :

```bash
cargo add ruest              # framework dans votre projet
cargo install ruest-cli      # commande `ruest` (new, g, start, db, …)
```

Guide de publication pour les mainteneurs : [docs/PUBLISHING.md](./docs/PUBLISHING.md).

## Démarrage (monorepo)

```bash
cargo build
cargo run -p basic-api    # port 3000 — users + products
cargo run -p shop-api     # port 3001 — customers + orders (structure README)
```

| Exemple | Port | Description |
|---------|------|-------------|
| [basic-api](examples/basic-api/) | 3000 | API minimale |
| [shop-api](examples/shop-api/) | 3001 | Boutique : dto, entities, repository, `ruest_err!` |

## Exemple (aligné sur `basic-api`)

```rust
use ruest::prelude::*;

#[service]
pub struct UserService { /* … */ }

impl Default for UserService { /* … */ }

#[controller("/users")]
pub struct UserController {
    service: Inject<UserService>,
}

#[routes]
impl UserController {
    #[get("/")]
    async fn get_users(&self) -> AppResult<Json<Vec<User>>> {
        Ok(Json(self.service.find_all().await))
    }

    #[post("/")]
    async fn create_user(&self) -> AppResult<Json<User>> {
        // …
        Ok(Json(user))
    }
}

#[module(controllers = [UserController], providers = [UserService])]
pub struct UsersModule;

// Module racine (shop-api, basic-api) — pas besoin de relister controllers/services :
#[module(imports = [UsersModule, ProductsModule])]
pub struct AppModule;
```

```rust
#[tokio::main]
async fn main() -> Result<(), CoreError> {
    ruest::logger::init();
    ruest::bootstrap_app(AppModule)?
        .port(3000)
        .listen()
        .await
}
```

Erreurs métier dans les handlers :

```rust
return Err(ruest_err!(Conflict, "Email already exists"));
```

### Sécurité JWT

```rust
bootstrap_app(AppModule)?
    .with_jwt_auth(SecurityConfig::dev())?  // ou SecurityConfig::from_env()?
    .port(3000)
    .listen()
    .await?;
```

Voir [docs/SECURITY.md](docs/SECURITY.md) — guards `#[guard(roles = ["admin"])]`, extracteur `AuthUser`, routes publiques `/auth/login`, etc.

## Structure du framework

```text
ruest/
├── core/          # modules, bootstrap
├── di/            # conteneur typé
├── macros/        # #[module], #[controller], #[routes], …
├── http/          # Axum, AppResult
├── security/      # JWT, guards, AuthUser
├── router/        # chemins statiques
├── validation/    # Validate, ValidatedJson
├── config/, logger/, cli/, testing/
└── src/           # bootstrap_app, prelude

ruest-db/            # RuestDB (schema.ruest, migrations, client SQLx)
├── schema/, parser/, codegen/, runtime/, migrate/
```

## Structure d’une app (générée par `ruest new`)

```text
src/
├── main.rs
├── config/
├── common/
└── modules/
    └── users/
        ├── dto/
        ├── entities/
        ├── users.controller.rs
        ├── users.service.rs
        ├── users.repository.rs
        └── users.module.rs
```

## CLI `ruest`

```bash
cargo install --path ruest/cli   # binaire `ruest`
ruest new my-api
ruest g resource users
ruest start --watch
ruest doctor
```

## Roadmap

| Phase | Contenu |
|-------|---------|
| **1** | MVP compile-time routing + DI typée + CLI ✅ |
| **2** | JWT/guards ✅, RuestDB (Prisma-like) ✅, OpenAPI, extracteurs `#[routes]` |
| **3** | microservices, queues, cache, observabilité |

## Licence

MIT
