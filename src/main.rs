use crate::handlers::{admin::moderators::*, orders::*};
use axum::extract::DefaultBodyLimit;
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
use utoipauto::utoipauto;

use migration::{Migrator, MigratorTrait};
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

    //? I will leave it here for testing notifications
    /*
    let cloned_redis = redis_client.clone();
    tokio::spawn(async move {
        let mut connection = cloned_redis.get_async_connection().await.unwrap();
        loop {
            let order = OrderModel {
                id: 1,
                payment_method: "СБП".to_owned(),
                status: entity::sea_orm_active_enums::Status::Created,
                requisites_id: 1,
                created_at: Default::default(),
                finished_at: None,
                steam_id: 1234565,
                moderator_id: Some(1),
                amount: Decimal::new(3141, 3),
                fixed_currency_rate: Decimal::new(3141, 3),
                currency_symbol: "R".to_owned(),
            };

            let _: Result<(), _> = connection
                .publish(
                    "new_orders_notifications",
                    serde_json::to_string(&order).unwrap(),
                )
                .await;

            tokio::time::sleep(Duration::from_secs(2)).await;
        }
    }); */

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
    #[utoipauto]
    #[derive(OpenApi)]
    #[openapi(
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
        .layer(CorsLayer::permissive())
        .layer(DefaultBodyLimit::max(10 * 1024 * 1024)) //10 mb
        .with_state(Arc::new(state));

    axum::serve(listener, app).await.unwrap();
}
