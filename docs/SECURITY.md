# Sécurité RUEST

## Vue d'ensemble

Le crate `ruest-security` fournit :

| Composant | Rôle |
|-----------|------|
| [`SecurityConfig`] | Secret JWT, expiration, routes publiques |
| [`JwtService`] | Signature / vérification des tokens |
| [`AppBuilder::with_jwt_auth`] | DI + middleware HTTP |
| [`AuthUser`] | Extracteur Axum (handlers manuels ou futurs `#[routes]`) |
| [`JwtGuard`] / [`RolesGuard`] / `#[guard]` | Gardes style NestJS |
| `ruest_err!(Unauthorized, …)` | Erreurs HTTP lisibles |

## Activation

```rust
use ruest::prelude::*;

#[tokio::main]
async fn main() -> Result<(), CoreError> {
    ruest::logger::init();

    let security = SecurityConfig::dev(); // ou SecurityConfig::from_env()?

    bootstrap_app(AppModule)?
        .with_jwt_auth(security)?
        .port(3000)
        .listen()
        .await
}
```

### Variables d'environnement

| Variable | Description |
|----------|-------------|
| `RUEST_JWT_SECRET` | Secret HMAC (obligatoire en prod) |
| `RUEST_JWT_EXPIRES_IN_SECS` | Durée de vie (défaut `3600`) |
| `RUEST_JWT_ISSUER` | Claim `iss` optionnel |
| `RUEST_PUBLIC_ROUTES` | Chemins publics supplémentaires (séparés par `,`) |

Routes publiques par défaut : `/health`, `/auth/login`, `/auth/register`.

## Login (exemple shop-api)

```http
GET /auth/login
```

Réponse :

```json
{ "access_token": "<jwt>", "token_type": "Bearer", "expires_in": 3600 }
```

Requêtes protégées :

```http
GET /customers/
Authorization: Bearer <jwt>
```

## Gardes

```rust
#[guard]
pub struct AuthenticatedGuard;

#[guard(roles = ["admin"])]
pub struct AdminGuard;
```

Dans un handler (hors macro `#[routes]` pour l'instant) :

```rust
async fn admin_only(user: AuthUser) -> AppResult<Json<()>> {
    user.require_roles(&["admin"])?;
    Ok(Json(()))
}
```

## RBAC

```rust
if claims.has_role("admin") { /* … */ }
claims.require_roles(&["admin", "manager"])?;
```

## Prochaines étapes

- `#[get("/path", guards = [JwtGuard])]` sur les macros de routes
- OAuth2, API keys, rate limiting
- `ruest add jwt` dans le CLI
