use std::sync::Arc;

use crate::services::auth::{JwtCheckParams, Service as AuthService};
use crate::{errors::AppError, state::AppState};
use axum::async_trait;
use axum::extract::{FromRef, FromRequestParts};
use axum::http::request::Parts;
use entity::user::{Entity as UserEntity, Model as UserModel};
use sea_orm::EntityTrait;

pub struct AuthJWT(pub UserModel);

#[async_trait]
impl<S> FromRequestParts<S> for AuthJWT
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
            .get("Authorization")
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

        match UserEntity::find_by_id(claims.sub)
            .one(app_state.database_connection())
            .await
        {
            Ok(Some(user)) => Ok(Self(user)),
            Ok(None) => Err(AppError::Unauthorized),
            Err(cause) => Err(AppError::InternalServerError(Box::new(cause))),
        }
    }
}
