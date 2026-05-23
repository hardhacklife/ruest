# Developer Experience (DX) — RustForge / Forge

> **Mission :** rendre Rust backend **simple, agréable et rapide à développer** — pas seulement performant.

Rust est déjà rapide. Ce qui manque, c’est un framework **agréable** comme NestJS, sans exposer lifetimes, `Arc<Mutex<T>>`, et traits partout.

## Philosophie

| Principe | Implémentation |
|----------|----------------|
| Masquer la complexité Rust | `Inject<T>`, `#[service]`, `prelude` unique |
| Convention over configuration | Structure `src/modules/`, `#[module]` |
| Messages humains | `[Forge DI]`, `[Forge] Bad Request`, `forge_err!` |
| Compile-time > runtime reflection | Macros routes + DI typée |
| CLI first | Commande **`forge`** (alias `rustforge`) |

## Une seule importation

```rust
use rustforge::prelude::*;
// ou à terme : use forge::prelude::*;
```

Tout le nécessaire : macros, `Json`, `AppResult`, `Inject`, `bootstrap_app`, `Validate`, etc.

## Style NestJS

```rust
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
}

#[service]
#[derive(Default)]
pub struct UserService { /* ... */ }

#[module(controllers = [UserController], providers = [UserService])]
pub struct UsersModule;
```

### Module racine (`imports`)

Évite de dupliquer controllers/services dans `app_module.rs` : chaque domaine déclare son `#[module]`, le module racine ne compose que les sous-modules.

```rust
use crate::modules::users::UsersModule;
use crate::modules::orders::OrdersModule;

#[module(imports = [UsersModule, OrdersModule])]
pub struct AppModule;
```

`imports` enregistre la DI des enfants et monte leurs routes avant les controllers locaux (s’il y en a).

## CLI `forge`

| Commande | Description |
|----------|-------------|
| `forge new my-api` | App avec `modules/`, `config/`, `common/` |
| `forge new my-api -t microservice` | Template microservice |
| `forge g resource users` | Scaffold complet (dto, entity, repo, service, controller, module) |
| `forge g module users` | Module seul |
| `forge g controller users` | Controller |
| `forge g service users` | Service |
| `forge start` | `cargo run` |
| `forge start --watch` | Hot reload (`cargo-watch`) |
| `forge build` | Build incremental (profil dev) |
| `forge test` | Tests |
| `forge doctor` | Vérifie la structure du projet |

## Structure imposée (anti-chaos)

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

## Erreurs lisibles

### DI

```
[Forge DI] Service `my_app::UserService` is not registered.
Did you forget to add it to your module?

Try:
#[module(controllers = [...], providers = [UserService])]
```

### HTTP

```rust
async fn handler() -> AppResult<Json<User>> {
    if exists {
        return Err(forge_err!(Conflict, "Email already exists"));
    }
    Ok(Json(user))
}
```

## Validation (style FastAPI)

```rust
#[derive(Validate, serde::Deserialize)]
pub struct CreateUserDto {
    #[validate(email)]
    pub email: String,
}
// ValidatedJson<CreateUserDto> dans les handlers (extracteurs — roadmap)
```

## Roadmap DX (priorités)

| Priorité | Feature | Statut |
|----------|---------|--------|
| P0 | CLI `forge` + `g resource` | ✅ |
| P0 | Prelude + AppResult | ✅ |
| P0 | DI erreurs humaines | ✅ |
| P1 | Extracteurs dans `#[routes]` (body, ValidatedJson) | 🔜 |
| P1 | OpenAPI / Swagger auto (utoipa) | 🔜 |
| P1 | JWT / guards / `with_jwt_auth` | ✅ voir [SECURITY.md](SECURITY.md) |
| P1 | `forge add postgres/redis` | 🔜 |
| P2 | `forge studio` (routes, providers, logs) | 🔜 |
| P2 | `forge explain error` (AI) | 🔜 |
| P3 | Playground cloud | 🔜 |

## Build rapide

- `incremental = true` dans les apps générées (profil dev)
- `forge start --watch` pour recompiler au save
- Compilation distribuée : roadmap

## Ce qu’on évite volontairement

- Reflection runtime (Java / Nest metadata lourde)
- `Box<dyn>` par route
- Forcer `Result<Result<T>>` — utiliser `AppResult<T>`
- Macros illisibles de 500 lignes

## Références

- [ARCHITECTURE.md](../ARCHITECTURE.md) — performance & compile-time
- [HTTP_FEATURES.md](./HTTP_FEATURES.md) — Axum features
- [BRD&PRD.md](../BRD&PRD.md) — vision produit
