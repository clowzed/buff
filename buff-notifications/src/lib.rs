use chrono::NaiveDateTime as DateTime;
use redis::RedisError;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use state::State;
use std::fmt::Display;
use teloxide::{
    dispatching::dialogue::{serializer::Json, Dialogue, SqliteStorage},
    types::ChatId,
};
use url::Url;

pub mod apiservice;
pub mod commands;
pub mod configuration;
pub mod handlers;
pub mod repository;
pub mod schema;
pub mod state;

type MyDialogue = Dialogue<State, SqliteStorage<Json>>;
type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;

#[derive(thiserror::Error, Debug)]
pub enum RedisTaskStatusError {
    #[error(transparent)]
    RedisError(#[from] RedisError),
}

pub enum RedisTaskStatus {
    Ready,
    Failed(RedisTaskStatusError),
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Hash, Clone)]
pub struct ModeratorId(pub i64);

pub struct Config {
    admin_id: ChatId,
    server_url: Url,
}

impl Config {
    pub fn new(admin_id: ChatId, server_url: Url) -> Self {
        Self {
            admin_id,
            server_url,
        }
    }

    pub fn admin_id(&self) -> ChatId {
        self.admin_id
    }

    pub fn server_url(&self) -> Url {
        self.server_url.clone()
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Model {
    pub id: i64,
    pub payment_method: String,
    pub status: String,
    pub requisites_id: i64,
    pub created_at: DateTime,
    pub finished_at: Option<DateTime>,
    pub steam_id: i64,
    pub moderator_id: Option<i64>,
    pub amount: Decimal,
    pub fixed_currency_rate: Decimal,
    pub currency_symbol: String,
}

impl Display for Model {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let id = self.id;
        let amount = self.amount;
        let currency_symbol = &self.currency_symbol;
        let payment_method = &self.payment_method;
        let fixed_currency_rate = self.fixed_currency_rate;

        write!(
            f,
            "✉️ - Новое сообщение!\n📕  - Номер заказа: {id}\n💰  - {amount}{currency_symbol}\n💳  - Способ оплаты: {payment_method}\n💴  - Курс: {fixed_currency_rate}"
        )
    }
}
