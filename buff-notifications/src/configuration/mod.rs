use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use teloxide::types::ChatId;
use url::Url;

pub mod env;
pub mod reader;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Configuration {
    admin_id: ChatId,
    redis_url: String,
    new_orders_channel_name: String,
    bot_token: String,
    repository_storage: PathBuf,
    states_storage: PathBuf,
    site_url: Url,
}

impl Configuration {
    pub fn admin_id(&self) -> ChatId {
        self.admin_id
    }

    pub fn redis_url(&self) -> &str {
        &self.redis_url
    }

    pub fn new_orders_channel_name(&self) -> &str {
        &self.new_orders_channel_name
    }

    pub fn bot_token(&self) -> &str {
        &self.bot_token
    }

    pub fn repository_storage(&self) -> &PathBuf {
        &self.repository_storage
    }

    pub fn states_storage(&self) -> &PathBuf {
        &self.states_storage
    }

    pub fn site_url(&self) -> Url {
        self.site_url.clone()
    }
}
