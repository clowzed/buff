use crate::{
    errors::AppError,
    extractors::user_jwt::AuthJWT,
    services::reviews::{AddReviewParameters, Service as ReviewsService},
};
use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    Form, Json,
};

use chrono::NaiveDateTime as DateTime;
use entity::review::Model as ReviewModel;
use entity::video_review::Model as VideoReviewModel;
use std::sync::Arc;
use utoipa::ToSchema;

use crate::state::AppState;

#[utoipa::path(
    get,
    path = "/api/review",
    responses(
        (status = 200, description = "Reviews were successfully retrieved",  body = [Review]),
        (status = 500, description = "Internal Server Error",                body = Details),
    )
)]
#[tracing::instrument(skip(app_state))]
pub async fn all_users_reviews(State(app_state): State<Arc<AppState>>) -> Response {
    match ReviewsService::users_all(app_state.database_connection()).await {
        Ok(reviews) => Json(
            reviews
                .into_iter()
                .map(Into::<Review>::into)
                .collect::<Vec<_>>(),
        )
        .into_response(),
        Err(cause) => Into::<AppError>::into(cause).into_response(),
    }
}

#[utoipa::path(
    get,
    path = "/api/review/video",
    responses(
        (status = 200, description = "Video reviews were successfully retrieved",  body = [VideoReview]),
        (status = 500, description = "Internal Server Error",                      body = Details),
    )
)]
#[tracing::instrument(skip(app_state))]
pub async fn all_video_reviews(State(app_state): State<Arc<AppState>>) -> Response {
    match ReviewsService::videos_all(app_state.database_connection()).await {
        Ok(reviews) => Json(
            reviews
                .into_iter()
                .map(Into::<VideoReview>::into)
                .collect::<Vec<_>>(),
        )
        .into_response(),
        Err(cause) => Into::<AppError>::into(cause).into_response(),
    }
}

#[utoipa::path(
    post,
    path = "/api/review",
    request_body = AddReviewRequest,
    responses(
        (status = 204, description = "Review was successfully saved"),
        (status = 400, description = "Bad request",                        body = Details),
        (status = 401, description = "Unauthorized",                       body = Details),
        (status = 500, description = "Internal Server Error",              body = Details),
    ),
    security(
        ("jwt_user" = [])
    )
)]
#[tracing::instrument(skip(app_state))]
pub async fn add_users_review(
    AuthJWT(user): AuthJWT,
    State(app_state): State<Arc<AppState>>,
    Form(payload): Form<AddReviewRequest>,
) -> Response {
    let review_to_be_added = AddReviewParameters {
        steam_id: user.steam_id,
        review: payload.review,
        stars: payload.stars,
    };

    match ReviewsService::add_users_review(review_to_be_added, app_state.database_connection())
        .await
    {
        Ok(()) => StatusCode::NO_CONTENT.into_response(),
        Err(cause) => Into::<AppError>::into(cause).into_response(),
    }
}

#[derive(ToSchema, serde::Serialize, serde::Deserialize, Debug)]
pub struct AddReviewRequest {
    pub review: String,
    pub stars: i16,
}

#[derive(ToSchema, serde::Serialize, serde::Deserialize)]
pub struct Review {
    pub id: i64,
    pub steam_id: i64,
    pub review: String,
    pub stars: i16,
    pub created_at: DateTime,
}

//? I was forced to write this by utoipa
//? It failed to resolve ToSchema in entity
//? Resulting in bad html
impl From<ReviewModel> for Review {
    fn from(value: ReviewModel) -> Self {
        Self {
            id: value.id,
            steam_id: value.steam_id,
            review: value.review,
            stars: value.stars,
            created_at: value.created_at,
        }
    }
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
