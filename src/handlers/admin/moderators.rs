use crate::extractors::admin_jwt::ModeratorAuthJWT;
use crate::services::admin::moderators::AssignModeratorParameters;
use crate::services::admin::moderators::CreateModeratorParameters;
use crate::services::admin::moderators::Service as AdminService;
use crate::services::admin::moderators::UnassignModeratorParameters;
use crate::Order;
use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use axum::{extract::State, response::Response};
use entity::admin::Model as AdminModel;
use sea_orm::TransactionTrait;
use std::sync::Arc;
use utoipa::ToSchema;

use crate::{errors::AppError, extractors::admin_jwt::AdminAuthJWT, state::AppState};

#[derive(serde::Serialize, serde::Deserialize, ToSchema)]
pub struct ModeratorCredentials {
    pub login: String,
    pub password: String,
}

impl ModeratorCredentials {
    pub fn valid(&self) -> bool {
        !self.login.is_empty() && !self.password.is_empty()
    }
}

#[derive(serde::Serialize, serde::Deserialize, ToSchema)]
pub struct ModeratorResponse {
    pub id: i64,
    pub login: String,
}

impl From<AdminModel> for ModeratorResponse {
    fn from(value: AdminModel) -> Self {
        Self {
            id: value.id,
            login: value.login,
        }
    }
}

#[utoipa::path(
    post,
    path = "/api/admin/moderator",
    request_body = ModeratorCredentials,
    responses(
        (status = 200, description = "Moderator was successfully created", body = ModeratorResponse),
        (status = 400, description = "Bad request",                        body = Details),
        (status = 409, description = "Login was occupied",                 body = Details),
        (status = 401, description = "Unauthorized",                       body = Details),
        (status = 500, description = "Internal Server Error",              body = Details),
    ),
    security(
        ("jwt_admin" = [])
    )
)]
pub async fn create_moderator(
    State(app_state): State<Arc<AppState>>,
    AdminAuthJWT(_admin): AdminAuthJWT,
    Json(credentials): Json<ModeratorCredentials>,
) -> Response {
    if !credentials.valid() {
        return AppError::EmptyCredentials.into_response();
    }
    let parameters = CreateModeratorParameters {
        login: credentials.login,
        password: credentials.password,
    };
    match app_state.database_connection().begin().await {
        Ok(transaction) => match AdminService::create_moderator(parameters, &transaction).await {
            Ok(moderator) => {
                if let Err(cause) = transaction.commit().await {
                    return AppError::InternalServerError(Box::new(cause)).into_response();
                }
                Json(Into::<ModeratorResponse>::into(moderator)).into_response()
            }
            Err(cause) => Into::<AppError>::into(cause).into_response(),
        },
        Err(cause) => AppError::InternalServerError(Box::new(cause)).into_response(),
    }
}

#[utoipa::path(
    delete,
    path = "/api/admin/moderator/{id}",
    responses(
        (status = 204, description = "Moderator was successfully deleted"),
        (status = 400, description = "Bad request",                        body = Details),
        (status = 401, description = "Unauthorized",                       body = Details),
        (status = 404, description = "Moderator was not found",            body = Details),
        (status = 403, description = "Moderator has admin role",           body = Details),
        (status = 500, description = "Internal Server Error",              body = Details),
    ),
    params(("id" = i32, Path, description = "Moderator id")),
    security(
        ("jwt_admin" = [])
    )
)]
#[tracing::instrument(skip(app_state, _admin))]
pub async fn delete_moderator(
    State(app_state): State<Arc<AppState>>,
    AdminAuthJWT(_admin): AdminAuthJWT,
    Path(moderator_id): Path<i64>,
) -> Response {
    match app_state.database_connection().begin().await {
        Ok(transaction) => match AdminService::delete_moderator(moderator_id, &transaction).await {
            Ok(()) => {
                if let Err(cause) = transaction.commit().await {
                    return AppError::InternalServerError(Box::new(cause)).into_response();
                }
                StatusCode::NO_CONTENT.into_response()
            }
            Err(cause) => Into::<AppError>::into(cause).into_response(),
        },
        Err(cause) => AppError::InternalServerError(Box::new(cause)).into_response(),
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, ToSchema)]
pub struct AssignModeratorRequest {
    order_id: i64,
}

#[utoipa::path(
    patch,
    path = "/api/admin/moderator/assign",
    request_body = AssignModeratorRequest,
    responses(
        (status = 204, description = "Moderator was successfully assigned to the order"),
        (status = 400, description = "Bad request",                        body = Details),
        (status = 401, description = "Unauthorized",                       body = Details),
        (status = 404, description = "Moderator was not found",            body = Details),
        (status = 403, description = "Moderator has admin role",           body = Details),
        (status = 500, description = "Internal Server Error",              body = Details),
    ),
    security(
        ("jwt_admin" = [])
    )
)]
#[tracing::instrument(skip(app_state, moderator))]
pub async fn assign_moderator(
    State(app_state): State<Arc<AppState>>,
    ModeratorAuthJWT(moderator): ModeratorAuthJWT,
    Json(payload): Json<AssignModeratorRequest>,
) -> Response {
    match app_state.database_connection().begin().await {
        Ok(transaction) => {
            let parameters = AssignModeratorParameters {
                moderator_id: moderator.id,
                order_id: payload.order_id,
            };
            match AdminService::assign_moderator(parameters, &transaction).await {
                Ok(()) => {
                    if let Err(cause) = transaction.commit().await {
                        return AppError::InternalServerError(Box::new(cause)).into_response();
                    }
                    StatusCode::NO_CONTENT.into_response()
                }
                Err(cause) => Into::<AppError>::into(cause).into_response(),
            }
        }
        Err(cause) => AppError::InternalServerError(Box::new(cause)).into_response(),
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, ToSchema)]
pub struct UnassignModeratorRequest {
    order_id: i64,
}

#[utoipa::path(
    patch,
    path = "/api/admin/moderator/unassign",
    request_body = UnassignModeratorRequest,
    responses(
        (status = 204, description = "Moderator was successfully unassigned from the order"),
        (status = 400, description = "Bad request",                        body = Details),
        (status = 401, description = "Unauthorized",                       body = Details),
        (status = 409, description = "Some conflict occurred",              body = Details),
        (status = 500, description = "Internal Server Error",              body = Details),
    ),
    security(
        ("jwt_admin" = [])
    )
)]
#[tracing::instrument(skip(app_state, moderator))]
pub async fn unassign_moderator(
    State(app_state): State<Arc<AppState>>,
    ModeratorAuthJWT(moderator): ModeratorAuthJWT,
    Json(payload): Json<UnassignModeratorRequest>,
) -> Response {
    match app_state.database_connection().begin().await {
        Ok(transaction) => {
            let parameters = UnassignModeratorParameters {
                moderator_id: moderator.id,
                order_id: payload.order_id,
            };
            match AdminService::unassign_moderator(parameters, &transaction).await {
                Ok(()) => {
                    if let Err(cause) = transaction.commit().await {
                        return AppError::InternalServerError(Box::new(cause)).into_response();
                    }
                    StatusCode::NO_CONTENT.into_response()
                }
                Err(cause) => Into::<AppError>::into(cause).into_response(),
            }
        }
        Err(cause) => AppError::InternalServerError(Box::new(cause)).into_response(),
    }
}

#[utoipa::path(
    get,
    path = "/api/admin/moderator",
    responses(
        (status = 200, description = "Moderators were successfully retrieved", body = [ModeratorResponse]),
        (status = 400, description = "Bad request",                        body = Details),
        (status = 401, description = "Unauthorized",                       body = Details),
        (status = 500, description = "Internal Server Error",              body = Details),
    ),
    security(
        ("jwt_admin" = [])
    )
)]
pub async fn list_moderators(
    State(app_state): State<Arc<AppState>>,
    AdminAuthJWT(_admin): AdminAuthJWT,
) -> Response {
    match AdminService::moderators(app_state.database_connection()).await {
        Ok(moderators) => Json(
            moderators
                .into_iter()
                .map(Into::<ModeratorResponse>::into)
                .collect::<Vec<_>>(),
        )
        .into_response(),
        Err(cause) => Into::<AppError>::into(cause).into_response(),
    }
}

#[utoipa::path(
    get,
    path = "/api/admin/moderator/orders",
    responses(
        (status = 200, description = "Orders were successfully retrieved", body = [Order]),
        (status = 400, description = "Bad request",                        body = Details),
        (status = 401, description = "Unauthorized",                       body = Details),
        (status = 404, description = "Moderator was not found",            body = Details),
        (status = 500, description = "Internal Server Error",              body = Details),
    ),
    security(
        ("jwt_admin" = [])
    )
)]
pub async fn list_moderators_orders(
    State(app_state): State<Arc<AppState>>,
    ModeratorAuthJWT(moderator): ModeratorAuthJWT,
) -> Response {
    match AdminService::moderators_orders(moderator.id, app_state.database_connection()).await {
        Ok(moderators) => Json(
            moderators
                .into_iter()
                .map(Into::<Order>::into)
                .collect::<Vec<_>>(),
        )
        .into_response(),
        Err(cause) => Into::<AppError>::into(cause).into_response(),
    }
}

#[derive(serde::Serialize, serde::Deserialize, ToSchema)]
pub struct ModeratorOrAdminInfo {
    login: String,
    role: String,
}

impl From<AdminModel> for ModeratorOrAdminInfo {
    fn from(value: AdminModel) -> Self {
        Self {
            login: value.login,
            role: serde_json::to_string(&value.role).unwrap(),
        }
    }
}

#[utoipa::path(
    get,
    path = "/api/admin/self",
    responses(
        (status = 200, description = "Information was successfully retrieved", body = ModeratorOrAdminInfo),
        (status = 401, description = "Unauthorized",                       body = Details),
    ),
    security(
        ("jwt_admin" = [])
    )
)]
pub async fn self_info(ModeratorAuthJWT(moderator): ModeratorAuthJWT) -> impl IntoResponse {
    Json(Into::<ModeratorOrAdminInfo>::into(moderator))
}
