use crate::errors::AppError;
use crate::openid::VerifyForm;
use crate::services::auth::{GenerateUserJwtParameters, Jwt, Service as AuthService};
use crate::services::users::Service as UserService;
use crate::state::AppState;
use axum::extract::{Query, State};
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use axum::Json;
use sea_orm::TransactionTrait;
use std::sync::Arc;
use utoipa::ToSchema;

#[derive(serde::Serialize, serde::Deserialize, ToSchema)]
pub struct LoginLinkResponse {
    pub link: String,
}

#[derive(serde::Serialize, serde::Deserialize, ToSchema)]
pub struct JwtResponse {
    #[schema(value_type = String)]
    pub token: Jwt,
}

#[utoipa::path(
    get,
    path = "/api/auth/user/link",
    responses(
        (status = 200, description = "Link was successfully generated", body = LoginLinkResponse),
    ),
)]
#[tracing::instrument(skip(app_state))]
pub async fn login_link(State(app_state): State<Arc<AppState>>) -> impl IntoResponse {
    Json(LoginLinkResponse {
        link: app_state.steam_openid().get_redirect_url().to_owned(),
    })
}

#[utoipa::path(
    get,
    path = "/api/auth/user/callback",
    params(
        VerifyForm
    ),
    responses(
        (status = 200, description = "User was successfully authenticated", body = JwtResponse),
        (status = 403, description = "Steam denied user registration", body = Details),
        (status = 500, description = "Internal server error", body = Details),
        (status = 400, description = "Bad request", body = Details),
    ),
)]
#[tracing::instrument(skip(app_state))]
pub async fn login(
    State(app_state): State<Arc<AppState>>,
    Query(form): Query<VerifyForm>,
) -> Response {
    let steam_id = match app_state
        .steam_openid()
        .verify(form)
        .await
        .map_err(Into::<AppError>::into)
    {
        Ok(id) => id,
        Err(err) => return err.into_response(),
    };

    match app_state.database_connection().begin().await {
        Ok(transaction) => {
            let user = match UserService::safe_registration(steam_id, &transaction).await {
                Ok(user) => {
                    if let Err(cause) = transaction.commit().await {
                        return AppError::InternalServerError(Box::new(cause)).into_response();
                    }
                    user
                }
                Err(cause) => return Into::<AppError>::into(cause).into_response(),
            };

            let parameters = GenerateUserJwtParameters {
                steam_id: user.steam_id,
                secret: app_state.configuration().jwt_secret(),
            };

            return match AuthService::user_jwt(parameters).map_err(Into::<AppError>::into) {
                Ok(jwt) => Json(JwtResponse { token: jwt }).into_response(),
                Err(cause) => cause.into_response(),
            };
        }
        Err(cause) => return AppError::InternalServerError(Box::new(cause)).into_response(),
    };
}

pub fn router() -> axum::Router<Arc<AppState>> {
    axum::Router::new()
        .route("/link", get(login_link))
        .route("/callback", get(login))
}
