use std::fmt::Debug;

use crate::errors::AppError;
use argon2::Argon2;
use argon2::PasswordHash;
use argon2::PasswordVerifier;
use chrono::Duration;
use chrono::Utc;

use entity::admin::Column as AdminColumn;
use entity::admin::Entity as AdminEntity;
use entity::admin::Model as AdminModel;
use jsonwebtoken::DecodingKey;
use jsonwebtoken::Validation;
use sea_orm::prelude::*;
use sea_orm::ColumnTrait;
use sea_orm::ConnectionTrait;
use sea_orm::TransactionTrait;
use thiserror::Error;
pub type Jwt = String;

#[derive(Error, Debug)]
pub enum ServiceError {
    #[error(transparent)]
    JWTError(#[from] jsonwebtoken::errors::Error),
    #[error("Bad login or password")]
    Unauthorized,
    #[error(transparent)]
    DbErr(#[from] sea_orm::DbErr),
}

impl From<ServiceError> for AppError {
    fn from(value: ServiceError) -> Self {
        match value {
            ServiceError::JWTError(cause) => AppError::JwtError(Box::new(cause)),
            ServiceError::Unauthorized => AppError::Unauthorized,
            ServiceError::DbErr(cause) => AppError::InternalServerError(Box::new(cause)),
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct TokenClaims {
    pub sub: i64,
    pub iat: u64,
    pub exp: u64,
}

pub struct Service;

pub struct JwtCheckParams<'a> {
    pub token: Jwt,
    pub secret: &'a str,
}

pub struct AdminCredentials {
    pub login: String,
    pub password: String,
}

//? This is manual for hiding password in logs
impl Debug for AdminCredentials {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AdminCredentials")
            .field("login", &self.login)
            .finish()
    }
}

pub struct GenerateUserJwtParameters<'a> {
    pub steam_id: i64,
    pub secret: &'a str,
}

pub struct GenerateAdminJwtParameters<'a> {
    pub admin_id: i64,
    pub secret: &'a str,
}

impl Service {
    #[tracing::instrument(skip(jwt_params))]
    pub fn check(jwt_params: JwtCheckParams<'_>) -> Result<TokenClaims, ServiceError> {
        let validation = Validation::default();

        let decoded = jsonwebtoken::decode::<TokenClaims>(
            &jwt_params.token,
            &DecodingKey::from_secret(jwt_params.secret.as_bytes()),
            &validation,
        )?;

        Ok(decoded.claims)
    }

    #[tracing::instrument(skip(parameters))]
    pub fn user_jwt(parameters: GenerateUserJwtParameters<'_>) -> Result<Jwt, ServiceError> {
        let claims: TokenClaims = TokenClaims {
            sub: parameters.steam_id,
            exp: (Utc::now() + Duration::minutes(10)).timestamp() as u64,
            iat: Utc::now().timestamp() as u64,
        };

        Ok(jsonwebtoken::encode(
            &jsonwebtoken::Header::default(),
            &claims,
            &jsonwebtoken::EncodingKey::from_secret(parameters.secret.as_bytes()),
        )?)
    }

    #[tracing::instrument(skip(parameters))]
    pub fn admin_jwt(parameters: GenerateAdminJwtParameters<'_>) -> Result<Jwt, ServiceError> {
        let claims: TokenClaims = TokenClaims {
            sub: parameters.admin_id,
            exp: (Utc::now() + Duration::minutes(10)).timestamp() as u64,
            iat: Utc::now().timestamp() as u64,
        };

        Ok(jsonwebtoken::encode(
            &jsonwebtoken::Header::default(),
            &claims,
            &jsonwebtoken::EncodingKey::from_secret(parameters.secret.as_bytes()),
        )?)
    }

    #[tracing::instrument(skip(connection))]
    pub async fn login_admin<T>(
        credentials: AdminCredentials,
        connection: &T,
    ) -> Result<AdminModel, ServiceError>
    where
        T: ConnectionTrait + TransactionTrait,
    {
        let admin = match AdminEntity::find()
            .filter(AdminColumn::Login.eq(&credentials.login))
            .one(connection)
            .await?
        {
            Some(admin) => Ok(admin),
            None => Err(ServiceError::Unauthorized),
        }?;

        match PasswordHash::new(&admin.password) {
            Ok(parsed_hash) => match Argon2::default()
                .verify_password(credentials.password.as_bytes(), &parsed_hash)
            {
                Ok(()) => Ok(admin),
                Err(_) => Err(ServiceError::Unauthorized),
            },

            Err(_) => Err(ServiceError::Unauthorized),
        }
    }
}
