use crate::{
    errors::AppError,
    openid::VerifyForm,
    services::{
        auth::{GenerateUserJwtParameters, Jwt, Service as AuthService},
        users::Service as UserService,
    },
    state::AppState,
};
use axum::{
    extract::{Query, State},
    response::{IntoResponse, Response},
    routing::get,
    Json,
};
use sea_orm::ActiveModelTrait;
use sea_orm::{IntoActiveModel, Set, TransactionTrait};
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
    Query(verify): Query<VerifyForm>,
) -> Response {
    let steam_id = match app_state
        .steam_openid()
        .verify(verify)
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
                    // FIXME Everything here is trash

                    let fetched_text = match reqwest::get(format!(
                        "https://steamcommunity.com/profiles/{steam_id}"
                    ))
                    .await
                    {
                        Ok(response) => Some(response.text().await.unwrap_or_default()),
                        Err(_) => None,
                    };

                    let avatar_url_regex =
                        once_cell_regex::regex!(r#"<link\s+rel="image_src"\s+href="([^"]+)"\s*>"#);
                    let username_regex = once_cell_regex::regex!(r#""personaname":"([^"]+)""#);

                    let avatar_url = fetched_text.as_ref().and_then(|html| {
                        avatar_url_regex.captures(&html).and_then(|captures| {
                            captures
                                .get(1)
                                .map(|personaname| personaname.as_str().replace(r"\", ""))
                        })
                    });

                    let username = fetched_text.as_ref().and_then(|html| {
                        username_regex.captures(&html).and_then(|captures| {
                            captures
                                .get(1)
                                .map(|personaname| personaname.as_str().replace(r"\", ""))
                        })
                    });

                    let cloned_user = user.clone();
                    let cloned_user_again = user.clone();

                    if let Some(url) = avatar_url {
                        match user.avatar_url {
                            Some(ref user_avatar_url) if url.ne(user_avatar_url) => {
                                let mut active = cloned_user.into_active_model();
                                active.avatar_url = Set(Some(url.clone()));
                                if let Err(cause) = active.update(&transaction).await {
                                    return AppError::InternalServerError(Box::new(cause))
                                        .into_response();
                                }
                            }
                            None => {
                                let mut active = cloned_user.into_active_model();
                                active.avatar_url = Set(Some(url.clone()));
                                if let Err(cause) = active.update(&transaction).await {
                                    return AppError::InternalServerError(Box::new(cause))
                                        .into_response();
                                }
                            }
                            _ => {}
                        }
                    };

                    if let Some(name) = username {
                        match user.username {
                            Some(ref used_username) if used_username.ne(&name) => {
                                let mut active = cloned_user_again.into_active_model();
                                active.username = Set(Some(name.clone()));
                                if let Err(cause) = active.update(&transaction).await {
                                    return AppError::InternalServerError(Box::new(cause))
                                        .into_response();
                                }
                            }
                            None => {
                                let mut active = cloned_user_again.into_active_model();
                                active.username = Set(Some(name.clone()));
                                if let Err(cause) = active.update(&transaction).await {
                                    return AppError::InternalServerError(Box::new(cause))
                                        .into_response();
                                }
                            }
                            _ => {}
                        }
                    };

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
