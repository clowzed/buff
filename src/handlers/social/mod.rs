use crate::{errors::AppError, services::social::Service as SocialService, state::AppState};
use axum::{
    extract::State,
    response::{IntoResponse, Response},
    routing::get,
    Json, Router,
};
use entity::social::Model as SocialModel;

use serde::{Deserialize, Serialize};
use std::sync::Arc;
use utoipa::ToSchema;

#[derive(Deserialize, Serialize, Debug, ToSchema)]
pub struct Social {
    id: i64,
    name: String,
    url: Option<String>,
}

impl From<SocialModel> for Social {
    fn from(value: SocialModel) -> Self {
        Self {
            id: value.id,
            name: value.name,
            url: value.url,
        }
    }
}

#[utoipa::path(
    get,
    path = "/api/socials",
    responses(
        (status = 200, description = "Socials were successfully retrieved", body = [Social]),
        (status = 500, description = "Internal Server Error",              body = Details),
    ),
)]
#[tracing::instrument(skip(app_state))]
pub async fn socials(State(app_state): State<Arc<AppState>>) -> Response {
    Json(
        match SocialService::all(app_state.database_connection()).await {
            Ok(models) => models
                .into_iter()
                .map(Into::<Social>::into)
                .collect::<Vec<_>>(),
            Err(cause) => return Into::<AppError>::into(cause).into_response(),
        },
    )
    .into_response()
}

pub fn router() -> axum::Router<Arc<AppState>> {
    Router::new().route("/", get(socials))
}
