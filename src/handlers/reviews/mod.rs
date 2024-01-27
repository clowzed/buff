use axum::routing::{get, post};

use crate::state::AppState;
use std::sync::Arc;

use self::users::{add_users_review, all_users_reviews, all_video_reviews};

pub mod users;

pub fn router() -> axum::Router<Arc<AppState>> {
    axum::Router::new()
        .route("/", get(all_users_reviews))
        .route("/", post(add_users_review))
        .route("/video", get(all_video_reviews))
}
