use crate::{
    errors::AppError,
    extractors::admin_jwt::AdminAuthJWT,
    services::social::{Service as SocialService, SetSocialParameters},
};
use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};

use sea_orm::TransactionTrait;
use serde::{Deserialize, Serialize};

use std::sync::Arc;
use utoipa::ToSchema;

use crate::state::AppState;

#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct SetSocialUrlRequest {
    id: String,
    url: Option<String>,
}

#[utoipa::path(
    patch,
    path = "/api/admin/social",
    request_body = SetSocialUrlRequest,
    responses(
        (status = 204, description = "Social url was successfully updated"),
        (status = 400, description = "Bad request",                        body = Details),
        (status = 404, description = "Name was not found",                 body = Details),
        (status = 401, description = "Unauthorized",                       body = Details),
        (status = 500, description = "Internal Server Error",              body = Details),
    ),
    security(
        ("jwt_admin" = [])
    )
)]
#[tracing::instrument(skip(app_state))]
pub async fn set_url(
    AdminAuthJWT(user): AdminAuthJWT,
    State(app_state): State<Arc<AppState>>,
    Json(payload): Json<SetSocialUrlRequest>,
) -> Response {
    let id = match payload.id.parse::<i64>() {
        Ok(id) => id,
        Err(cause) => {
            return Into::<AppError>::into(cause).into_response();
        }
    };
    let social_update_parameters = SetSocialParameters {
        url: payload.url,
        id,
    };

    match app_state.database_connection().begin().await {
        Ok(transaction) => match SocialService::set(social_update_parameters, &transaction).await {
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
