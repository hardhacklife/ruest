# Architecture RUEST

## Philosophie

**Objectif :** « NestJS DX + performance Rust + simplicité Axum » — pas un clone Spring/Nest avec reflection runtime.

| Priorité | Choix |
|----------|--------|
| **Modulaire** | Crates séparés + `#[module(imports)]` |
| **Typé** | DI `get::<T>()`, macros routes, `AppResult` |
| **Sécurisé** | JWT, guards, validation, SQL paramétré (RuestDB) |
| **Fonctionnel** | Chaque brique testée (voir `ruest/tests/principles.rs`) |
| DX enterprise | Modules, attributs, DI, conventions |
| Performance | Routing et résolution DI au **compile-time** autant que possible |
| Simplicité runtime | Axum/Tower natifs, pas de métadonnées runtime lourdes |

Voir [docs/PRINCIPES.md](./docs/PRINCIPES.md) pour le détail de chaque pilier.

Nous ne visons pas « le framework Rust le plus rapide » (Actix/Axum purs le sont déjà). Nous visons **le meilleur équilibre DX enterprise / coût runtime**.

## Couches

| Couche | Technologie | Rôle |
|--------|-------------|------|
| Runtime | Tokio | Concurrence async |
| HTTP | Hyper (via Axum) | Transport HTTP/1 + HTTP/2 |
| Router | Axum (généré par macros) | Routes **monomorphisées** au compile-time |

### Features Axum (activées par défaut)

RUEST active **toutes** les features optionnelles d'Axum 0.7 :

`http1`, `http2`, `json`, `macros`, `matched-path`, `multipart`, `original-uri`, `tokio`, `tower-log`, `tracing`, `ws`, `form`, `query`.

Extracteurs disponibles via `ruest::prelude` / `ruest::http` : `Json`, `Form`, `Query`, `Path`, `Multipart`, `MatchedPath`, `OriginalUri`, `WebSocketUpgrade`, `ConnectInfo`, etc.

| DI | `Container` typé + macros | Enregistrement statique, `get::<T>()` monomorphisé |
| Validation | Serde + validator | DTOs |
| Sécurité | `ruest-security` | JWT, guards, middleware |
| Données | `ruest-db/*` | Schema DSL, migrations, client typé |
| Middleware | Tower | Pipeline HTTP |
| Sérialisation | Serde / `Json` | JSON |

## Routing compile-time

Les macros `#[controller]`, `#[routes]`, `#[get]` / `#[post]` génèrent :

- des chemins `&'static str` (concaténation à la compilation) ;
- des handlers Axum **monomorphisés** (`get`, `post`, …) ;
- **une** fonction `mount(router, Arc<Controller>) -> Router` par contrôleur.

**Évité :** `Arc<dyn RouteHandler>` par route, registre runtime avec reflection.

**Coût restant :** un `Arc<Controller>` partagé par route (cheap) ; comparable à Axum + état partagé.

## DI sans reflection

- Chaque `#[service]` génère `register_provider` qui appelle `Container::register_singleton::<T>`.
- `Inject<T>` et `get::<T>()` sont **monomorphisés** : le compilateur connaît `T` à chaque site d’appel.
- Les singletons sont stockés en `Arc<T>` (une conversion `TypeId` au premier accès, puis cache typé).

**Évité :** conteneur style Spring avec métadonnées runtime, factories `dyn` par défaut.

**Extensions futures :** conteneur entièrement généré par `#[module]` (`struct AppServices { user: Arc<UserService>, … }`) pour zéro `HashMap` au hot path.

## Zero-copy (principes)

- Préférer `&str`, slices, `Bytes`, `Cow` dans les DTOs et réponses quand c’est possible.
- `Arc<T>` pour l’état partagé (services, contrôleurs), pas de clone profond des services.
- Éviter les allocations dans le hot path des handlers (chemins statiques, pas de `format!` par requête pour le routing).

## Ce que nous limitons volontairement

- Pas de `Box<dyn Trait>` par route.
- Pas de métadonnées runtime pour les décorateurs.
- Macros ciblées (pas de génération de milliers de lignes par fichier).
- Plugins : un seul niveau d’indirection (`dyn Plugin`), pas dans la chaîne requête/réponse.

## Comparaison réaliste (cible)

| vs | Attendu |
|----|---------|
| NestJS | Beaucoup plus rapide, même DX approximative |
| Spring Boot | Meilleure latence / RAM |
| Axum pur | Légèrement plus lent (abstractions DI/modules) |
| Actix pur | Probablement un peu plus lent |

## Fichiers clés

- `ruest/macros/` — génération compile-time routes + DI
- `ruest/di/container.rs` — résolution typée `get::<T>()`
- `ruest/http/server.rs` — assemblage `Router` Axum
- `ruest/src/bootstrap.rs` — bootstrap HTTP sans registre dynamique

## Developer Experience

Voir [docs/DX.md](./docs/DX.md) — CLI `ruest`, prelude, `AppResult`, génération de ressources, conventions projet.
