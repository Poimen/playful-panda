use crate::{
    configuration::AppSettings,
    endpoints::redis_client::{RedisClient, RedisClientError},
};
use axum::extract::FromRef;
use std::env;

#[derive(Clone)]
pub struct AppState {
    pub settings: AppSettings,
    redis_client: RedisClient,
}

impl AppState {
    pub async fn new() -> Result<Self, String> {
        let settings = AppSettings::new(env::vars().collect())?;

        let redis_client = RedisClient::new(&settings).await.map_err(|e| match e {
            RedisClientError::ConnectionFailed(s) => s,
            RedisClientError::SetKeyFailed(s) => s,
            RedisClientError::KeyExists(s) => s,
            RedisClientError::ExpireKeyFailed(s) => s,
            RedisClientError::GetValueFromKeyFailed(s) => s,
        })?;

        Ok(AppState {
            settings,
            redis_client,
        })
    }
}

impl FromRef<AppState> for AppSettings {
    fn from_ref(app_state: &AppState) -> AppSettings {
        app_state.settings.clone()
    }
}

impl FromRef<AppState> for RedisClient {
    fn from_ref(app_state: &AppState) -> RedisClient {
        app_state.redis_client.clone()
    }
}
