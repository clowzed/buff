use buff_notifications::configuration::reader::ConfigurationReader;
use buff_notifications::configuration::{self, Configuration};
use buff_notifications::repository::Repository;
use buff_notifications::schema::schema;

use buff_notifications::{Config, Model, ModeratorId, RedisTaskStatus, RedisTaskStatusError};
use dotenvy::dotenv;
use futures::StreamExt as _;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::sync::Arc;
use teloxide::dispatching::dialogue::serializer::Json;
use teloxide::dispatching::dialogue::SqliteStorage;
use teloxide::dispatching::Dispatcher;
use teloxide::dptree::deps;
use teloxide::requests::Requester;
use teloxide::types::ChatId;
use teloxide::Bot;
use tokio::sync::RwLock;
use tracing_subscriber::layer::SubscriberExt;

pub fn initialize_tracing() {
    dotenv().ok();

    //* Enable logging
    let stdout_log = tracing_subscriber::fmt::layer().pretty();
    let subscriber = tracing_subscriber::Registry::default()
        .with(stdout_log)
        .with(tracing_subscriber::EnvFilter::from_default_env());

    tracing::subscriber::set_global_default(subscriber).ok();
}

#[tracing::instrument(skip(configuration, bot, repository))]
async fn initialize_redis_event_listener(
    configuration: &Arc<Configuration>,
    bot: &Bot,
    repository: Arc<RwLock<Repository<ModeratorId, ChatId>>>,
) {
    let configuration_for_notification_listener_task = configuration.clone();
    let cloned_bot = bot.clone();

    let (redis_listener_task_status_sender, redis_listener_task_status_receiver) =
        async_channel::bounded(1);

    //* This task subscribes to redis channel and resend messages to
    //* corresponding moderator
    tokio::spawn(async move {
        //* Checks before connection
        let client =
            match redis::Client::open(configuration_for_notification_listener_task.redis_url()) {
                Ok(client) => client,
                Err(cause) => {
                    redis_listener_task_status_sender
                        .send(RedisTaskStatus::Failed(RedisTaskStatusError::from(cause)))
                        .await
                        .inspect_err(|_| {
                            tracing::error!("Failed to send redis task status!");
                        })
                        .ok();
                    return;
                }
            };

        //* Connecting with establishing pubsub
        let mut pubsub = match client.get_async_pubsub().await {
            Ok(pubsub) => pubsub,
            Err(cause) => {
                redis_listener_task_status_sender
                    .send(RedisTaskStatus::Failed(RedisTaskStatusError::from(cause)))
                    .await
                    .inspect_err(|_| {
                        tracing::error!("Failed to send redis task status!");
                    })
                    .ok();
                return;
            }
        };

        //? Using if let here makes reference for RedisError
        let subscribe_result = pubsub
            .subscribe(configuration_for_notification_listener_task.new_orders_channel_name())
            .await;

        //* Subscribing to redis channel
        if let Err(cause) = subscribe_result {
            redis_listener_task_status_sender
                .send(RedisTaskStatus::Failed(RedisTaskStatusError::from(cause)))
                .await
                .inspect_err(|_| {
                    tracing::error!("Failed to send redis task status!");
                })
                .ok();
            return;
        }

        //* Sending a notification about readiness
        if redis_listener_task_status_sender
            .send(RedisTaskStatus::Ready)
            .await
            .is_err()
        {
            tracing::error!("Failed to notify about task readiness!");
        }

        //? This resolves move in loop error
        let mut pubsub_into_message = pubsub.into_on_message();

        //* Start listening for new events
        while let Some(event) = pubsub_into_message.next().await {
            if let Ok(payload) = event.get_payload::<String>() {
                if let Ok(new_order) = serde_json::from_str::<Model>(&payload) {
                    if let Some(moderator_id) = new_order.moderator_id {
                        if let Some(chat_id) =
                            repository.read().await.get(ModeratorId(moderator_id)).await
                        {
                            cloned_bot
                                .send_message(chat_id, new_order.to_string())
                                .await
                                .ok(); //? In case user blocked the bot
                        }
                    }
                }
            }
        }
    });

    //* Receiving task status
    match redis_listener_task_status_receiver.recv().await {
        Ok(status) => match status {
            RedisTaskStatus::Ready => {
                tracing::info!("Redis task was successfully initialized!");
            }
            RedisTaskStatus::Failed(cause) => {
                tracing::error!(%cause, "Failed to initialize task listening for redis events!")
            }
        },
        Err(cause) => {
            tracing::error!(%cause, "Failed to receive task status for redis listening!");
            return;
        }
    };
}

#[tokio::main]
async fn main() {
    initialize_tracing();

    //* Reading configuration
    let configuration: Arc<Configuration> =
        match configuration::env::EnvConfigurationReader::read(Option::<PathBuf>::None) {
            Ok(configuration) => Arc::new(configuration),
            Err(cause) => {
                tracing::error!(%cause, "Failed to read configuration!");
                return;
            }
        };

    if !configuration.repository_storage().exists() {
        let mut repository_storage = match File::create(configuration.repository_storage()) {
            Ok(file) => file,
            Err(cause) => {
                tracing::error!(%cause, "Failed to create repository storage!");
                return;
            }
        };
        if let Err(cause) = write!(repository_storage, "{{}}") {
            tracing::error!(%cause, "Failed to write empty brackets to repository storage!");
            return;
        }
    }

    //* Initialize repository
    let repository: Arc<RwLock<Repository<ModeratorId, ChatId>>> =
        match Repository::from_path(configuration.repository_storage()) {
            Ok(repository) => Arc::new(RwLock::new(repository)),
            Err(cause) => {
                tracing::error!(%cause, "Failed to initialize repository!");
                return;
            }
        };

    let bot = Bot::new(configuration.bot_token());

    //* Initialize task which listens to redis channel
    initialize_redis_event_listener(&configuration, &bot, repository.clone()).await;

    //* SqliteStorage requires str for file path
    let storage_filepath = match configuration.states_storage().to_str() {
        Some(path) => path,
        None => {
            tracing::error!("Failed to convert repository file path to string");
            return;
        }
    };

    //* This will prevent bot from erazing states of users on reboot or shutdown
    let states_storage = match SqliteStorage::open(storage_filepath, Json).await {
        Ok(redis_storage) => redis_storage,
        Err(cause) => {
            tracing::error!(%cause, "Failed to initialize redis storage for saving user state");
            return;
        }
    };

    let config = Arc::new(Config::new(
        configuration.admin_id(),
        configuration.site_url(),
    ));

    Dispatcher::builder(bot, schema())
        .dependencies(deps![states_storage, repository, config])
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}
