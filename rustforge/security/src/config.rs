use std::env;

use crate::SecurityError;

/// Configuration JWT / sécurité (variables d'environnement ou builder).
#[derive(Debug, Clone)]
pub struct SecurityConfig {
    /// Secret HMAC (min. 32 caractères recommandé en production).
    pub jwt_secret: String,
    /// Durée de vie du token en secondes.
    pub jwt_expires_in_secs: u64,
    /// Émetteur optionnel (`iss` claim).
    pub jwt_issuer: Option<String>,
    /// Chemins HTTP publics (sans Bearer), ex. `/health`, `/auth/login`.
    pub public_routes: Vec<String>,
}

impl SecurityConfig {
    /// Valeurs de développement — **ne pas utiliser en production**.
    pub fn dev() -> Self {
        Self {
            jwt_secret: "rustforge-dev-secret-change-in-production!!".into(),
            jwt_expires_in_secs: 3600,
            jwt_issuer: Some("rustforge".into()),
            public_routes: vec![
                "/health".into(),
                "/auth/login".into(),
                "/auth/register".into(),
            ],
        }
    }

    pub fn builder() -> SecurityConfigBuilder {
        SecurityConfigBuilder::default()
    }

    /// Charge depuis l'environnement :
    /// `FORGE_JWT_SECRET`, `FORGE_JWT_EXPIRES_IN_SECS`, `FORGE_JWT_ISSUER`,
    /// `FORGE_PUBLIC_ROUTES` (séparés par des virgules).
    pub fn from_env() -> Result<Self, SecurityError> {
        let jwt_secret = env::var("FORGE_JWT_SECRET").map_err(|_| {
            SecurityError::Config(
                "FORGE_JWT_SECRET is not set (use SecurityConfig::dev() for local dev)".into(),
            )
        })?;

        let jwt_expires_in_secs = env::var("FORGE_JWT_EXPIRES_IN_SECS")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(3600);

        let jwt_issuer = env::var("FORGE_JWT_ISSUER").ok();

        let mut public_routes = vec![
            "/health".into(),
            "/auth/login".into(),
            "/auth/register".into(),
        ];
        if let Ok(extra) = env::var("FORGE_PUBLIC_ROUTES") {
            for path in extra.split(',').map(str::trim).filter(|s| !s.is_empty()) {
                public_routes.push(path.to_string());
            }
        }

        Ok(Self {
            jwt_secret,
            jwt_expires_in_secs,
            jwt_issuer,
            public_routes,
        })
    }

    pub fn is_public_route(&self, path: &str) -> bool {
        self.public_routes.iter().any(|p| path == p.as_str() || path.starts_with(&format!("{p}/")))
    }
}

#[derive(Debug, Default)]
pub struct SecurityConfigBuilder {
    jwt_secret: Option<String>,
    jwt_expires_in_secs: u64,
    jwt_issuer: Option<String>,
    public_routes: Vec<String>,
}

impl SecurityConfigBuilder {
    pub fn jwt_secret(mut self, secret: impl Into<String>) -> Self {
        self.jwt_secret = Some(secret.into());
        self
    }

    pub fn expires_in_secs(mut self, secs: u64) -> Self {
        self.jwt_expires_in_secs = secs;
        self
    }

    pub fn issuer(mut self, issuer: impl Into<String>) -> Self {
        self.jwt_issuer = Some(issuer.into());
        self
    }

    pub fn public_route(mut self, path: impl Into<String>) -> Self {
        self.public_routes.push(path.into());
        self
    }

    pub fn build(self) -> Result<SecurityConfig, SecurityError> {
        let jwt_secret = self.jwt_secret.ok_or_else(|| {
            SecurityError::Config("jwt_secret is required (call .jwt_secret(...))".into())
        })?;
        Ok(SecurityConfig {
            jwt_secret,
            jwt_expires_in_secs: if self.jwt_expires_in_secs == 0 {
                3600
            } else {
                self.jwt_expires_in_secs
            },
            jwt_issuer: self.jwt_issuer,
            public_routes: if self.public_routes.is_empty() {
                SecurityConfig::dev().public_routes
            } else {
                self.public_routes
            },
        })
    }
}
