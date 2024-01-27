use crate::errors::AppError;
use crate::extractors::admin_jwt::AdminAuthJWT;
use crate::services::admin::blacklist::Service as BlacklistService;
use crate::state::AppState;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::response::Response;
use axum::Form;
use axum::Json;
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
    Form(payload): Form<BlacklistUserRequest>,
) -> Response {
    match app_state.database_connection().begin().await {
        Ok(transaction) => {
            match BlacklistService::blacklist_user(payload.steam_id, &transaction).await {
                Ok(()) => {
                    if let Err(cause) = transaction.commit().await {
                        return AppError::InternalServerError(Box::new(cause)).into_response();
                    }
                    StatusCode::NO_CONTENT.into_response()
                }
                Err(error) => Into::<AppError>::into(error).into_response(),
            }
        }
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
    Form(payload): Form<UnblacklistUserRequest>,
) -> Response {
    match app_state.database_connection().begin().await {
        Ok(transaction) => {
            match BlacklistService::unblacklist_user(payload.steam_id, &transaction).await {
                Ok(()) => {
                    if let Err(cause) = transaction.commit().await {
                        return AppError::InternalServerError(Box::new(cause)).into_response();
                    }
                    StatusCode::NO_CONTENT.into_response()
                }
                Err(error) => Into::<AppError>::into(error).into_response(),
            }
        }
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
    steam_id: i64,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, ToSchema)]
pub struct UnblacklistUserRequest {
    steam_id: i64,
}
