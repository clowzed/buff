use sea_orm::DatabaseConnection;

use crate::config::Configuration;
use crate::openid::SteamOpenId;

#[derive(Clone)]
pub struct AppState {
    database_connection: DatabaseConnection,
    configuration: Configuration,
    redis_client: redis::Client,
    steam_openid: SteamOpenId,
}

impl AppState {
    pub fn new(
        database_connection: DatabaseConnection,
        configuration: Configuration,
        redis_client: redis::Client,
        steam_openid: SteamOpenId,
    ) -> Self {
        Self {
            database_connection,
            configuration,
            redis_client,
            steam_openid,
        }
    }

    pub fn database_connection(&self) -> &DatabaseConnection {
        &self.database_connection
    }

    pub fn configuration(&self) -> &Configuration {
        &self.configuration
    }

    pub fn redis_client(&self) -> &redis::Client {
        &self.redis_client
    }

    pub fn steam_openid(&self) -> &SteamOpenId {
        &self.steam_openid
    }
}
