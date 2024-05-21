use crate::{
    errors::AppError,
    extractors::user_jwt::AuthJWT,
    services::{
        chat::{SendMessageParameters, Sender, Service as ChatService},
        currency::Service as CurrencyService,
        orders::{
            CancelOrderParameters, CreateOrderParameters, GetUserOrderParameters,
            MayBePayedOrderParameters, Service as OrderService,
        },
    },
    state::AppState,
    Message, SendMessageResponse,
};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, patch, post},
    Json,
};
use chrono::NaiveDateTime as DateTime;
use chrono::NaiveDateTime;
use entity::{chat::Entity as ChatEntity, order::Model as OrderModel};
use redis::AsyncCommands;
use sea_orm::{prelude::Decimal, ModelTrait, TransactionTrait};
use std::sync::Arc;
use utoipa::{IntoParams, ToSchema};

pub mod live;

#[derive(Debug, ToSchema, serde::Serialize, serde::Deserialize)]
pub struct CreateOrderRequest {
    payment_method: String,
    #[schema(value_type = String)]
    amount: Decimal,
    currency: String,
    requisites_id: String,
}

#[derive(ToSchema, serde::Serialize, serde::Deserialize)]
pub struct Order {
    pub id: String,
    pub payment_method: String,
    pub status: String,
    pub created_at: DateTime,
    pub steam_id: String,
    pub moderator_id: Option<String>,
    #[schema(value_type = String)]
    pub amount: Decimal,
    #[schema(value_type = String)]
    pub fixed_currency_rate: Decimal,
    pub currency_symbol: String,
    pub requisites_id: String,
    pub finished_at: Option<DateTime>,
}

impl From<OrderModel> for Order {
    fn from(value: OrderModel) -> Self {
        Self {
            id: value.id.to_string(),
            payment_method: value.payment_method,
            status: serde_json::to_string(&value.status).unwrap(),
            created_at: value.created_at,
            steam_id: value.steam_id.to_string(),
            moderator_id: value.moderator_id.map(|id| id.to_string()),
            amount: value.amount,
            fixed_currency_rate: value.fixed_currency_rate,
            currency_symbol: value.currency_symbol,
            finished_at: value.finished_at,
            requisites_id: value.requisites_id.to_string(),
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
    let requisites_id = match payload.requisites_id.parse::<i64>() {
        Ok(id) => id,
        Err(cause) => {
            return Into::<AppError>::into(cause).into_response();
        }
    };

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
                requisites_id,
            };

            let created_order_model =
                match OrderService::create_order(parameters, &transaction).await {
                    Ok(order) => order,
                    Err(cause) => return Into::<AppError>::into(cause).into_response(),
                };

            if let Err(cause) = transaction.commit().await {
                return AppError::InternalServerError(Box::new(cause)).into_response();
            }

            match app_state.redis_client().get_async_connection().await {
                Ok(mut connection) => {
                    let _: Result<(), _> = connection
                        .publish(
                            app_state.configuration().new_orders_channel_name(),
                            serde_json::to_string(&created_order_model).unwrap(),
                        )
                        .await;
                }
                Err(cause) => {
                    // Not very important
                    tracing::warn!(%cause, "Failed to connect to redis!");
                }
            };

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
        ("id" = i64, Path, description = "Order id")
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
    patch,
    path = "/api/user/order/{id}/maybepayed",
    responses(
        (status = 204, description = "Order was successfully set to maybepayed"),
        (status = 404, description = "Order was not found", body = Details),
        (status = 401, description = "Unauthorized", body = Details),
        (status = 400, description = "Order has already been marked as succeeded or canceled", body = Details),
        (status = 500, description = "Internal Server Error", body = Details),
    ),
    params(
        ("id" = i64, Path, description = "Order id")
    ),
    security(
        ("jwt_user" = [])
    )
)]
#[tracing::instrument(skip(app_state))]
pub async fn set_order_maybepayed(
    AuthJWT(user): AuthJWT,
    State(app_state): State<Arc<AppState>>,
    Path(order_id): Path<i64>,
) -> Response {
    match app_state.database_connection().begin().await {
        Ok(transaction) => {
            let parameters = MayBePayedOrderParameters {
                steam_id: user.steam_id,
                order_id,
            };
            match OrderService::maybepayed(parameters, &transaction).await {
                Ok(order) => {
                    match order.find_related(ChatEntity).one(&transaction).await {
                        Ok(Some(chat)) => {
                            let params = SendMessageParameters {
                                folder: app_state.configuration().upload_folder().clone(),
                                chat_id: chat.id,
                                sender: Sender::Moderator,
                                text: String::from("automessage-payed"), // This will be parsed by frontend to a normal message of moderator
                                image: None,
                            };

                            match ChatService::send_message(params, &transaction).await {
                                Ok(res) => {
                                    let send = SendMessageResponse {
                                        message: Into::<Message>::into(res.0),
                                        images_ids: vec![], // No images in automessage
                                    };

                                    match app_state.redis_client().get_async_connection().await {
                                        Ok(mut connection) => {
                                            let _: Result<(), _> = connection
                                                .publish(
                                                    format!("chat-{}", chat.id),
                                                    serde_json::to_string(&send).unwrap(),
                                                )
                                                .await;
                                        }
                                        Err(cause) => {
                                            // Not very important
                                            tracing::warn!(%cause, "Failed to connect to redis!");
                                        }
                                    };
                                }
                                Err(cause) => return Into::<AppError>::into(cause).into_response(),
                            };
                        }
                        Ok(None) => {}
                        Err(cause) => return Into::<AppError>::into(cause).into_response(),
                    };

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
        ("id" = i64, Path, description = "Order id")
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

#[derive(serde::Serialize, serde::Deserialize, Debug, ToSchema, IntoParams)]
pub struct TimeBounds {
    start_datetime: NaiveDateTime,
    end_datetime: NaiveDateTime,
}

#[utoipa::path(
    post,
    path = "/api/user/order/all-in-period",
    request_body =
       TimeBounds
    ,
    responses(
        (status = 200, description = "Orders were successfully retrieved", body = [Order]),
        (status = 401, description = "Unauthorized",                              body = Details),
        (status = 500, description = "Internal Server Error",                     body = Details),
    ),
    security(
        ("jwt_user" = [])
    )
)]
#[tracing::instrument(skip(app_state))]
pub async fn all_in_period(
    AuthJWT(user): AuthJWT,
    State(app_state): State<Arc<AppState>>,
    Json(bounds): Json<TimeBounds>,
) -> axum::response::Response {
    let period = if bounds.start_datetime <= bounds.end_datetime {
        (bounds.start_datetime, bounds.end_datetime)
    } else {
        (bounds.end_datetime, bounds.start_datetime)
    };

    match OrderService::all_in_period(period, app_state.database_connection()).await {
        Ok(orders) => Json(
            orders
                .into_iter()
                .filter(|order| order.steam_id == user.steam_id)
                .map(Into::<Order>::into)
                .collect::<Vec<_>>(),
        )
        .into_response(),
        Err(cause) => Into::<AppError>::into(cause).into_response(),
    }
}

pub fn router() -> axum::Router<Arc<AppState>> {
    axum::Router::new()
        .route("/", post(create_order))
        .route("/:id/cancel", patch(cancel_order))
        .route("/:id/maybepayed", patch(set_order_maybepayed))
        .route("/", get(list_orders))
        .route("/:id", get(get_order))
        .route("/live", get(live::websocket_handler))
        .route("/all-in-period", post(all_in_period))
}
