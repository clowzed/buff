use axum::routing::{get, post};

use crate::state::AppState;
use std::sync::Arc;

pub mod users;
use users::{add_users_review, all_users_reviews, all_video_reviews, count_reviews, five_stars};

pub fn router() -> axum::Router<Arc<AppState>> {
    axum::Router::new()
        .route("/", get(all_users_reviews))
        .route("/", post(add_users_review))
        .route("/video", get(all_video_reviews))
        .route("/count", get(count_reviews))
        .route("/five-stars", get(five_stars))
}
