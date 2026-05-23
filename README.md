# RustForge Framework

**NestJS DX + performance Rust + simplicité Axum**

Framework backend Rust inspiré de NestJS et Spring Boot — voir [BRD&PRD.md](./BRD&PRD.md).

| Document | Contenu |
|----------|---------|
| [ARCHITECTURE.md](./ARCHITECTURE.md) | Performance, routing/DI compile-time |
| [docs/DX.md](./docs/DX.md) | CLI `forge`, prelude, conventions, roadmap DX |
| [docs/HTTP_FEATURES.md](./docs/HTTP_FEATURES.md) | Features Axum activées |

## Capacités HTTP (Axum)

Toutes les features Axum 0.7 sont activées : **HTTP/1**, **HTTP/2**, **WebSocket**, **multipart**, **form**, **query**, `MatchedPath`, `OriginalUri`, tracing des extracteurs, etc.

```rust
use rustforge::prelude::*;
// Json, Form, Query, Path, AppResult, Multipart, WebSocketUpgrade, …
```

## Principes

| Axe | Choix |
|-----|--------|
| **Performance** | Routes macros → `Router` Axum monomorphisé, DI `get::<T>()` typée, pas de `dyn` par route |
| **DX** | `prelude` unique, `AppResult<T>`, `forge_err!`, messages DI explicites, CLI `forge` |
| **Objectif** | Rust backend **agréable** — pas « le plus rapide », mais enterprise + simple |

## Phase 1 (MVP) ✅

Modules, DI, controllers, routing compile-time, validation, config, logger, CLI `forge`, `AppResult`, exemples `basic-api` et `shop-api`.

## Démarrage

```bash
cargo build
cargo run -p basic-api    # port 3000 — users + products
cargo run -p shop-api     # port 3001 — customers + orders (structure README)
```

| Exemple | Port | Description |
|---------|------|-------------|
| [basic-api](examples/basic-api/) | 3000 | API minimale |
| [shop-api](examples/shop-api/) | 3001 | Boutique : dto, entities, repository, `forge_err!` |

## Exemple (aligné sur `basic-api`)

```rust
use rustforge::prelude::*;

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
    rustforge::logger::init();
    rustforge::bootstrap_app(AppModule)?
        .port(3000)
        .listen()
        .await
}
```

Erreurs métier dans les handlers :

```rust
return Err(forge_err!(Conflict, "Email already exists"));
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
rustforge/
├── core/          # modules, bootstrap
├── di/            # conteneur typé
├── macros/        # #[module], #[controller], #[routes], …
├── http/          # Axum, AppResult
├── router/        # chemins statiques
├── validation/    # Validate, ValidatedJson
├── config/, logger/, cli/, testing/
└── src/           # bootstrap_app, prelude
```

## Structure d’une app (générée par `forge new`)

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

## CLI `forge`

```bash
cargo install --path rustforge/cli   # binaires forge + rustforge
forge new my-api
forge g resource users
forge start --watch
forge doctor
```

## Roadmap

| Phase | Contenu |
|-------|---------|
| **1** | MVP compile-time routing + DI typée + CLI ✅ |
| **2** | JWT/guards ✅, ForgeDB (Prisma-like) ✅, OpenAPI, extracteurs `#[routes]` |
| **3** | microservices, queues, cache, observabilité |

## Licence

MIT
