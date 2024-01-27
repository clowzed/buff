use crate::services::currency::CreateCurrencyRateParameters;
use crate::services::currency::Service as CurrencyService;
use crate::services::currency::SetCurrencyRateParameters;
use crate::{errors::AppError, extractors::admin_jwt::AdminAuthJWT, state::AppState};
use axum::extract::Path;
use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    Form, Json,
};
use entity::currency_rate::Model as RateModel;
use sea_orm::{prelude::Decimal, TransactionTrait};
use std::sync::Arc;
use utoipa::ToSchema;

#[derive(serde::Deserialize, serde::Serialize, Debug, ToSchema)]
pub struct CreateCurrencyRequest {
    symbol: String,
    #[schema(value_type = String)]
    rate: Decimal,
}
#[derive(serde::Deserialize, serde::Serialize, Debug, ToSchema)]
pub struct Currency {
    symbol: String,
    #[schema(value_type = String)]
    rate: Decimal,
}

impl From<RateModel> for Currency {
    fn from(value: RateModel) -> Self {
        Self {
            symbol: value.symbol,
            rate: value.rate,
        }
    }
}

#[utoipa::path(
    post,
    path = "/api/admin/currency",
    request_body = CreateCurrencyRequest,
    responses(
        (status = 201, description = "Currency rate was successfully created", body = Order),
        (status = 401, description = "Unauthorized", body = Details),
        (status = 500, description = "Internal Server Error", body = Details),
    ),
)]
pub async fn create_currency(
    State(app_state): State<Arc<AppState>>,
    AdminAuthJWT(_admin): AdminAuthJWT,
    Form(payload): Form<CreateCurrencyRequest>,
) -> Response {
    match app_state.database_connection().begin().await {
        Ok(transaction) => {
            let parameters = CreateCurrencyRateParameters {
                symbol: payload.symbol,
                rate: payload.rate,
            };
            let rate = match CurrencyService::create(parameters, &transaction).await {
                Ok(rate) => rate,
                Err(cause) => return Into::<AppError>::into(cause).into_response(),
            };

            if let Err(cause) = transaction.commit().await {
                return AppError::InternalServerError(Box::new(cause)).into_response();
            }

            (StatusCode::CREATED, Json(Into::<Currency>::into(rate))).into_response()
        }
        Err(cause) => AppError::InternalServerError(Box::new(cause)).into_response(),
    }
}

#[utoipa::path(
    delete,
    path = "/api/admin/currency/{id}",
    responses(
        (status = 204, description = "Currency rate was successfully deleted"),
        (status = 404, description = "Currency was not found", body = Details),
        (status = 401, description = "Unauthorized", body = Details),
        (status = 500, description = "Internal Server Error", body = Details),
    ),
    params(
        ("id" = i32, Path, description ="Currency rate id")
    ),
    security(
        ("jwt_admin" = [])
    )
)]
pub async fn delete_currency_rate_by_id(
    State(app_state): State<Arc<AppState>>,
    AdminAuthJWT(_admin): AdminAuthJWT,
    Path(id): Path<i64>,
) -> Response {
    match app_state.database_connection().begin().await {
        Ok(transaction) => {
            if let Err(cause) = CurrencyService::delete(id, &transaction).await {
                return Into::<AppError>::into(cause).into_response();
            };

            if let Err(cause) = transaction.commit().await {
                return AppError::InternalServerError(Box::new(cause)).into_response();
            }

            (StatusCode::NO_CONTENT).into_response()
        }
        Err(cause) => AppError::InternalServerError(Box::new(cause)).into_response(),
    }
}

#[derive(serde::Deserialize, serde::Serialize, ToSchema)]
pub struct SetRateRequest {
    #[schema(value_type = String)]
    pub rate: Decimal,
}

#[utoipa::path(
    patch,
    path = "/api/admin/currency/{id}",
    request_body = SetRateRequest,
    responses(
        (status = 204, description = "Currency rate was successfully changed"),
        (status = 404, description = "Currency was not found", body = Details),
        (status = 401, description = "Unauthorized", body = Details),
        (status = 500, description = "Internal Server Error", body = Details),
    ),
    params(
        ("id" = i32, Path, description ="Currency rate id")
    ),
    security(
        ("jwt_admin" = [])
    )
)]
pub async fn set_currency_rate_by_id(
    State(app_state): State<Arc<AppState>>,
    AdminAuthJWT(_admin): AdminAuthJWT,
    Path(id): Path<i64>,
    Form(payload): Form<SetRateRequest>,
) -> Response {
    match app_state.database_connection().begin().await {
        Ok(transaction) => {
            let parameters = SetCurrencyRateParameters {
                rate: payload.rate,
                id,
            };

            if let Err(cause) = CurrencyService::set_rate(parameters, &transaction).await {
                return Into::<AppError>::into(cause).into_response();
            };

            if let Err(cause) = transaction.commit().await {
                return AppError::InternalServerError(Box::new(cause)).into_response();
            }

            StatusCode::NO_CONTENT.into_response()
        }
        Err(cause) => AppError::InternalServerError(Box::new(cause)).into_response(),
    }
}
