use crate::{
    errors::AppError, extractors::admin_jwt::AdminAuthJWT,
    services::admin::blacklist::Service as BlacklistService, state::AppState,
};
use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use sea_orm::TransactionTrait;
use std::sync::Arc;
use utoipa::ToSchema;

#[utoipa::path(
    post,
    path = "/api/admin/blacklist",
    request_body = BlacklistUserRequest,
    responses(
        (status = 204, description = "User was successfully blacklisted"),
        (status = 400, description = "Bad request",                        body = Details),
        (status = 401, description = "Unauthorized",                       body = Details),
        (status = 409, description = "User has already been blacklisted",  body = Details),
        (status = 404, description = "User was not found",                 body = Details),
        (status = 500, description = "Internal Server Error",              body = Details),
    ),
    security(
        ("jwt_admin" = [])
    )
)]
#[tracing::instrument(skip(app_state))]
pub async fn blacklist_user(
    AdminAuthJWT(admin): AdminAuthJWT,
    State(app_state): State<Arc<AppState>>,
    Json(payload): Json<BlacklistUserRequest>,
) -> Response {
    let steam_id = match payload.steam_id.parse::<i64>() {
        Ok(id) => id,
        Err(error) => {
            return Into::<AppError>::into(error).into_response();
        }
    };
    match app_state.database_connection().begin().await {
        Ok(transaction) => match BlacklistService::blacklist_user(steam_id, &transaction).await {
            Ok(()) => {
                if let Err(cause) = transaction.commit().await {
                    return AppError::InternalServerError(Box::new(cause)).into_response();
                }
                StatusCode::NO_CONTENT.into_response()
            }
            Err(error) => Into::<AppError>::into(error).into_response(),
        },
        Err(cause) => AppError::InternalServerError(Box::new(cause)).into_response(),
    }
}

#[utoipa::path(
    delete,
    path = "/api/admin/blacklist",
    request_body = UnblacklistUserRequest,
    responses(
        (status = 204, description = "User was successfully unblacklisted"),
        (status = 400, description = "Bad request",                        body = Details),
        (status = 401, description = "Unauthorized",                       body = Details),
        (status = 409, description = "User is not blacklisted",            body = Details),
        (status = 404, description = "User was not found",                 body = Details),
        (status = 500, description = "Internal Server Error",              body = Details),
    ),
    security(
        ("jwt_admin" = [])
    )
)]
#[tracing::instrument(skip(app_state))]
pub async fn unblacklist_user(
    AdminAuthJWT(admin): AdminAuthJWT,
    State(app_state): State<Arc<AppState>>,
    Json(payload): Json<UnblacklistUserRequest>,
) -> Response {
    let steam_id = match payload.steam_id.parse::<i64>() {
        Ok(id) => id,
        Err(error) => {
            return Into::<AppError>::into(error).into_response();
        }
    };

    match app_state.database_connection().begin().await {
        Ok(transaction) => match BlacklistService::unblacklist_user(steam_id, &transaction).await {
            Ok(()) => {
                if let Err(cause) = transaction.commit().await {
                    return AppError::InternalServerError(Box::new(cause)).into_response();
                }
                StatusCode::NO_CONTENT.into_response()
            }
            Err(error) => Into::<AppError>::into(error).into_response(),
        },
        Err(cause) => AppError::InternalServerError(Box::new(cause)).into_response(),
    }
}

#[utoipa::path(
    get,
    path = "/api/admin/blacklist",
    responses(
        (status = 200, description = "Blacklist was successfully retrieved", body = [i64]),
        (status = 401, description = "Unauthorized",                         body = Details),
        (status = 500, description = "Internal Server Error",                body = Details),
    ),
    security(
        ("jwt_admin" = [])
    )
)]
#[tracing::instrument(skip(app_state))]
pub async fn full_blacklist(
    AdminAuthJWT(admin): AdminAuthJWT,
    State(app_state): State<Arc<AppState>>,
) -> Response {
    match BlacklistService::all(app_state.database_connection()).await {
        Ok(ids) => Json(ids).into_response(),
        Err(error) => Into::<AppError>::into(error).into_response(),
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, ToSchema)]
pub struct BlacklistUserRequest {
    steam_id: String,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, ToSchema)]
pub struct UnblacklistUserRequest {
    steam_id: String,
}
