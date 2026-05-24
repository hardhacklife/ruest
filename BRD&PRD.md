# BRD & PRD — Framework Backend Rust inspiré de NestJS et Spring Boot

## Nom du projet

**Nom provisoire :** RUEST Framework

---

# 1. BUSINESS REQUIREMENTS DOCUMENT (BRD)

# 1.1 Vision du projet

Créer un framework backend moderne en Rust inspiré de :

* NestJS
* Spring Boot
* FastAPI
* ASP.NET Core

Le framework doit permettre aux développeurs de construire rapidement des APIs, microservices et applications backend robustes avec :

* une architecture modulaire,
* l’injection de dépendances,
* les décorateurs/annotations,
* la validation,
* les middlewares,
* la sécurité,
* le support ORM,
* le support WebSocket,
* une CLI,
* une expérience développeur moderne.

Le framework doit combiner :

* les performances de Rust,
* la sécurité mémoire,
* la scalabilité,
* et l’expérience développeur simple de NestJS ou Spring Boot.

---

# 1.2 Objectifs business

## Objectifs principaux

1. Simplifier le développement backend en Rust.
2. Réduire la complexité actuelle de l’écosystème Rust backend.
3. Fournir un framework opinionated prêt pour la production.
4. Créer un écosystème complet similaire à NestJS.
5. Permettre aux développeurs Node.js, Java et Go de migrer facilement vers Rust.
6. Créer une architecture scalable pour les APIs modernes.
7. Permettre le développement rapide de microservices.

---

# 1.3 Problèmes actuels du marché

## Problèmes dans Rust backend

### Complexité élevée

Les frameworks Rust existants demandent souvent beaucoup de configuration.

### Architecture non standardisée

Chaque projet Rust backend a une structure différente.

### Mauvaise expérience développeur

Il manque souvent :

* une CLI complète,
* un système de modules,
* un système DI mature,
* une structure claire.

### Courbe d’apprentissage élevée

Les développeurs venant de Node.js ou Java trouvent Rust difficile.

### Intégration complexe

Les outils ORM, validation, auth et WebSocket sont souvent séparés.

---

# 1.4 Public cible

## Développeurs backend

* Développeurs Node.js
* Développeurs Java Spring Boot
* Développeurs Go
* Développeurs Python FastAPI
* Développeurs microservices

## Entreprises

* Startups
* SaaS
* plateformes fintech
* plateformes streaming
* e-commerce
* applications temps réel

---

# 1.5 Valeur ajoutée

Le framework doit fournir :

| Fonctionnalité                | Valeur                  |
| ----------------------------- | ----------------------- |
| Performance Rust              | APIs très rapides       |
| Sécurité mémoire              | Moins de bugs critiques |
| Architecture modulaire        | Maintenance simplifiée  |
| Dependency Injection          | Code propre             |
| CLI moderne                   | Productivité élevée     |
| Décorateurs                   | Développement rapide    |
| Convention over configuration | Standardisation         |
| Async natif                   | Haute concurrence       |
| Microservices                 | Scalabilité             |
| WebSocket                     | Temps réel              |

---

# 1.6 KPI techniques

## Performance

* Temps de réponse inférieur à Node.js
* Consommation mémoire optimisée
* Haute concurrence async

## Productivité

* Génération automatique de modules
* Génération automatique de CRUD
* CLI complète

## Adoption

* Documentation complète
* Templates starter
* Exemples prêts à utiliser

---

# 1.7 Vision long terme

## Phase 1

Créer le core framework.

## Phase 2

Créer :

* ORM officiel
* système auth
* système cache
* système queue
* microservices

## Phase 3

Créer un écosystème :

* marketplace plugins
* cloud deployment
* observabilité
* dashboard admin

---

# 2. PRODUCT REQUIREMENTS DOCUMENT (PRD)

# 2.1 Vision produit

Le framework doit permettre ceci :

```rust
#[controller("/users")]
pub struct UserController {
    service: Inject<UserService>
}

#[get("/")]
async fn get_users(&self) -> Json<Vec<User>> {
    self.service.find_all().await
}
```

Avec une expérience similaire à NestJS.

---

# 2.2 Architecture globale

# Architecture principale

Le framework sera composé de plusieurs crates Rust.

## Structure globale

```text
ruest/
├── core/
├── macros/
├── http/
├── router/
├── di/
├── config/
├── validation/
├── auth/
├── orm/
├── websocket/
├── cache/
├── queue/
├── cli/
├── testing/
├── microservices/
├── openapi/
├── logger/
└── telemetry/
```

---

# 2.3 Core framework

## Responsabilités

Le core doit gérer :

* le cycle de vie application,
* les modules,
* l’initialisation,
* les providers,
* les hooks,
* les services globaux.

## Fonctionnalités

### Application bootstrap

Exemple :

```rust
#[tokio::main]
async fn main() {
    RuestFactory::create(AppModule)
        .listen(3000)
        .await;
}
```

---

# 2.4 Système de modules

## Objectif

Permettre une architecture modulaire similaire à NestJS.

## Exemple

```rust
#[module(
    controllers = [UserController],
    providers = [UserService],
    imports = [DatabaseModule],
    exports = [UserService]
)]
pub struct UserModule;
```

## Fonctionnalités attendues

* import/export modules,
* providers,
* scope singleton,
* scope request,
* lazy loading,
* modules dynamiques.

---

# 2.5 Dependency Injection

## Objectif

Créer un conteneur DI complet.

## Fonctionnalités

### Providers

```rust
#[service]
pub struct UserService {}
```

### Injection automatique

```rust
pub struct UserController {
    service: Inject<UserService>
}
```

### Types supportés

* singleton,
* transient,
* request scope.

### Fonctionnalités avancées

* circular dependency handling,
* async providers,
* factory providers,
* conditional providers,
* lazy providers.

---

# 2.6 HTTP Server

## Objectif

Créer une couche HTTP moderne.

## Basé sur

* Hyper
* Axum
* Tokio

## Fonctionnalités

### Routing

```rust
#[get("/")]
#[post("/")]
#[put("/:id")]
#[delete("/:id")]
```

### Middleware

```rust
#[middleware]
pub async fn logger(req: Request, next: Next) {}
```

### Request extraction

```rust
async fn create_user(
    body: Json<CreateUserDto>,
    params: Path<UserParams>
)
```

### Support

* JSON,
* multipart,
* streaming,
* SSE,
* file upload,
* cookies,
* compression,
* CORS.

---

# 2.7 Macros système

## Objectif

Créer un système macros ergonomique.

## Macros principales

### Controllers

```rust
#[controller]
```

### Routes

```rust
#[get]
#[post]
#[put]
#[delete]
#[patch]
```

### Services

```rust
#[service]
```

### Modules

```rust
#[module]
```

### Middleware

```rust
#[middleware]
```

### Guards

```rust
#[guard]
```

### Validation

```rust
#[validate]
```

### DTO

```rust
#[dto]
```

---

# 2.8 Validation

## Objectif

Validation automatique des requêtes.

## Exemple

```rust
#[derive(Validate)]
pub struct CreateUserDto {
    #[email]
    email: String,

    #[length(min = 8)]
    password: String,
}
```

## Fonctionnalités

* validation automatique,
* erreurs standardisées,
* transformation automatique,
* sanitization.

---

# 2.9 ORM Integration

## Objectif

Intégrer un ORM officiel.

## Options

* SeaORM
* SQLx
* ORM custom

## Fonctionnalités

### Entités

```rust
#[entity]
pub struct User {
    #[id]
    id: Uuid,

    name: String,
}
```

### Repository pattern

```rust
pub struct UserRepository;
```

### Support DB

* PostgreSQL,
* MySQL,
* SQLite,
* MongoDB.

### Migrations

CLI migration support.

---

# 2.10 Authentication & Security

## Fonctionnalités

### JWT

```rust
#[use_guard(JwtGuard)]
```

### Roles

```rust
#[roles("admin")]
```

### Features

* JWT,
* OAuth2,
* session auth,
* RBAC,
* permissions,
* API keys,
* CSRF,
* rate limiting,
* encryption.

---

# 2.11 WebSocket

## Fonctionnalités

```rust
#[gateway]
pub struct ChatGateway {}
```

### Support

* rooms,
* events,
* scaling,
* auth,
* pub/sub.

---

# 2.12 Microservices

## Objectif

Support natif microservices.

## Transports

* TCP,
* Redis,
* NATS,
* Kafka,
* RabbitMQ,
* gRPC.

## Exemple

```rust
#[message_pattern("user.created")]
async fn on_user_created(data: UserCreatedEvent) {}
```

---

# 2.13 CLI

## Objectif

Créer une CLI similaire à Nest CLI.

## Commandes

```bash
ruest new app
ruest generate module users
ruest generate controller users
ruest generate service users
ruest generate resource users
ruest build
ruest start
ruest test
ruest migration:create
```

## Fonctionnalités

* scaffolding,
* génération de code,
* plugins,
* templates,
* monorepo support.

---

# 2.14 OpenAPI / Swagger

## Fonctionnalités

Documentation automatique.

```rust
#[api_operation(summary = "Get users")]
```

## Features

* Swagger UI,
* OpenAPI generation,
* DTO schemas,
* auth docs.

---

# 2.15 Configuration system

## Fonctionnalités

### Variables environnement

```rust
Config::get("DATABASE_URL")
```

### Support

* .env,
* YAML,
* TOML,
* JSON,
* secrets manager.

---

# 2.16 Logging & Telemetry

## Fonctionnalités

* structured logging,
* tracing,
* metrics,
* distributed tracing,
* OpenTelemetry,
* Prometheus.

---

# 2.17 Testing

## Fonctionnalités

### Unit tests

### Integration tests

### Mocking

### Test application container

Exemple :

```rust
let app = TestFactory::create(AppModule);
```

---

# 2.18 Error handling

## Fonctionnalités

### Standard errors

```json
{
  "status": 400,
  "message": "Validation failed",
  "errors": []
}
```

### Features

* global exception handler,
* custom exceptions,
* HTTP exceptions,
* validation exceptions.

---

# 2.19 Performance requirements

## Objectifs

### Latence

* très faible latence,
* async natif.

### Scalabilité

* multi-core,
* async runtime,
* streaming optimisé.

### Optimisation mémoire

* zero-copy quand possible,
* memory safety.

---

# 2.20 Developer Experience

## Priorités

### Simplicité

Le framework doit masquer la complexité Rust.

### Productivité

Le développeur doit produire rapidement.

### Lisibilité

Architecture propre.

### Documentation

Documentation moderne complète.

---

# 2.21 Convention architecture

## Structure recommandée

```text
src/
├── app.module.rs
├── main.rs
├── modules/
│   ├── users/
│   │   ├── dto/
│   │   ├── entities/
│   │   ├── users.controller.rs
│   │   ├── users.service.rs
│   │   ├── users.repository.rs
│   │   └── users.module.rs
```

---

# 2.22 Runtime architecture

## Runtime

Utiliser Tokio.

## Serveur HTTP

Utiliser Hyper/Axum.

## Injection container

Créer un runtime container global.

## Reflection system

Créer un système metadata avec macros.

---

# 2.23 Plugin system

## Objectif

Permettre des extensions.

## Exemple

```rust
app.use_plugin(AuthPlugin)
```

## Plugins possibles

* auth,
* graphql,
* cache,
* websocket,
* telemetry.

---

# 2.24 GraphQL support

## Fonctionnalités

* schema first,
* code first,
* subscriptions,
* federation.

---

# 2.25 Cache system

## Fonctionnalités

* Redis,
* in-memory,
* distributed cache.

## Exemple

```rust
#[cache(ttl = 60)]
```

---

# 2.26 Queue system

## Fonctionnalités

* background jobs,
* retries,
* delayed jobs,
* distributed queues.

---

# 2.27 Architecture technique recommandée

## Crates recommandées

| Domaine       | Crate             |
| ------------- | ----------------- |
| Runtime       | Tokio             |
| HTTP          | Hyper             |
| Router        | Axum              |
| Serialization | Serde             |
| Validation    | Validator         |
| Async trait   | async-trait       |
| Logging       | tracing           |
| Config        | config            |
| ORM           | SeaORM / SQLx     |
| JWT           | jsonwebtoken      |
| WebSocket     | tokio-tungstenite |
| CLI           | clap              |
| OpenAPI       | utoipa            |

---

# 2.28 Architecture interne DI

## Concepts

### Provider registry

```rust
HashMap<TypeId, Arc<dyn Any>>
```

### Lifetime management

* singleton,
* transient,
* request scoped.

### Resolution system

* recursive resolution,
* async resolution.

---

# 2.29 AI instructions

# Objectif principal

L’IA doit créer un framework backend Rust complet inspiré de NestJS et Spring Boot.

---

# Contraintes importantes

## Architecture

Le framework doit être :

* modulaire,
* extensible,
* async-first,
* production-ready,
* strongly typed.

## Performance

Optimiser :

* allocation mémoire,
* async runtime,
* throughput,
* startup time.

## Developer Experience

Le framework doit être très simple à utiliser.

## Style de code

* clean architecture,
* SOLID,
* trait-oriented,
* modular crates,
* faible couplage.

---

# 2.30 Roadmap technique

# Phase 1

## MVP

Créer :

* router,
* controllers,
* DI,
* modules,
* middleware,
* config,
* validation.

---

# Phase 2

Créer :

* auth,
* ORM,
* websocket,
* swagger,
* testing.

---

# Phase 3

Créer :

* microservices,
* queues,
* cache,
* GraphQL,
* observabilité.

---

# 2.31 Non Functional Requirements

## Sécurité

* protection mémoire Rust,
* secure defaults,
* protection injection.

## Scalabilité

* horizontal scaling,
* distributed support.

## Maintenabilité

* architecture claire,
* documentation.

## Compatibilité

* Linux,
* macOS,
* Windows,
* Docker.

---

# 2.32 Deliverables attendus

L’IA doit produire :

1. architecture complète,
2. structure crates,
3. système macros,
4. système DI,
5. router,
6. HTTP server,
7. CLI,
8. exemples,
9. tests,
10. documentation.

---

# 2.33 Priorités MVP

## Haute priorité

* routing,
* controllers,
* modules,
* dependency injection,
* middleware,
* validation,
* config,
* CLI.

## Priorité moyenne

* ORM,
* auth,
* swagger,
* websocket.

## Priorité basse

* GraphQL,
* microservices,
* distributed tracing.

---

# 2.34 Inspirations techniques

## Inspirations principales

### NestJS

* modules,
* decorators,
* DI,
* CLI.

### Spring Boot

* auto configuration,
* enterprise architecture,
* annotations.

### FastAPI

* simplicité,
* validation,
* docs auto.

### ASP.NET Core

* middleware pipeline,
* dependency injection.

---

# 2.35 Résultat final attendu

Le résultat final doit être un framework Rust moderne permettant à un développeur de construire rapidement :

* APIs REST,
* WebSocket,
* microservices,
* applications temps réel,
* systèmes distribués,
* plateformes haute performance.

Le framework doit offrir une expérience développeur proche de NestJS tout en exploitant pleinement les performances et la sécurité de Rust.
