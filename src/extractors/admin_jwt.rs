use std::sync::Arc;

use crate::services::auth::{JwtCheckParams, Service as AuthService};
use crate::{errors::AppError, state::AppState};
use axum::async_trait;
use axum::extract::{FromRef, FromRequestParts};
use axum::http::request::Parts;
use entity::admin::Entity as AdminEntity;
use entity::admin::Model as AdminModel;
use entity::sea_orm_active_enums::Role;
use sea_orm::prelude::*;

pub struct AdminAuthJWT(pub AdminModel);

#[async_trait]
impl<S> FromRequestParts<S> for AdminAuthJWT
where
    Arc<AppState>: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = AppError;

    #[tracing::instrument(skip(parts, state))]
    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let app_state = Arc::from_ref(state);

        let auth_header_value = parts
            .headers
            .get("X-AM-Authorization") // Admin-Moderator auth in another header
            .ok_or(AppError::AuthorizationHeaderMissing)?
            .to_str()
            .map_err(|_| AppError::AuthorizationHeaderBadChars)?;

        let token = match auth_header_value.split_once(' ') {
            Some(("Bearer", contents)) => Ok(contents.to_string()),
            _ => Err(AppError::AuthorizationHeaderBadSchema),
        }?;

        let params = JwtCheckParams {
            token,
            secret: app_state.configuration().jwt_secret(),
        };

        let claims = match AuthService::check(params) {
            Ok(claims) => Ok(claims),
            Err(cause) => Err(AppError::JwtError(Box::new(cause))),
        }?;

        match AdminEntity::find_by_id(claims.sub)
            .one(app_state.database_connection())
            .await
        {
            Ok(None) => Err(AppError::Unauthorized),
            Ok(Some(admin)) if admin.role == Role::Moderator => Err(AppError::Forbidden),
            Ok(Some(admin)) => Ok(Self(admin)),
            Err(cause) => Err(AppError::InternalServerError(Box::new(cause))),
        }
    }
}

pub struct ModeratorAuthJWT(pub AdminModel);

#[async_trait]
impl<S> FromRequestParts<S> for ModeratorAuthJWT
where
    Arc<AppState>: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = AppError;

    #[tracing::instrument(skip(parts, state))]
    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let app_state = Arc::from_ref(state);

        let auth_header_value = parts
            .headers
            .get("X-AM-Authorization")
            // Admin-Moderator auth token sits in another header from users
            .ok_or(AppError::AuthorizationHeaderMissing)?
            .to_str()
            .map_err(|_| AppError::AuthorizationHeaderBadChars)?;

        let token = match auth_header_value.split_once(' ') {
            Some(("Bearer", contents)) => Ok(contents.to_string()),
            _ => Err(AppError::AuthorizationHeaderBadSchema),
        }?;

        let params = JwtCheckParams {
            token,
            secret: app_state.configuration().jwt_secret(),
        };

        let claims = match AuthService::check(params) {
            Ok(claims) => Ok(claims),
            Err(cause) => Err(Into::<AppError>::into(cause)),
        }?;

        match AdminEntity::find_by_id(claims.sub)
            .one(app_state.database_connection())
            .await
        {
            Ok(Some(admin)) => Ok(Self(admin)),
            Ok(None) => Err(AppError::Unauthorized),
            Err(cause) => Err(AppError::InternalServerError(Box::new(cause))),
        }
    }
}
