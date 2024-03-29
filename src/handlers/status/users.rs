use crate::{errors::AppError, extractors::user_jwt::AuthJWT, state::AppState};
use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, patch},
    Form, Json,
};
use redis::AsyncCommands;
use std::sync::Arc;
use utoipa::{IntoParams, ToSchema};

#[derive(Debug, serde::Serialize, ToSchema)]
pub struct StatusResponse {
    pub statuses: Vec<UserStatus>,
}

#[derive(Debug, serde::Serialize, ToSchema)]
pub struct UserStatus {
    steam_id: String,
    is_online: bool,
}

#[utoipa::path(
    patch,
    path = "/api/status/user",
    responses(
        (status = 204, description = "Status was successfully refreshed"),
        (status = 400, description = "Bad request",                        body = Details),
        (status = 401, description = "Unauthorized",                       body = Details),
        (status = 500, description = "Internal Server Error",              body = Details),
    ),
    security(
        ("jwt_user" = [])
    )
)]
#[tracing::instrument(skip(app_state))]
pub async fn refresh_status(
    State(app_state): State<Arc<AppState>>,
    AuthJWT(user): AuthJWT,
) -> Response {
    let mut client = match app_state.redis_client().get_async_connection().await {
        Ok(connection) => connection,
        Err(cause) => {
            return AppError::InternalServerError(Box::new(cause)).into_response();
        }
    };

    let exp = app_state.configuration().status_expiration_seconds();

    match client.set_ex(user.steam_id, true, exp).await {
        Ok(()) => StatusCode::NO_CONTENT.into_response(),
        Err(cause) => AppError::InternalServerError(Box::new(cause)).into_response(),
    }
}

#[derive(serde::Serialize, serde::Deserialize, ToSchema, IntoParams)]
pub struct StatusRequest {
    ids: Vec<String>,
}

#[utoipa::path(
    get,
    path = "/api/status/user",
    request_body =     StatusRequest ,
    responses(
        (status = 500, description = "Internal server error",              body = Details),
        (status = 400, description = "Bad request",                        body = Details),
        (status = 200, description = "Status was successfully retrieved",  body = [StatusResponse])
    )
)]
#[tracing::instrument(skip(app_state, payload))]
pub async fn fetch_status(
    State(app_state): State<Arc<AppState>>,
    Form(payload): Form<StatusRequest>,
) -> Response {
    let mut client = match app_state.redis_client().get_async_connection().await {
        Ok(connection) => connection,
        Err(cause) => {
            return AppError::InternalServerError(Box::new(cause)).into_response();
        }
    };

    let ids: Vec<_> = payload
        .ids
        .into_iter()
        .flat_map(|id| id.parse::<i64>())
        .collect();

    match client
        .mget::<Vec<i64>, Vec<Option<bool>>>(ids.to_owned())
        .await
    {
        Ok(statuses) => Json(StatusResponse {
            statuses: statuses
                .into_iter()
                .zip(ids)
                .map(|status| UserStatus {
                    steam_id: status.1.to_string(),
                    is_online: status.0.unwrap_or(false),
                })
                .collect::<Vec<_>>(),
        })
        .into_response(),
        Err(cause) => AppError::InternalServerError(Box::new(cause)).into_response(),
    }
}

pub fn router() -> axum::Router<Arc<AppState>> {
    axum::Router::new()
        .route("/user", patch(refresh_status))
        .route("/user", get(fetch_status))
}
