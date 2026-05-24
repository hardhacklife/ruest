use async_trait::async_trait;
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use ruest_http::AppError;

use crate::claims::RuestClaims;
use crate::context::AuthContext;

/// Utilisateur authentifié (extracteur Axum — nécessite le middleware JWT).
#[derive(Debug, Clone)]
pub struct AuthUser(pub RuestClaims);

impl AuthUser {
    pub fn claims(&self) -> &RuestClaims {
        &self.0
    }

    pub fn subject(&self) -> &str {
        &self.0.sub
    }

    pub fn require_roles(&self, roles: &[&str]) -> Result<(), AppError> {
        self.0.require_roles(roles)
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        parts
            .extensions
            .get::<AuthContext>()
            .map(|ctx| AuthUser(ctx.claims.clone()))
            .ok_or_else(|| {
                AppError::unauthorized(
                    "not authenticated — enable .with_jwt_auth() or use a public route",
                )
            })
    }
}
