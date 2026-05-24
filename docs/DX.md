# Developer Experience (DX) — RUEST

> **Mission :** rendre Rust backend **simple, agréable et rapide à développer** — pas seulement performant.

Rust est déjà rapide. Ce qui manque, c’est un framework **agréable** comme NestJS, sans exposer lifetimes, `Arc<Mutex<T>>`, et traits partout.

## Philosophie

Voir le guide complet des piliers : **[PRINCIPES.md](./PRINCIPES.md)** (modulaire, typé, sécurisé, fonctionnel).

| Principe | Implémentation |
|----------|----------------|
| Modulaire | `#[module(imports)]`, crates `core` / `http` / `security` / `ruest-db` |
| Typé | `Inject<T>`, `AppResult`, macros compile-time |
| Sécurisé | `with_jwt_auth`, `#[guard]`, `ValidatedJson` |
| Fonctionnel | Exemples + `cargo test -p ruest --test principles` |
| Masquer la complexité Rust | `prelude` unique |
| Messages humains | `[Ruest DI]`, `ruest_err!` |
| CLI first | Commande **`ruest`** (alias `ruest`) |

## Une seule importation

```rust
use ruest::prelude::*;
// ou à terme : use ruest::prelude::*;
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

## CLI `ruest`

| Commande | Description |
|----------|-------------|
| `ruest new my-api` | App avec `modules/`, `config/`, `common/` |
| `ruest new my-api -t microservice` | Template microservice |
| `ruest g resource users` | Scaffold complet (dto, entity, repo, service, controller, module) |
| `ruest g module users` | Module seul |
| `ruest g controller users` | Controller |
| `ruest g service users` | Service |
| `ruest start` | `cargo run` |
| `ruest start --watch` | Hot reload (`cargo-watch`) |
| `ruest build` | Build incremental (profil dev) |
| `ruest test` | Tests |
| `ruest doctor` | Vérifie la structure du projet |

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
[Ruest DI] Service `my_app::UserService` is not registered.
Did you forget to add it to your module?

Try:
#[module(controllers = [...], providers = [UserService])]
```

### HTTP

```rust
async fn handler() -> AppResult<Json<User>> {
    if exists {
        return Err(ruest_err!(Conflict, "Email already exists"));
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
| P0 | CLI `ruest` + `g resource` | ✅ |
| P0 | Prelude + AppResult | ✅ |
| P0 | DI erreurs humaines | ✅ |
| P1 | Extracteurs dans `#[routes]` (body, ValidatedJson) | 🔜 |
| P1 | OpenAPI / Swagger auto (utoipa) | 🔜 |
| P1 | JWT / guards / `with_jwt_auth` | ✅ voir [SECURITY.md](SECURITY.md) |
| P1 | RuestDB (schema.ruest, migrations, client) | ✅ voir [RUESTDB.md](RUESTDB.md) |
| P1 | `ruest add redis` | 🔜 |
| P2 | `ruest studio` (routes, providers, logs) | 🔜 |
| P2 | `ruest explain error` (AI) | 🔜 |
| P3 | Playground cloud | 🔜 |

## Build rapide

- `incremental = true` dans les apps générées (profil dev)
- `ruest start --watch` pour recompiler au save
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
