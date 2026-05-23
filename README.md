# RustForge Framework

**NestJS DX + performance Rust + simplicité Axum** — voir [ARCHITECTURE.md](./ARCHITECTURE.md).

Framework backend Rust inspiré de NestJS et Spring Boot, défini dans [BRD&PRD.md](./BRD&PRD.md).

## Capacités HTTP (Axum)

Toutes les features Axum 0.7 sont activées : **HTTP/1**, **HTTP/2**, **WebSocket**, **multipart**, **form**, **query**, `MatchedPath`, `OriginalUri`, tracing des extracteurs, etc.

```rust
use rustforge::prelude::*;
// Json, Form, Query, Path, Multipart, WebSocketUpgrade, MatchedPath, OriginalUri, ConnectInfo
```

## Principes performance

| Choix | Pourquoi |
|-------|----------|
| Routes générées par macros → `Router` Axum | Handlers **monomorphisés**, pas de `dyn` par route |
| DI `get::<T>()` + `register_singleton` | Résolution **typée**, pas de reflection Spring |
| `Arc` pour services / contrôleurs | Partage zero-copy, pas de clone profond |
| Pas de registre runtime lourd | Évite le coût Nest/Spring metadata |

**Objectif réaliste :** meilleure DX enterprise Rust, légèrement au-dessus d’Axum pur en coût, bien en-dessous de NestJS en latence.

## Phase 1 (MVP)

- Modules, DI, controllers, routing compile-time, validation, config, logger, CLI
- Exemple : `examples/basic-api`

## Démarrage

```bash
cargo build
cargo run -p basic-api
```

- `GET http://localhost:3000/users/`
- `POST http://localhost:3000/users/`
- `GET http://localhost:3000/health`

## Exemple

```rust
#[service]
#[derive(Default)]
pub struct UserService { /* ... */ }

#[controller("/users")]
pub struct UserController {
    service: Inject<UserService>,
}

#[routes]
impl UserController {
    #[get("/")]
    async fn get_users(&self) -> Json<Vec<User>> { /* ... */ }
}

#[module(controllers = [UserController], providers = [UserService])]
pub struct UsersModule;
```

```rust
rustforge::bootstrap_app(UsersModule)?
    .port(3000)
    .listen()
    .await
```

## Structure

```text
rustforge/
├── core/       # modules, bootstrap DI
├── di/         # Container typé
├── macros/     # compile-time routes + providers
├── http/       # Axum (router assemblé au bootstrap)
├── router/     # helpers chemins statiques
├── config/, validation/, logger/, cli/, testing/
```

## CLI

```bash
cargo run -p rustforge-cli -- new my-app
cargo run -p rustforge-cli -- generate resource users
```

## Roadmap

| Phase | Contenu |
|-------|---------|
| **1** | MVP compile-time routing + DI typée ✅ |
| **2** | auth, ORM, OpenAPI, WebSocket |
| **3** | microservices, queues, cache, observabilité |

## Licence

MIT
