use crate::claims::ForgeClaims;

/// Contexte d'authentification injecté dans les extensions de requête Axum.
#[derive(Debug, Clone)]
pub struct AuthContext {
    pub claims: ForgeClaims,
    pub token: String,
}

impl AuthContext {
    pub fn claims(&self) -> &ForgeClaims {
        &self.claims
    }
}
