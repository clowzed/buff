use std::{
    fmt::Debug,
    path::{Path, PathBuf},
};

#[derive(serde::Deserialize, Clone)]
pub struct Configuration {
    database_url: String,
    redis_url: String,
    sqlx_logging: bool,
    port: u16,
    jwt_secret: String,
    status_expiration_seconds: u64,
    realm: String,
    upload_folder: PathBuf,
    jwt_ttl: i64,
}

impl Configuration {
    pub fn database_url(&self) -> &str {
        self.database_url.as_ref()
    }

    pub fn sqlx_logging(&self) -> bool {
        self.sqlx_logging
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub fn jwt_secret(&self) -> &str {
        self.jwt_secret.as_ref()
    }

    pub fn redis_url(&self) -> &str {
        self.redis_url.as_ref()
    }

    pub fn status_expiration_seconds(&self) -> u64 {
        self.status_expiration_seconds
    }

    pub fn realm(&self) -> &str {
        self.realm.as_ref()
    }

    pub fn upload_folder(&self) -> &PathBuf {
        &self.upload_folder
    }
    
    pub fn jwt_ttl(&self) -> i64 {
        self.jwt_ttl
    }
}

pub trait ConfigurationReader {
    type Error;

    fn read<T>(path: Option<impl AsRef<Path> + Debug>) -> Result<T, Self::Error>
    where
        T: for<'de> serde::de::Deserialize<'de>;
}

#[derive(thiserror::Error, Debug)]
pub enum ConfigurationError {
    #[error(transparent)]
    DotenvyError(#[from] dotenvy::Error),

    #[error(transparent)]
    EnvyError(#[from] envy::Error),

    #[error(
        "Configuration file was not provided! \\
        Providing None as path is supported  \\ 
        only with EnvConfigurationReader"
    )]
    MissingConfigurationFile,

    #[error(transparent)]
    TOMLError(#[from] toml::de::Error),

    #[error(transparent)]
    IOError(#[from] std::io::Error),

    #[error(transparent)]
    JSONErrors(#[from] serde_json::Error),
}

#[derive(Debug)]
pub struct EnvConfigurationReader;

impl ConfigurationReader for EnvConfigurationReader {
    type Error = ConfigurationError;

    #[tracing::instrument]
    fn read<T>(path: Option<impl AsRef<Path> + Debug>) -> Result<T, Self::Error>
    where
        T: for<'de> serde::de::Deserialize<'de>,
    {
        tracing::trace!("Reading configuration with {:?}", Self);
        Ok(envy::from_env::<T>()?)
    }
}

#[derive(Debug)]
pub struct TOMLConfigurationReader;

impl ConfigurationReader for TOMLConfigurationReader {
    type Error = ConfigurationError;

    #[tracing::instrument]
    fn read<T>(path: Option<impl AsRef<Path> + Debug>) -> Result<T, Self::Error>
    where
        T: for<'de> serde::de::Deserialize<'de>,
    {
        tracing::trace!("Reading configuration with {:?}", Self);

        if path.is_none() {
            return Err(ConfigurationError::MissingConfigurationFile);
        }

        // Checked
        let config_path = path.unwrap();

        tracing::debug!(
            "Reading configuration from file with path {:?}",
            config_path
        );

        Ok(toml::from_str::<T>(&std::fs::read_to_string(
            &config_path,
        )?)?)
    }
}

#[derive(Debug)]
pub struct JSONConfigurationReader;

impl ConfigurationReader for JSONConfigurationReader {
    type Error = ConfigurationError;

    #[tracing::instrument]
    fn read<T>(path: Option<impl AsRef<Path> + Debug>) -> Result<T, Self::Error>
    where
        T: for<'de> serde::de::Deserialize<'de>,
    {
        tracing::trace!("Reading configuration with {:?}", Self);

        if path.is_none() {
            return Err(ConfigurationError::MissingConfigurationFile);
        }

        let config_path = path.unwrap();

        tracing::debug!(
            "Reading configuration from file with path {:?}",
            config_path
        );

        Ok(serde_json::from_str::<T>(&std::fs::read_to_string(
            &config_path,
        )?)?)
    }
}
