use crate::handlers::{
    admin::{
        blacklist::*, currency::*, moderators::*, orders::TimeBounds,
        requisites::SetRequisitesDataRequest, reviews::*, social::SetSocialUrlRequest,
    },
    auth::{
        admins::*,
        users::{JwtResponse, LoginLinkResponse},
    },
    orders::*,
    requisites::Requisites,
    reviews::users::{
        AddReviewRequest, Bounds as ReviewsBounds, Review, ReviewCountResponse, VideoReview,
    },
    social::Social,
    status::users::{StatusRequest, StatusResponse, UserStatus},
    user::{Bounds, EmailForm, TopUser, TradeUrlForm, User},
};
use config::{Configuration, ConfigurationReader, EnvConfigurationReader};
use entity::{
    admin::{ActiveModel as AdminActiveModel, Column as AdminColumn, Entity as AdminEntity},
    requisites::{
        ActiveModel as RequisitesActiveModel, Column as RequisitesColumn,
        Entity as RequisitesEntity,
    },
    sea_orm_active_enums::Role,
    social::{ActiveModel as SocialActiveModel, Column as SocialColumn, Entity as SocialEntity},
};
use errors::Details;
use migration::{Migrator, MigratorTrait};
use openid::VerifyForm;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectOptions, Database, EntityTrait, QueryFilter, Set,
};
use state::AppState;
use std::{path::PathBuf, sync::Arc};
use tower_http::cors::CorsLayer;
use tracing_subscriber::layer::SubscriberExt;
use utoipa::{
    openapi::security::{ApiKey, ApiKeyValue, HttpAuthScheme, HttpBuilder, SecurityScheme},
    Modify, OpenApi,
};
use utoipa_rapidoc::RapiDoc;
use utoipa_redoc::{Redoc, Servable};
use utoipa_swagger_ui::SwaggerUi;

mod config;
mod errors;
mod extractors;
mod handlers;
mod openid;
mod services;
mod state;

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
    if !configuration.upload_folder().exists() {
        std::fs::create_dir(configuration.upload_folder()).unwrap();
    }

    if configuration.upload_folder().is_file() {
        panic!("Upload folder is a file!");
    }

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

    let openid = openid::SteamOpenId::new(configuration.realm(), "/auth/steam-success").unwrap();

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
            handlers::reviews::users::count_reviews,
            handlers::reviews::users::five_stars,
            handlers::admin::blacklist::unblacklist_user,
            handlers::admin::moderators::list_moderators,
            handlers::admin::moderators::assign_moderator,
            handlers::admin::moderators::create_moderator,
            handlers::admin::moderators::delete_moderator,
            handlers::admin::reviews::remove_video_review,
            handlers::admin::reviews::remove_video_review,
            handlers::admin::moderators::unassign_moderator,
            handlers::admin::moderators::list_moderators_orders,
            handlers::admin::moderators::list_unassigned_orders,
            handlers::admin::currency::create_currency,
            handlers::admin::currency::set_currency_rate_by_id,
            handlers::admin::currency::delete_currency_rate_by_id,
            handlers::currency::get_currency_rate_by_id,
            handlers::currency::get_currency_rates,
            handlers::orders::set_requisites,
            handlers::admin::moderators::self_info,
            handlers::admin::social::set_url,
            handlers::social::socials,
            handlers::requisites::requisites,
            handlers::admin::requisites::set_data,
            handlers::admin::reviews::update_video_review,
            handlers::admin::moderators::change_password,
            handlers::admin::moderators::chat,
            handlers::admin::moderators::send_message,
            handlers::admin::moderators::image,
            handlers::user::image,
            handlers::user::chat,
            handlers::user::send_message,
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
                    SetRateRequest, SetRequisitesRequest, ModeratorOrAdminInfo,
                    SetSocialUrlRequest, Social, SetRequisitesDataRequest, Requisites,
                    UpdateVideoReviewRequest, ChangePasswordRequest, GetChatRequest, ChatResponse,
                    SendMessageResponse, ChatHistory, ReviewsBounds, ReviewCountResponse
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
        .nest("/currency", handlers::currency::router())
        .nest("/socials", handlers::social::router())
        .nest("/requisites", handlers::requisites::router());

    let admin_login = "admin";
    let admin_password = "$argon2id$v=19$m=19456,t=2,p=1$ZKBXQv1LtIbVXKASHcIbYw$MYqUU8AI5K2OWN3b4QdkFP+g3Dh6IDnXo40EvFvYeYQ";

    if AdminEntity::find()
        .filter(AdminColumn::Login.eq(admin_login))
        .one(state.database_connection())
        .await
        .unwrap()
        .is_none()
    {
        let admin_to_be_inserted = AdminActiveModel {
            login: Set(admin_login.to_owned()),
            password: Set(admin_password.to_owned()),
            role: Set(Role::Admin),
            ..Default::default()
        };

        AdminEntity::insert(admin_to_be_inserted)
            .exec_with_returning(state.database_connection())
            .await
            .unwrap();
    }

    let requisites = [
        "Тинькофф",
        "Сбер Банк",
        "Киви",
        "Юмани",
        "Каспи Банк",
        "USDT",
    ];

    for req in requisites {
        if RequisitesEntity::find()
            .filter(RequisitesColumn::Name.eq(req))
            .one(state.database_connection())
            .await
            .unwrap()
            .is_none()
        {
            let req_to_be_inserted = RequisitesActiveModel {
                name: Set(req.to_owned()),
                ..Default::default()
            };

            req_to_be_inserted
                .insert(state.database_connection())
                .await
                .unwrap();
        }
    }

    let socials = ["Вконтакте", "Ютуб", "Телеграм"];

    for social in socials {
        if SocialEntity::find()
            .filter(SocialColumn::Name.eq(social))
            .one(state.database_connection())
            .await
            .unwrap()
            .is_none()
        {
            let social_to_be_inserted = SocialActiveModel {
                name: Set(social.to_owned()),
                ..Default::default()
            };

            social_to_be_inserted
                .insert(state.database_connection())
                .await
                .unwrap();
        }
    }

    let app = axum::Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .merge(Redoc::with_url("/redoc", ApiDoc::openapi()))
        .merge(RapiDoc::new("/api-docs/openapi.json").path("/rapidoc"))
        .nest("/api", api_router)
        .layer(CorsLayer::very_permissive())
        .with_state(Arc::new(state));

    axum::serve(listener, app).await.unwrap();
}
