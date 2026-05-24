use async_trait::async_trait;

use crate::http::AppError;

use super::context::AuthContext;

/// Garde d'autorisation (style NestJS `CanActivate`).
#[async_trait]
pub trait Guard: Send + Sync {
    async fn can_activate(&self, ctx: Option<&AuthContext>) -> Result<(), AppError>;
}

/// Exige un JWT valide (contexte présent).
#[derive(Debug, Default, Clone, Copy)]
pub struct JwtGuard;

#[async_trait]
impl Guard for JwtGuard {
    async fn can_activate(&self, ctx: Option<&AuthContext>) -> Result<(), AppError> {
        if ctx.is_some() {
            Ok(())
        } else {
            Err(AppError::unauthorized(
                "authentication required — provide Authorization: Bearer <token>",
            ))
        }
    }
}

/// Exige au moins un des rôles listés.
#[derive(Debug, Clone)]
pub struct RolesGuard {
    roles: Vec<String>,
}

impl RolesGuard {
    pub fn new(roles: impl IntoIterator<Item = impl AsRef<str>>) -> Self {
        Self {
            roles: roles.into_iter().map(|r| r.as_ref().to_string()).collect(),
        }
    }

    pub fn require(roles: &[&str]) -> Self {
        Self::new(roles.iter().copied())
    }
}

#[async_trait]
impl Guard for RolesGuard {
    async fn can_activate(&self, ctx: Option<&AuthContext>) -> Result<(), AppError> {
        let Some(ctx) = ctx else {
            return Err(AppError::unauthorized("authentication required"));
        };
        let required: Vec<&str> = self.roles.iter().map(String::as_str).collect();
        ctx.claims.require_roles(&required)
    }
}

/// Vérifie une liste de gardes en séquence.
pub async fn run_guards(guards: &[&dyn Guard], ctx: Option<&AuthContext>) -> Result<(), AppError> {
    for guard in guards {
        guard.can_activate(ctx).await?;
    }
    Ok(())
}
