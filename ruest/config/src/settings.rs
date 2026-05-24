use std::path::Path;

use serde::Deserialize;

use crate::ConfigError;

/// Global configuration accessor (env, .env, TOML, YAML, JSON).
#[derive(Debug, Clone)]
pub struct RuestConfig {
    inner: config::Config,
}

impl RuestConfig {
    /// Load configuration with optional file path.
    pub fn load(path: Option<&Path>) -> Result<Self, ConfigError> {
        let _ = dotenvy::dotenv();

        let mut builder = config::Config::builder().add_source(
            config::Environment::default()
                .separator("__")
                .try_parsing(true),
        );

        if let Some(path) = path {
            builder = builder.add_source(config::File::from(path).required(false));
        }

        let inner = builder.build()?;
        Ok(Self { inner })
    }

    /// Get a configuration value by key.
    pub fn get<T: for<'de> Deserialize<'de>>(&self, key: &str) -> Result<T, ConfigError> {
        self.inner
            .get(key)
            .map_err(|e| ConfigError::Message(e.to_string()))
    }

    /// Get optional value.
    pub fn get_optional<T: for<'de> Deserialize<'de>>(
        &self,
        key: &str,
    ) -> Result<Option<T>, ConfigError> {
        match self.get(key) {
            Ok(v) => Ok(Some(v)),
            Err(ConfigError::Message(_)) => Ok(None),
            Err(e) => Err(e),
        }
    }
}

/// Convenience static-style accessor when a global config is initialized.
pub fn get<T: for<'de> Deserialize<'de>>(key: &str) -> Result<T, ConfigError> {
    RuestConfig::load(None)?.get(key)
}
