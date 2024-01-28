use crate::services::currency::Service as CurrencyService;
use crate::services::orders::{
    CancelOrderParameters, GetUserOrderParameters, Service as OrderService,
    SetOrderRequisitesParameters,
};
use crate::{
    errors::AppError, extractors::user_jwt::AuthJWT, services::orders::CreateOrderParameters,
    state::AppState,
};
use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{get, patch, post};
use axum::Json;
use axum::{extract::State, response::Response};
use chrono::NaiveDateTime as DateTime;
use entity::order::Model as OrderModel;

use sea_orm::prelude::Decimal;
use sea_orm::TransactionTrait;
use std::sync::Arc;
use utoipa::ToSchema;

pub mod live;

#[derive(Debug, ToSchema, serde::Serialize, serde::Deserialize)]
pub struct CreateOrderRequest {
    payment_method: String,
    #[schema(value_type = String)]
    amount: Decimal,
    currency: String,
}

#[derive(ToSchema, serde::Serialize, serde::Deserialize)]
pub struct Order {
    pub id: i64,
    pub payment_method: String,
    pub status: String,
    pub created_at: DateTime,
    pub steam_id: i64,
    pub moderator_id: Option<i64>,
    #[schema(value_type = String)]
    pub amount: Decimal,
    #[schema(value_type = String)]
    pub fixed_currency_rate: Decimal,
    pub currency_symbol: String,
    pub finished_at: Option<DateTime>,
}

impl From<OrderModel> for Order {
    fn from(value: OrderModel) -> Self {
        Self {
            id: value.id,
            payment_method: value.payment_method,
            status: serde_json::to_string(&value.status).unwrap(),
            created_at: value.created_at,
            steam_id: value.steam_id,
            moderator_id: value.moderator_id,
            amount: value.amount,
            fixed_currency_rate: value.fixed_currency_rate,
            currency_symbol: value.currency_symbol,
            finished_at: value.finished_at,
        }
    }
}

#[utoipa::path(
    post,
    path = "/api/user/order",
    request_body = CreateOrderRequest,
    responses(
        (status = 201, description = "Order was successfully created",     body = Order),
        (status = 400, description = "Bad request",                        body = Details),
        (status = 404, description = "Currency symbol was not found",             body = Details),
        (status = 401, description = "Unauthorized",                       body = Details),
        (status = 500, description = "Internal Server Error",              body = Details),
    ),
    security(
        ("jwt_user" = [])
    )
)]
#[tracing::instrument(skip(app_state))]
pub async fn create_order(
    AuthJWT(user): AuthJWT,
    State(app_state): State<Arc<AppState>>,
    Json(payload): Json<CreateOrderRequest>,
) -> Response {
    match app_state.database_connection().begin().await {
        Ok(transaction) => {
            let currency_rate =
                match CurrencyService::currency_rate(&payload.currency, &transaction).await {
                    Ok(currency_rate) => currency_rate,
                    Err(cause) => return Into::<AppError>::into(cause).into_response(),
                };

            let parameters = CreateOrderParameters {
                steam_id: user.steam_id,
                amount: payload.amount,
                payment_method: payload.payment_method,
                symbol: currency_rate.symbol,
                currency_rate: currency_rate.rate,
            };

            let created_order_model =
                match OrderService::create_order(parameters, &transaction).await {
                    Ok(order) => order,
                    Err(cause) => return Into::<AppError>::into(cause).into_response(),
                };

            if let Err(cause) = transaction.commit().await {
                return AppError::InternalServerError(Box::new(cause)).into_response();
            }

            (
                StatusCode::CREATED,
                Json(Into::<Order>::into(created_order_model)),
            )
                .into_response()
        }
        Err(cause) => AppError::InternalServerError(Box::new(cause)).into_response(),
    }
}

#[utoipa::path(
    patch,
    path = "/api/user/order/{id}/cancel",
    responses(
        (status = 204, description = "Order was successfully canceled"),
        (status = 404, description = "Order was not found", body = Details),
        (status = 401, description = "Unauthorized", body = Details),
        (status = 400, description = "Order has already been marked as succeeded", body = Details),
        (status = 500, description = "Internal Server Error", body = Details),
    ),
    params(
        ("id" = i32, Path, description = "Order id")
    ),
    security(
        ("jwt_user" = [])
    )
)]
#[tracing::instrument(skip(app_state))]
pub async fn cancel_order(
    AuthJWT(user): AuthJWT,
    State(app_state): State<Arc<AppState>>,
    Path(order_id): Path<i64>,
) -> Response {
    match app_state.database_connection().begin().await {
        Ok(transaction) => {
            let parameters = CancelOrderParameters {
                steam_id: user.steam_id,
                order_id,
            };

            match OrderService::cancel_order(parameters, &transaction).await {
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
    path = "/api/user/order",
    responses(
        (status = 200, description = "Orders were successfully retrieved", body = [Order]),
        (status = 401, description = "Unauthorized", body = Details),
        (status = 500, description = "Internal Server Error", body = Details),
    ),
    security(
        ("jwt_user" = [])
    )
)]
#[tracing::instrument(skip(app_state))]
pub async fn list_orders(
    AuthJWT(user): AuthJWT,
    State(app_state): State<Arc<AppState>>,
) -> Response {
    match app_state.database_connection().begin().await {
        Ok(transaction) => {
            let user_orders = match OrderService::user_orders(user.steam_id, &transaction).await {
                Ok(orders) => orders,
                Err(cause) => return Into::<AppError>::into(cause).into_response(),
            };

            if let Err(cause) = transaction.commit().await {
                return AppError::InternalServerError(Box::new(cause)).into_response();
            }

            (
                StatusCode::OK,
                Json(
                    user_orders
                        .into_iter()
                        .map(Into::<Order>::into)
                        .collect::<Vec<_>>(),
                ),
            )
                .into_response()
        }
        Err(cause) => AppError::InternalServerError(Box::new(cause)).into_response(),
    }
}

#[utoipa::path(
    get,
    path = "/api/user/order/{id}",
    responses(
        (status = 200, description = "Order was successfully retrieved", body = Order),
        (status = 404, description = "Order was not found", body = Details),
        (status = 401, description = "Unauthorized", body = Details),
        (status = 500, description = "Internal Server Error", body = Details),
    ),
    params(
        ("id" = i32, Path, description = "Order id")
    ),
    security(
        ("jwt_user" = [])
    )
)]
#[tracing::instrument(skip(app_state))]
pub async fn get_order(
    AuthJWT(user): AuthJWT,
    State(app_state): State<Arc<AppState>>,
    Path(order_id): Path<i64>,
) -> Response {
    match app_state.database_connection().begin().await {
        Ok(transaction) => {
            let parameters = GetUserOrderParameters {
                steam_id: user.steam_id,
                order_id,
            };

            let user_order: Order = match OrderService::user_order(parameters, &transaction).await {
                Ok(Some(order)) => order.into(),
                Ok(None) => return AppError::OrderWasNotFound.into_response(),
                Err(cause) => return Into::<AppError>::into(cause).into_response(),
            };
            if let Err(cause) = transaction.commit().await {
                return AppError::InternalServerError(Box::new(cause)).into_response();
            }
            (StatusCode::OK, Json(user_order)).into_response()
        }
        Err(cause) => AppError::InternalServerError(Box::new(cause)).into_response(),
    }
}

#[derive(serde::Serialize, serde::Deserialize, ToSchema, Debug)]
pub struct SetRequisitesRequest {
    pub requisites: Option<String>,
}
#[utoipa::path(
    patch,
    path = "/api/user/order/{id}",
    request_body = SetRequisitesRequest,
    responses(
        (status = 204, description = "Requisites were successfully set"),
        (status = 404, description = "Order was not found", body = Details),
        (status = 401, description = "Unauthorized", body = Details),
        (status = 500, description = "Internal Server Error", body = Details),
    ),
    params(
        ("id" = i32, Path, description = "Order id")
    ),
    security(
        ("jwt_user" = [])
    )
)]
#[tracing::instrument(skip(app_state))]
pub async fn set_requisites(
    AuthJWT(user): AuthJWT,
    State(app_state): State<Arc<AppState>>,
    Path(order_id): Path<i64>,
    Json(payload): Json<SetRequisitesRequest>,
) -> Response {
    match app_state.database_connection().begin().await {
        Ok(transaction) => {
            let parameters = SetOrderRequisitesParameters {
                steam_id: user.steam_id,
                order_id,
                requisites: payload.requisites,
            };

            if let Err(cause) = OrderService::set_requisites(parameters, &transaction).await {
                return Into::<AppError>::into(cause).into_response();
            }
            if let Err(cause) = transaction.commit().await {
                return AppError::InternalServerError(Box::new(cause)).into_response();
            }
            (StatusCode::NO_CONTENT).into_response()
        }
        Err(cause) => AppError::InternalServerError(Box::new(cause)).into_response(),
    }
}

pub fn router() -> axum::Router<Arc<AppState>> {
    axum::Router::new()
        .route("/", post(create_order))
        .route("/:id/cancel", patch(cancel_order))
        .route("/", get(list_orders))
        .route("/:id", get(get_order))
        .route("/live", get(live::websocket_handler))
        .route("/:id", patch(set_requisites))
}
