use crate::services::requisites::Service as RequisitesService;
use crate::{errors::AppError, state::AppState};
use axum::Router;
use axum::{
    extract::State,
    response::{IntoResponse, Response},
    routing::get,
    Json,
};
use entity::requisites::Model as RequisitesModel;

use serde::{Deserialize, Serialize};
use std::sync::Arc;
use utoipa::ToSchema;

#[derive(Deserialize, Serialize, Debug, ToSchema)]
pub struct Requisites {
    id: i64,
    name: String,
    data: Option<String>,
}

impl From<RequisitesModel> for Requisites {
    fn from(value: RequisitesModel) -> Self {
        Self {
            id: value.id,
            name: value.name,
            data: value.data,
        }
    }
}

#[utoipa::path(
    get,
    path = "/api/requisites",
    responses(
        (status = 200, description = "Requisites were successfully retrieved", body = [Requisites]),
        (status = 500, description = "Internal Server Error",              body = Details),
    ),
)]
#[tracing::instrument(skip(app_state))]
pub async fn requisites(State(app_state): State<Arc<AppState>>) -> Response {
    Json(
        match RequisitesService::all(app_state.database_connection()).await {
            Ok(models) => models
                .into_iter()
                .map(Into::<Requisites>::into)
                .collect::<Vec<_>>(),
            Err(cause) => return Into::<AppError>::into(cause).into_response(),
        },
    )
    .into_response()
}

pub fn router() -> axum::Router<Arc<AppState>> {
    Router::new().route("/", get(requisites))
}
