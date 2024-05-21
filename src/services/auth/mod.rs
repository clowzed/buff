use crate::errors::AppError;
use argon2::{password_hash::SaltString, Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use chrono::{Duration, Utc};
use entity::admin::{
    ActiveModel as AdminActiveModel, Column as AdminColumn, Entity as AdminEntity,
    Model as AdminModel,
};
use jsonwebtoken::{DecodingKey, Validation};
use rand_core::OsRng;
use sea_orm::{prelude::*, ColumnTrait, ConnectionTrait, Set, TransactionTrait};
use std::fmt::Debug;
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
    #[error(transparent)]
    PasswordHashError(#[from] argon2::password_hash::Error),
}

impl From<ServiceError> for AppError {
    fn from(value: ServiceError) -> Self {
        match value {
            ServiceError::JWTError(cause) => AppError::JwtError(Box::new(cause)),
            ServiceError::Unauthorized => AppError::Unauthorized,
            ServiceError::DbErr(cause) => AppError::InternalServerError(Box::new(cause)),
            ServiceError::PasswordHashError(cause) => {
                AppError::InternalServerError(Box::new(cause))
            }
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
    pub ttl: i64,
}
pub struct GenerateAdminJwtParameters<'a> {
    pub admin_id: i64,
    pub secret: &'a str,
    pub ttl: i64,
}

pub struct ResetPasswordParameters<'a> {
    pub moderator_id: i64,
    pub old_password: &'a str,
    pub new_password: &'a str,
}

impl Debug for ResetPasswordParameters<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ResetPasswordParameters")
            .field("moderator_id", &self.moderator_id)
            .finish()
    }
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
            exp: (Utc::now() + Duration::minutes(60)).timestamp() as u64,
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
            exp: (Utc::now() + Duration::minutes(60)).timestamp() as u64,
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

    #[tracing::instrument(skip(connection))]
    pub async fn reset_password<T>(
        parameters: ResetPasswordParameters<'_>,
        connection: &T,
    ) -> Result<(), ServiceError>
    where
        T: ConnectionTrait + TransactionTrait,
    {
        let admin = match AdminEntity::find_by_id(parameters.moderator_id)
            .one(connection)
            .await?
        {
            Some(admin) => Ok(admin),
            None => Err(ServiceError::Unauthorized),
        }?;

        match PasswordHash::new(&admin.password) {
            Ok(parsed_hash) => match Argon2::default()
                .verify_password(parameters.old_password.as_bytes(), &parsed_hash)
            {
                Ok(()) => {
                    let salt = SaltString::generate(&mut OsRng);

                    let hashed_password = Argon2::default()
                        .hash_password(parameters.new_password.as_bytes(), &salt)?
                        .to_string();

                    let mut admin_to_be_updated: AdminActiveModel = admin.into();

                    admin_to_be_updated.password = Set(hashed_password);

                    admin_to_be_updated.update(connection).await?;

                    Ok(())
                }
                Err(_) => Err(ServiceError::Unauthorized),
            },

            Err(_) => Err(ServiceError::Unauthorized),
        }
    }
}
