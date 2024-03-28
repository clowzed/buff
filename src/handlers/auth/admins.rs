use axum::{
    extract::State,
    response::{IntoResponse, Response},
    routing::post,
    Json,
};
use sea_orm::TransactionTrait;
use utoipa::ToSchema;

use crate::{
    errors::AppError,
    services::auth::{AdminCredentials, GenerateAdminJwtParameters, Jwt, Service as AuthService},
    state::AppState,
};
use std::{fmt::Debug, sync::Arc};

#[derive(serde::Serialize, serde::Deserialize, ToSchema)]
pub struct Credentials {
    pub login: String,
    pub password: String,
}

impl Debug for Credentials {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Credentials")
            .field("login", &self.login)
            .finish()
    }
}

#[derive(serde::Serialize, serde::Deserialize, ToSchema)]
pub struct AdminLoginResponse {
    #[schema(value_type = String)]
    pub token: Jwt,
}

#[utoipa::path(
    post,
    path = "/api/auth/admin/login",
    request_body = Credentials,
    responses(
        (status = 200, description = "Admin or moderator was successfully authenticated", body = AdminLoginResponse),
        (status = 500, description = "Internal server error", body = Details),
        (status = 401, description = "Bad username or password", body = Details),
        (status = 400, description = "Bad request", body = Details),
    ),
)]
#[tracing::instrument(skip(app_state))]
pub async fn login(
    State(app_state): State<Arc<AppState>>,
    Json(admin_or_moderator_credentials): Json<Credentials>,
) -> Response {
    match app_state.database_connection().begin().await {
        Ok(transaction) => {
            let credentials = AdminCredentials {
                login: admin_or_moderator_credentials.login,
                password: admin_or_moderator_credentials.password,
            };

            match AuthService::login_admin(credentials, &transaction).await {
                Ok(admin_or_moderator) => {
                    if let Err(cause) = transaction.commit().await {
                        return AppError::InternalServerError(Box::new(cause)).into_response();
                    }

                    let parameters = GenerateAdminJwtParameters {
                        admin_id: admin_or_moderator.id,
                        secret: app_state.configuration().jwt_secret(),
                        ttl: app_state.configuration().jwt_ttl(),
                    };

                    match AuthService::admin_jwt(parameters) {
                        Ok(token) => Json(AdminLoginResponse { token }).into_response(),
                        Err(cause) => Into::<AppError>::into(cause).into_response(),
                    }
                }
                Err(cause) => Into::<AppError>::into(cause).into_response(),
            }
        }
        Err(cause) => AppError::InternalServerError(Box::new(cause)).into_response(),
    }
}

pub fn router() -> axum::Router<Arc<AppState>> {
    axum::Router::new().route("/login", post(login))
}
