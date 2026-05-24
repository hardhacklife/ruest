# Principes du framework RUEST

RUEST est conçu autour de **piliers explicites**. Chaque pilier correspond à du code réel (crates, macros, tests) — pas à des slogans.

```
┌─────────────────────────────────────────────────────────────┐
│  App (votre code)                                           │
│  modules/ · controllers · services · schema.ruest           │
├─────────────────────────────────────────────────────────────┤
│  Modulaire     │ #[module] + imports · crates séparés       │
│  Typé          │ DI get::<T>() · AppResult · macros        │
│  Sécurisé      │ JWT · guards · validation · SQL paramétré   │
│  Fonctionnel   │ bootstrap → routes → HTTP (tests E2E)    │
└─────────────────────────────────────────────────────────────┘
```

---

## 1. Modulaire

**Objectif :** composer l’application par blocs indépendants, comme NestJS, sans tout lister dans un seul fichier.

| Mécanisme | Usage |
|-----------|--------|
| `#[module(controllers, providers)]` | Module métier (Users, Orders, Auth) |
| `#[module(imports = [A, B])]` | Module racine qui assemble les domaines |
| Crates workspace | `core`, `http`, `di`, `security`, `ruest-db/*` — une responsabilité par crate |
| `ruest g resource` | Génère dto, entity, repository, service, controller, **module** |

```rust
#[module(imports = [AuthModule, CustomersModule, OrdersModule])]
pub struct AppModule;
```

**Règle :** un domaine = un dossier `src/modules/<nom>/` + un `*_module.rs`. Le `AppModule` ne connaît que les **modules**, pas chaque controller.

Voir aussi : [DX.md](./DX.md#module-racine-imports).

---

## 2. Typé (type-safe)

**Objectif :** le compilateur Rust attrape les erreurs avant la production — pas de `dyn` par route, pas de résolution par nom de string.

| Mécanisme | Garantie |
|-----------|----------|
| `Inject<T>` | Dépendances résolues via `get::<T>()` monomorphisé |
| `#[service]` | Enregistrement `register_provider` statique |
| `AppResult<T>` | Erreurs HTTP typées (`AppError`), pas de `Result<Result<…>>` |
| `#[routes]` + `#[get]` | Handlers et chemins générés au **compile-time** |
| RuestDB | Client généré : `client.customer.find_many().await?` |
| `ValidatedJson<T>` | DTOs validés avec `validator` |

```rust
// Erreur de compilation si UserService n’est pas dans le module :
service: Inject<UserService>,
```

Messages DI explicites :

```text
[Ruest DI] Service `my_app::UserService` is not registered.
Did you forget to add it to your module?
```

---

## 3. Sécurisé

**Objectif :** sécurité par défaut raisonnable — auth, validation, pas d’injection SQL dans le stack officiel.

| Couche | Outil |
|--------|--------|
| HTTP / Auth | `SecurityConfig`, `JwtService`, `with_jwt_auth()` |
| Autorisation | `#[guard]`, `JwtGuard`, `RolesGuard`, `AuthUser` |
| Erreurs | `ruest_err!(Unauthorized, …)`, `Forbidden` |
| Entrées | `Validate` + `ValidatedJson` sur les DTOs |
| Données | RuestDB + SQLx — **requêtes paramétrées** uniquement |
| Transport | Axum + Tower (CORS, trace) via `serve()` |

```rust
bootstrap_app(AppModule)?
    .with_jwt_auth(SecurityConfig::from_env()?)?
    .listen()
    .await?;
```

**Production :** ne jamais utiliser `SecurityConfig::dev()` ; définir `RUEST_JWT_SECRET` et des routes publiques minimales.

Détails : [SECURITY.md](./SECURITY.md).

---

## 4. Fonctionnel

**Objectif :** chaque brique livrée **marche de bout en bout** — pas d’API « placeholder » sans tests.

| Brique | Preuve |
|--------|--------|
| Bootstrap + routes | `cargo test -p ruest` · `framework_integration` |
| DI + modules | `ruest-core/tests/module_configure` |
| Sécurité JWT | `ruest-security/tests/jwt_test` |
| RuestDB | `ruest-db-parser`, `ruest-db-codegen`, `examples/ruest-db-demo` |
| Apps exemples | `basic-api` :3000, `shop-api` :3001 (+ auth JWT) |

Flux standard qui doit toujours fonctionner :

```bash
cargo build
cargo test -p ruest -p ruest-security -p ruest-db-parser
cargo run -p shop-api
```

---

## 5. Autres piliers (déjà en place ou en cours)

| Pilier | État | Référence |
|--------|------|-----------|
| **Performant** (compile-time) | ✅ | [ARCHITECTURE.md](../ARCHITECTURE.md) |
| **DX** (prelude, CLI, messages) | ✅ | [DX.md](./DX.md) |
| **Testable** | ✅ | `ruest::testing::TestFactory` |
| **Observable** | Partiel | `tracing`, `logger::init()` |
| **Persistant** | ✅ MVP | [RUESTDB.md](./RUESTDB.md) |
| **Documenté** | ✅ | README, ARCHITECTURE, docs/ |

---

## Checklist projet (avant prod)

- [ ] `AppModule` compose uniquement des sous-modules (`imports`)
- [ ] Services enregistrés via `#[module(providers = …)]`
- [ ] `SecurityConfig::from_env()` (pas `dev()`)
- [ ] DTOs avec `Validate` sur les entrées utilisateur
- [ ] `DATABASE_URL` + `ruest migrate deploy` si RuestDB
- [ ] `cargo test` vert sur le workspace

---

## Évolution (sans casser les piliers)

Les ajouts futurs (OpenAPI, extracteurs dans `#[routes]`, `ruest db pull`) doivent :

1. Rester **optionnels** (feature flag ou crate séparé si lourd).
2. Conserver la **composition par modules**.
3. Préserver le **typage compile-time** autant que possible.
4. Ne pas affaiblir JWT / validation / SQL paramétré par défaut.
