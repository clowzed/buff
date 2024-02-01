use axum::extract::Query;
use axum::routing::{get, patch};
use axum::{extract::State, http::StatusCode, response::IntoResponse};
use axum::{Json, Router};
use sea_orm::prelude::Decimal;
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

use crate::services::users::Service as UsersService;
use crate::{errors::AppError, extractors::user_jwt::AuthJWT, state::AppState};
use entity::user::Model as UserModel;
use std::sync::Arc;

#[derive(serde::Serialize, serde::Deserialize, ToSchema, Debug)]
pub struct User {
    pub steam_id: i64,
    pub trade_url: Option<String>,
    pub email: Option<String>,
}

impl From<UserModel> for User {
    fn from(value: UserModel) -> Self {
        Self {
            steam_id: value.steam_id,
            trade_url: value.trade_url,
            email: value.email,
        }
    }
}

#[utoipa::path(
    get,
    path = "/api/user",
    responses(
        (status = 200, description = "User was successfully retrieved",    body = User),
        (status = 401, description = "Unauthorized",                       body = Details),
        (status = 500, description = "Internal Server Error",              body = Details),
    ),
    security(
        ("jwt_user" = [])
    )
)]
#[tracing::instrument(skip(app_state))]
pub async fn get_user(
    State(app_state): State<Arc<AppState>>,
    AuthJWT(user): AuthJWT,
) -> axum::response::Response {
    match UsersService::get_by_steam_id(user.steam_id, app_state.database_connection()).await {
        Ok(Some(user)) => Json(Into::<User>::into(user)).into_response(),
        Ok(None) => AppError::UserWasNotFound(user.steam_id).into_response(),
        Err(cause) => Into::<AppError>::into(cause).into_response(),
    }
}

#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct TradeUrlForm {
    url: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct EmailForm {
    email: Option<String>,
}

#[utoipa::path(
    patch,
    path = "/api/user/email",
    responses(
        (status = 200, description = "Email was successfully changed"),
        (status = 401, description = "Unauthorized",                       body = Details),
        (status = 500, description = "Internal Server Error",              body = Details),
    ),
    request_body = EmailForm,
    security(
        ("jwt_user" = [])
    )
)]
#[tracing::instrument(skip(app_state))]
pub async fn set_email(
    State(app_state): State<Arc<AppState>>,
    AuthJWT(user): AuthJWT,
    Json(payload): Json<EmailForm>,
) -> axum::response::Response {
    match UsersService::set_email(
        user.steam_id,
        payload.email,
        app_state.database_connection(),
    )
    .await
    {
        Ok(()) => StatusCode::NO_CONTENT.into_response(),
        Err(cause) => Into::<AppError>::into(cause).into_response(),
    }
}

#[utoipa::path(
    patch,
    path = "/api/user/trade-url",
    responses(
        (status = 200, description = "Trade url was successfully changed"),
        (status = 401, description = "Unauthorized",                       body = Details),
        (status = 500, description = "Internal Server Error",              body = Details),
    ),
    request_body = TradeUrlForm,
    security(
        ("jwt_user" = [])
    )
)]
#[tracing::instrument(skip(app_state))]
pub async fn set_trade_url(
    State(app_state): State<Arc<AppState>>,
    AuthJWT(user): AuthJWT,
    Json(payload): Json<TradeUrlForm>,
) -> axum::response::Response {
    match UsersService::set_trade_url(user.steam_id, payload.url, app_state.database_connection())
        .await
    {
        Ok(()) => StatusCode::NO_CONTENT.into_response(),
        Err(cause) => Into::<AppError>::into(cause).into_response(),
    }
}

#[derive(serde::Serialize, serde::Deserialize, ToSchema)]
pub struct TopUser {
    pub steam_id: i64,
    #[schema(value_type = String)]
    pub amount: Decimal,
}

impl From<crate::services::users::TopUser> for TopUser {
    fn from(value: crate::services::users::TopUser) -> Self {
        Self {
            steam_id: value.steam_id,
            amount: value.amount,
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize, ToSchema, Debug, IntoParams)]
pub struct Bounds {
    limit: u64,
    offset: u64,
}

#[utoipa::path(
    get,
    path = "/api/user/top",
    responses(
        (status = 200, description = "Top users were successfully retrieved", body = [TopUser]),
        (status = 500, description = "Internal Server Error",              body = Details),
    ),
    params(
        Bounds

    )
)]
#[tracing::instrument(skip(app_state))]
pub async fn get_top(
    State(app_state): State<Arc<AppState>>,
    Query(payload): Query<Bounds>,
) -> axum::response::Response {
    match UsersService::top(
        payload.limit,
        payload.offset,
        app_state.database_connection(),
    )
    .await
    {
        Ok(users) => Json(
            users
                .into_iter()
                .map(Into::<TopUser>::into)
                .collect::<Vec<_>>(),
        )
        .into_response(),
        Err(cause) => Into::<AppError>::into(cause).into_response(),
    }
}

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(get_user))
        .route("/trade-url", patch(set_trade_url))
        .route("/email", patch(set_email))
        .route("/top", get(get_top))
}
