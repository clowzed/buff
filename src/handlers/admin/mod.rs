use crate::state::AppState;
use axum::routing::{delete, get, patch, post};
use std::sync::Arc;

pub mod blacklist;
pub mod currency;
pub mod moderators;
pub mod orders;
pub mod requisites;
pub mod reviews;
pub mod social;

pub fn router() -> axum::Router<Arc<AppState>> {
    axum::Router::new()
        .route("/blacklist", get(blacklist::full_blacklist))
        .route("/blacklist", post(blacklist::blacklist_user))
        .route("/blacklist", delete(blacklist::unblacklist_user))
        .route("/review/video", post(reviews::add_video_review))
        .route("/review/video", delete(reviews::remove_video_review))
        .route("/review/video", patch(reviews::update_video_review))
        .route("/moderator", post(moderators::create_moderator))
        .route("/moderator/:id", delete(moderators::delete_moderator))
        .route("/moderator", get(moderators::list_moderators))
        .route("/moderator/orders", get(moderators::list_moderators_orders))
        .route(
            "/moderator/unassigned-orders",
            get(moderators::list_unassigned_orders),
        )
        .route("/moderator/unassign", patch(moderators::unassign_moderator))
        .route("/moderator/assign", patch(moderators::assign_moderator))
        .route("/review", delete(reviews::remove_review))
        .route("/order/:id/cancel", patch(orders::cancel_order_by_id))
        .route("/order/:id/success", patch(orders::finish_order_by_id))
        .route("/order/all-in-period", get(orders::all_in_period))
        .route("/currency", post(currency::create_currency))
        .route(
            "/currency/:id",
            delete(currency::delete_currency_rate_by_id),
        )
        .route("/currency/:id", patch(currency::set_currency_rate_by_id))
        .route("/self", get(moderators::self_info))
        .route("/social", patch(social::set_url))
        .route("/requisites", patch(requisites::set_data))
        .route("/moderator/password", patch(moderators::change_password))
        .route("/moderator/chat", patch(moderators::chat))
        .route(
            "/moderator/chat/:id/message",
            post(moderators::send_message),
        )
        .route("/moderator/chat/:id/history", get(moderators::history))
        .route("/moderator/chat/:id", get(moderators::websocket_handler))
        .route("/moderator/chat/:id/image/:id", get(moderators::image))
}
