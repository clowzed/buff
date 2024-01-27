use crate::{
    errors::AppError,
    extractors::admin_jwt::AdminAuthJWT,
    services::reviews::{AddVideoReviewParameters, Service as ReviewsService},
};
use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    Form,
};
use entity::video_review::Model as VideoReviewModel;
use sea_orm::TransactionTrait;

use std::sync::Arc;
use utoipa::ToSchema;

use crate::state::AppState;

#[utoipa::path(
    post,
    path = "/api/admin/review/video",
    request_body = AddVideoReviewRequest,
    responses(
        (status = 204, description = "Video review was successfully saved"),
        (status = 400, description = "Bad request",                        body = Details),
        (status = 401, description = "Unauthorized",                       body = Details),
        (status = 409, description = "Provided url has already been added to video reviews", body = Details),
        (status = 500, description = "Internal Server Error",              body = Details),
    ),
    security(
        ("jwt_admin" = [])
    )
)]
#[tracing::instrument(skip(app_state))]
pub async fn add_video_review(
    AdminAuthJWT(user): AdminAuthJWT,
    State(app_state): State<Arc<AppState>>,
    Form(payload): Form<AddVideoReviewRequest>,
) -> Response {
    let review_to_be_added = AddVideoReviewParameters { url: payload.url };

    match app_state.database_connection().begin().await {
        Ok(transaction) => {
            match ReviewsService::add_video_review(review_to_be_added, &transaction).await {
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
    delete,
    path = "/api/admin/review/video",
    request_body = RemoveVideoReviewRequest,
    responses(
        (status = 204, description = "Video review was successfully deleted"),
        (status = 400, description = "Bad request",                        body = Details),
        (status = 401, description = "Unauthorized",                       body = Details),
        (status = 404, description = "Provided review id does not exist",  body = Details),
        (status = 500, description = "Internal Server Error",              body = Details),
    ),
    security(
        ("jwt_admin" = [])
    )
)]
#[tracing::instrument(skip(app_state))]
pub async fn remove_video_review(
    AdminAuthJWT(user): AdminAuthJWT,
    State(app_state): State<Arc<AppState>>,
    Form(payload): Form<RemoveVideoReviewRequest>,
) -> Response {
    match app_state.database_connection().begin().await {
        Ok(transaction) => {
            match ReviewsService::remove_video_review(payload.id, &transaction).await {
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

#[derive(ToSchema, serde::Serialize, serde::Deserialize, Debug)]
pub struct AddVideoReviewRequest {
    pub url: String,
}

#[derive(ToSchema, serde::Serialize, serde::Deserialize, Debug)]
pub struct RemoveVideoReviewRequest {
    pub id: i64,
}

#[derive(ToSchema, serde::Serialize, serde::Deserialize)]
pub struct VideoReview {
    pub id: i64,
    pub url: String,
}

//? I was forced to write this by utoipa
//? It failed to resolve ToSchema in entity
//? Resulting in bad html
impl From<VideoReviewModel> for VideoReview {
    fn from(value: VideoReviewModel) -> Self {
        Self {
            id: value.id,
            url: value.url,
        }
    }
}

#[derive(ToSchema, serde::Serialize, serde::Deserialize, Debug)]
pub struct RemoveReviewRequest {
    pub id: i64,
}
#[utoipa::path(
    delete,
    path = "/api/admin/review",
    request_body = RemoveReviewRequest,
    responses(
        (status = 204, description = "Review was successfully deleted"),
        (status = 400, description = "Bad request",                        body = Details),
        (status = 401, description = "Unauthorized",                       body = Details),
        (status = 404, description = "Provided review id does not exist",  body = Details),
        (status = 500, description = "Internal Server Error",              body = Details),
    ),
    security(
        ("jwt_admin" = [])
    )
)]
#[tracing::instrument(skip(app_state))]
pub async fn remove_review(
    AdminAuthJWT(user): AdminAuthJWT,
    State(app_state): State<Arc<AppState>>,
    Form(payload): Form<RemoveVideoReviewRequest>,
) -> Response {
    match app_state.database_connection().begin().await {
        Ok(transaction) => match ReviewsService::remove_review(payload.id, &transaction).await {
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
