mod config;
mod errors;
mod extractors;
mod handlers;
mod openid;
mod services;
mod state;

use crate::handlers::admin::blacklist::{BlacklistUserRequest, UnblacklistUserRequest};
use crate::handlers::admin::currency::SetRateRequest;
use crate::handlers::admin::currency::{CreateCurrencyRequest, Currency};
use crate::handlers::admin::moderators::{AssignModeratorRequest, UnassignModeratorRequest};
use crate::handlers::admin::moderators::{ModeratorCredentials, ModeratorResponse};
use crate::handlers::admin::orders::TimeBounds;
use crate::handlers::admin::reviews::{AddVideoReviewRequest, RemoveVideoReviewRequest};
use crate::handlers::auth::admins::{AdminLoginResponse, Credentials};
use crate::handlers::auth::users::{JwtResponse, LoginLinkResponse};
use crate::handlers::orders::*;
use crate::handlers::reviews::users::AddReviewRequest;
use crate::handlers::reviews::users::Review;
use crate::handlers::reviews::users::VideoReview;
use crate::handlers::status::users::StatusRequest;
use crate::handlers::status::users::StatusResponse;
use crate::handlers::status::users::UserStatus;
use crate::handlers::user::{Bounds, EmailForm, TopUser, TradeUrlForm, User};
use config::{Configuration, ConfigurationReader, EnvConfigurationReader};
use errors::Details;
use migration::{Migrator, MigratorTrait};
use openid::VerifyForm;
use sea_orm::{ConnectOptions, Database};
use state::AppState;
use std::path::PathBuf;
use std::sync::Arc;
use tower_http::cors::CorsLayer;
use tracing_subscriber::layer::SubscriberExt;
use utoipa::openapi::security::{ApiKey, ApiKeyValue, HttpAuthScheme, HttpBuilder, SecurityScheme};
use utoipa::{Modify, OpenApi};
use utoipa_rapidoc::RapiDoc;
use utoipa_redoc::{Redoc, Servable};
use utoipa_swagger_ui::SwaggerUi;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    //* Setting up tracing
    let stdout_log = tracing_subscriber::fmt::layer().pretty();
    let subscriber = tracing_subscriber::Registry::default()
        .with(stdout_log)
        .with(tracing_subscriber::EnvFilter::from_default_env());

    tracing::subscriber::set_global_default(subscriber).ok();

    //* Reading configuration
    let configuration: Configuration = match EnvConfigurationReader::read(None::<PathBuf>) {
        Ok(config) => config,
        Err(cause) => {
            tracing::error!(%cause);
            return;
        }
    };

    //* Connecting to redis
    let redis_client = match redis::Client::open(configuration.redis_url()) {
        Ok(client) => client,
        Err(cause) => {
            tracing::error!(%cause);
            return;
        }
    };

    //* Getting database connection and running migrations
    let mut opt = ConnectOptions::new(configuration.database_url());
    opt.sqlx_logging(configuration.sqlx_logging());

    let database_connection = Database::connect(opt).await.unwrap();
    Migrator::up(&database_connection, None).await.unwrap();

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", configuration.port()))
        .await
        .unwrap();

    let openid = openid::SteamOpenId::new(configuration.realm(), "/callback").unwrap();

    let state = AppState::new(database_connection, configuration, redis_client, openid);

    //* Setting utoipa for openapi
    #[derive(OpenApi)]
    #[openapi(
        paths(
            handlers::user::get_top,
            handlers::user::get_user,
            handlers::user::set_email,
            handlers::orders::get_order,
            handlers::auth::users::login,
            handlers::auth::admins::login,
            handlers::orders::list_orders,
            handlers::user::set_trade_url,
            handlers::orders::cancel_order,
            handlers::orders::create_order,
            handlers::auth::users::login_link,
            handlers::status::users::fetch_status,
            handlers::admin::orders::all_in_period,
            handlers::status::users::refresh_status,
            handlers::admin::blacklist::blacklist_user,
            handlers::admin::blacklist::full_blacklist,
            handlers::admin::reviews::add_video_review,
            handlers::reviews::users::add_users_review,
            handlers::admin::orders::cancel_order_by_id,
            handlers::admin::orders::finish_order_by_id,
            handlers::reviews::users::all_users_reviews,
            handlers::reviews::users::all_video_reviews,
            handlers::admin::blacklist::unblacklist_user,
            handlers::admin::moderators::list_moderators,
            handlers::admin::moderators::assign_moderator,
            handlers::admin::moderators::create_moderator,
            handlers::admin::moderators::delete_moderator,
            handlers::admin::reviews::remove_video_review,
            handlers::admin::reviews::remove_video_review,
            handlers::admin::moderators::unassign_moderator,
            handlers::admin::moderators::list_moderators_orders,
            handlers::admin::currency::create_currency,
            handlers::admin::currency::set_currency_rate_by_id,
            handlers::admin::currency::delete_currency_rate_by_id,
            handlers::currency::get_currency_rate_by_id,
            handlers::currency::get_currency_rates,
            handlers::orders::set_requisites
        ),
        components(
            schemas(
                    TimeBounds, UserStatus,
                    VerifyForm, Credentials,
                    JwtResponse, VideoReview,
                    UnassignModeratorRequest,
                    User, Order, Bounds, Review,
                    Details, TopUser, EmailForm,
                    TradeUrlForm, StatusRequest,
                    StatusResponse, AddReviewRequest,
                    LoginLinkResponse, ModeratorResponse,
                    AdminLoginResponse, CreateOrderRequest,
                    BlacklistUserRequest, ModeratorCredentials,
                    AddVideoReviewRequest, AssignModeratorRequest,
                    UnblacklistUserRequest, RemoveVideoReviewRequest,
                    Currency, CreateCurrencyRequest,
                    SetRateRequest, SetRequisitesRequest
            )
        ),
        modifiers(&SecurityAddon),
    )]
    struct ApiDoc;
    struct SecurityAddon;

    //* Adding security schemas
    impl Modify for SecurityAddon {
        fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
            if let Some(components) = openapi.components.as_mut() {
                components.add_security_scheme(
                    "jwt_user",
                    SecurityScheme::Http(
                        HttpBuilder::new()
                            .scheme(HttpAuthScheme::Bearer)
                            .bearer_format("JWT")
                            .build(),
                    ),
                );
                components.add_security_scheme(
                    "jwt_admin",
                    SecurityScheme::ApiKey(ApiKey::Header(ApiKeyValue::new("X-AM-Authorization"))),
                );
            }
        }
    }

    let api_router = axum::Router::new()
        .nest("/auth/user", handlers::auth::users::router())
        .nest("/auth/admin", handlers::auth::admins::router())
        .nest("/status", handlers::status::users::router())
        .nest("/admin", handlers::admin::router())
        .nest("/review", handlers::reviews::router())
        .nest("/user/order", handlers::orders::router())
        .nest("/user", handlers::user::router())
        .nest("/currency", handlers::currency::router());

    let app = axum::Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .merge(Redoc::with_url("/redoc", ApiDoc::openapi()))
        .merge(RapiDoc::new("/api-docs/openapi.json").path("/rapidoc"))
        .nest("/api", api_router)
        .layer(CorsLayer::very_permissive())
        .with_state(Arc::new(state));

    axum::serve(listener, app).await.unwrap();
}
