use std::sync::Arc;

use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use crate::di::Container;

use super::claims::RuestClaims;
use super::config::SecurityConfig;
use super::SecurityError;

/// Service JWT : signature et vérification des tokens (enregistrable en DI).
#[derive(Clone)]
pub struct JwtService {
    encoding: EncodingKey,
    decoding: DecodingKey,
    validation: Validation,
    expires_in_secs: u64,
    issuer: Option<String>,
}

impl JwtService {
    pub fn new(config: &SecurityConfig) -> Result<Self, SecurityError> {
        if config.jwt_secret.len() < 16 {
            return Err(SecurityError::Config(
                "jwt_secret must be at least 16 characters".into(),
            ));
        }

        let mut validation = Validation::default();
        validation.validate_exp = true;
        if let Some(ref iss) = config.jwt_issuer {
            validation.set_issuer(&[iss.as_str()]);
        }

        Ok(Self {
            encoding: EncodingKey::from_secret(config.jwt_secret.as_bytes()),
            decoding: DecodingKey::from_secret(config.jwt_secret.as_bytes()),
            validation,
            expires_in_secs: config.jwt_expires_in_secs,
            issuer: config.jwt_issuer.clone(),
        })
    }

    /// Enregistre le service comme singleton (pattern `#[service]` / module).
    pub fn register_provider(container: &Container, config: SecurityConfig) -> Result<(), SecurityError> {
        let service = Arc::new(Self::new(&config)?);
        container.register_singleton(service);
        Ok(())
    }

    /// Enregistrement pratique pour le développement (`SecurityConfig::dev()`).
    pub fn register_dev_provider(container: &Container) {
        let config = SecurityConfig::dev();
        let service = Arc::new(Self::new(&config).expect("dev JWT config"));
        container.register_singleton(service);
    }

    pub fn sign_subject(&self, sub: impl Into<String>, roles: Vec<String>) -> Result<String, SecurityError> {
        let now = Utc::now();
        let exp = now + Duration::seconds(self.expires_in_secs as i64);
        let claims = RuestClaims::new(
            sub,
            roles,
            self.issuer.clone(),
            exp.timestamp(),
            now.timestamp(),
        );
        self.sign(&claims)
    }

    pub fn sign(&self, claims: &RuestClaims) -> Result<String, SecurityError> {
        encode(&Header::default(), claims, &self.encoding).map_err(SecurityError::from)
    }

    pub fn verify(&self, token: &str) -> Result<RuestClaims, SecurityError> {
        let data = decode::<RuestClaims>(token, &self.decoding, &self.validation)
            .map_err(|_| SecurityError::InvalidToken)?;
        Ok(data.claims)
    }
}
