use crate::claims::RuestClaims;

/// Contexte d'authentification injecté dans les extensions de requête Axum.
#[derive(Debug, Clone)]
pub struct AuthContext {
    pub claims: RuestClaims,
    pub token: String,
}

impl AuthContext {
    pub fn claims(&self) -> &RuestClaims {
        &self.claims
    }
}
