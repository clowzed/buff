use crate::handlers::admin::currency::Currency;

use crate::services::currency::Service as CurrencyService;

use crate::{errors::AppError, state::AppState};
use axum::extract::Path;
use axum::routing::get;
use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};

use sea_orm::TransactionTrait;
use std::sync::Arc;

#[utoipa::path(
    get,
    path = "/api/currency/{id}",
    responses(
        (status = 200, description = "Currency rate was successfully retrieved", body = Currency),
        (status = 404, description = "Currency was not found", body = Details),
        (status = 500, description = "Internal Server Error", body = Details),
    ),
    params(
        ("id" = i32, Path, description ="Currency rate id")
    ),
)]
pub async fn get_currency_rate_by_id(
    State(app_state): State<Arc<AppState>>,
    Path(id): Path<i64>,
) -> Response {
    match app_state.database_connection().begin().await {
        Ok(transaction) => {
            let rate = match CurrencyService::get(id, &transaction).await {
                Ok(rate) => rate,
                Err(cause) => return Into::<AppError>::into(cause).into_response(),
            };

            if let Err(cause) = transaction.commit().await {
                return AppError::InternalServerError(Box::new(cause)).into_response();
            }

            (StatusCode::OK, Json(Into::<Currency>::into(rate))).into_response()
        }
        Err(cause) => AppError::InternalServerError(Box::new(cause)).into_response(),
    }
}

#[utoipa::path(
    get,
    path = "/api/currency",
    responses(
        (status = 200, description = "Currency rates were successfully retrieved", body = [Currency]),
        (status = 500, description = "Internal Server Error", body = Details),
    ),
)]
pub async fn get_currency_rates(State(app_state): State<Arc<AppState>>) -> Response {
    match app_state.database_connection().begin().await {
        Ok(transaction) => {
            let rates = match CurrencyService::all(&transaction).await {
                Ok(rates) => rates,
                Err(cause) => return Into::<AppError>::into(cause).into_response(),
            };

            if let Err(cause) = transaction.commit().await {
                return AppError::InternalServerError(Box::new(cause)).into_response();
            }

            (
                StatusCode::OK,
                Json(
                    rates
                        .into_iter()
                        .map(Into::<Currency>::into)
                        .collect::<Vec<_>>(),
                ),
            )
                .into_response()
        }
        Err(cause) => AppError::InternalServerError(Box::new(cause)).into_response(),
    }
}

pub fn router() -> axum::Router<Arc<AppState>> {
    axum::Router::new()
        .route("/", get(get_currency_rates))
        .route("/:id", get(get_currency_rate_by_id))
}
