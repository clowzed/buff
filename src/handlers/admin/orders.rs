use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use chrono::NaiveDateTime;
use sea_orm::TransactionTrait;
use std::sync::Arc;
use utoipa::{IntoParams, ToSchema};

use crate::{
    errors::AppError, extractors::admin_jwt::ModeratorAuthJWT,
    services::orders::Service as OrderService, state::AppState, Order,
};

use redis::AsyncCommands;

#[utoipa::path(
    patch,
    path = "/api/admin/order/{id}/cancel",
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
        ("jwt_admin" = [])
    )
)]
#[tracing::instrument(skip(app_state))]
pub async fn cancel_order_by_id(
    ModeratorAuthJWT(admin): ModeratorAuthJWT,
    State(app_state): State<Arc<AppState>>,
    Path(order_id): Path<i64>,
) -> axum::response::Response {
    match app_state.database_connection().begin().await {
        Ok(transaction) => match OrderService::cancel_order_by_id(order_id, &transaction).await {
            Ok(()) => {
                if let Err(cause) = transaction.commit().await {
                    return AppError::InternalServerError(Box::new(cause)).into_response();
                }
                StatusCode::NO_CONTENT.into_response()
            }
            Err(cause) => Into::<AppError>::into(cause).into_response(),
        },
        Err(cause) => AppError::InternalServerError(Box::new(cause)).into_response(),
    }
}

#[utoipa::path(
    patch,
    path = "/api/admin/order/{id}/success",
    responses(
        (status = 204, description = "Order was marked as succeeded"),
        (status = 404, description = "Order was not found", body = Details),
        (status = 401, description = "Unauthorized", body = Details),
        (status = 400, description = "Order has already been marked as canceled", body = Details),
        (status = 500, description = "Internal Server Error", body = Details),
    ),
    params(
        ("id" = i64, Path, description = "Order id")
    ),
    security(
        ("jwt_admin" = [])
    )
)]
#[tracing::instrument(skip(app_state))]
pub async fn finish_order_by_id(
    ModeratorAuthJWT(admin): ModeratorAuthJWT,
    State(app_state): State<Arc<AppState>>,
    Path(order_id): Path<i64>,
) -> axum::response::Response {
    match app_state.database_connection().begin().await {
        Ok(transaction) => match OrderService::finish_order_by_id(order_id, &transaction).await {
            Ok(order) => {
                if let Err(cause) = transaction.commit().await {
                    return AppError::InternalServerError(Box::new(cause)).into_response();
                }

                match app_state.redis_client().get_async_connection().await {
                    Ok(mut connection) => {
                        let _: Result<(), _> = connection
                            .publish("live_orders", serde_json::to_string(&order).unwrap())
                            .await;
                    }
                    Err(cause) => {
                        tracing::warn!(%cause, "Failed to connect to redis!");
                    }
                };

                StatusCode::NO_CONTENT.into_response()
            }
            Err(cause) => Into::<AppError>::into(cause).into_response(),
        },
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
    path = "/api/admin/order/all-in-period",
    request_body =
       TimeBounds
    ,
    responses(
        (status = 200, description = "Orders were successfully retrieved", body = [Order]),
        (status = 401, description = "Unauthorized",                              body = Details),
        (status = 500, description = "Internal Server Error",                     body = Details),
    ),
    security(
        ("jwt_admin" = [])
    )
)]
#[tracing::instrument(skip(app_state))]
pub async fn all_in_period(
    ModeratorAuthJWT(admin): ModeratorAuthJWT,
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
                .map(Into::<Order>::into)
                .collect::<Vec<_>>(),
        )
        .into_response(),
        Err(cause) => Into::<AppError>::into(cause).into_response(),
    }
}
