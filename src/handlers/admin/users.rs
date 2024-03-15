use crate::errors::AppError;
use crate::services::users::Service as UserService;
use crate::{extractors::admin_jwt::ModeratorAuthJWT, state::AppState};
use axum::response::IntoResponse;
use axum::{
    extract::{Query, State},
    Json,
};
use chrono::NaiveDateTime;
use std::sync::Arc;
use utoipa::{IntoParams, ToSchema};

#[derive(serde::Serialize, serde::Deserialize, Debug, ToSchema, IntoParams)]
pub struct TimeBounds {
    start_datetime: NaiveDateTime,
    end_datetime: NaiveDateTime,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, ToSchema)]
pub struct RegistrationsInPeriodResponse {
    count: u32,
}

#[utoipa::path(
    get,
    path = "/api/admin/users/registrations-in-period",
    params(
       TimeBounds
    ),
    responses(
        (status = 200, description = "Registrations were successfully retrieved", body = RegistrationsInPeriodResponse),
        (status = 401, description = "Unauthorized",                              body = Details),
        (status = 500, description = "Internal Server Error",                     body = Details),
    ),
    security(
        ("jwt_admin" = [])
    )
)]
#[tracing::instrument(skip(app_state))]
pub async fn registrations_in_period(
    ModeratorAuthJWT(admin): ModeratorAuthJWT,
    State(app_state): State<Arc<AppState>>,
    Query(bounds): Query<TimeBounds>,
) -> axum::response::Response {
    let period = if bounds.start_datetime <= bounds.end_datetime {
        (bounds.start_datetime, bounds.end_datetime)
    } else {
        (bounds.end_datetime, bounds.start_datetime)
    };

    match UserService::registered_in_period(period, app_state.database_connection()).await {
        Ok(count) => Json(RegistrationsInPeriodResponse {
            count: count as u32,
        })
        .into_response(),
        Err(cause) => Into::<AppError>::into(cause).into_response(),
    }
}
