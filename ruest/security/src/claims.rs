use ruest_http::AppError;
use serde::{Deserialize, Serialize};

/// Claims JWT standard RUEST (`sub`, `roles`, `exp`, `iat`).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuestClaims {
    pub sub: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub roles: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iss: Option<String>,
    pub exp: i64,
    pub iat: i64,
}

impl RuestClaims {
    pub fn new(
        sub: impl Into<String>,
        roles: Vec<String>,
        iss: Option<String>,
        exp: i64,
        iat: i64,
    ) -> Self {
        Self {
            sub: sub.into(),
            roles,
            iss,
            exp,
            iat,
        }
    }

    pub fn has_role(&self, role: &str) -> bool {
        self.roles.iter().any(|r| r == role)
    }

    pub fn has_any_role(&self, roles: &[&str]) -> bool {
        roles.iter().any(|r| self.has_role(r))
    }

    /// Vérifie les rôles ; renvoie `403 Forbidden` si aucun rôle ne correspond.
    pub fn require_roles(&self, roles: &[&str]) -> Result<(), AppError> {
        if roles.is_empty() || self.has_any_role(roles) {
            Ok(())
        } else {
            Err(AppError::forbidden(format!(
                "missing required role (need one of: {})",
                roles.join(", ")
            )))
        }
    }
}
