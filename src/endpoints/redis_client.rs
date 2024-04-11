use crate::configuration::AppSettings;
use redis::aio::MultiplexedConnection;

#[derive(Debug)]
pub enum RedisClientError {
    ConnectionFailed(String),
    SetKeyFailed(String),
    KeyExists(String),
    ExpireKeyFailed(String),
    GetValueFromKeyFailed(String),
}

#[derive(Debug, Clone)]
pub struct RedisClient {
    pub connection: MultiplexedConnection,
}

impl RedisClient {
    pub async fn new(settings: &AppSettings) -> Result<Self, RedisClientError> {
        let client = match redis::Client::open(settings.redis.server.as_str()) {
            Err(e) => {
                return Err(RedisClientError::ConnectionFailed(format!("{e}")));
            }
            Ok(c) => c,
        };

        let connection = match client.get_multiplexed_async_connection().await {
            Err(e) => {
                return Err(RedisClientError::ConnectionFailed(format!("{e}")));
            }
            Ok(c) => c,
        };

        Ok(RedisClient { connection })
    }

    pub async fn set_if_not_exists(
        &self,
        key: &String,
        value: &String,
        seconds: Option<u64>,
    ) -> Result<(), RedisClientError> {
        let mut connection = self.connection.clone();

        self.exists(&mut connection, key).await?;

        self.set_key(&mut connection, key, value).await?;

        self.expire_key(&mut connection, key, seconds).await?;

        Ok(())
    }

    pub async fn get(&self, key: &String) -> Result<String, RedisClientError> {
        let mut connection = self.connection.clone();

        self.get_value(&mut connection, key).await
    }

    async fn exists(
        &self,
        connection: &mut MultiplexedConnection,
        key: &String,
    ) -> Result<(), RedisClientError> {
        match redis::cmd("EXISTS")
            .arg(key)
            .query_async::<MultiplexedConnection, bool>(connection)
            .await
        {
            Err(e) => {
                Err(RedisClientError::KeyExists(format!("{e}")))
            }
            Ok(_) => Ok(()),
        }
    }

    async fn set_key(
        &self,
        connection: &mut MultiplexedConnection,
        key: &String,
        value: &String,
    ) -> Result<(), RedisClientError> {
        match redis::cmd("SET")
            .arg(&[&key, &value])
            .query_async::<MultiplexedConnection, bool>(connection)
            .await
        {
            Err(e) => {
                Err(RedisClientError::SetKeyFailed(format!("{e}")))
            }
            Ok(_) => Ok(()),
        }
    }

    async fn expire_key(
        &self,
        connection: &mut MultiplexedConnection,
        key: &String,
        seconds: Option<u64>,
    ) -> Result<(), RedisClientError> {
        if seconds.is_none() {
            return Ok(());
        }

        match redis::cmd("EXPIRE")
            .arg(key)
            .arg(seconds.unwrap())
            .query_async::<MultiplexedConnection, bool>(connection)
            .await
        {
            Err(e) => {
                Err(RedisClientError::ExpireKeyFailed(format!("{e}")))
            }
            Ok(_) => Ok(()),
        }
    }

    async fn get_value(
        &self,
        connection: &mut MultiplexedConnection,
        key: &String,
    ) -> Result<String, RedisClientError> {
        match redis::cmd("GET")
            .arg(key)
            .query_async::<MultiplexedConnection, String>(connection)
            .await
        {
            Err(e) => {
                Err(RedisClientError::GetValueFromKeyFailed(format!("{e}")))
            }
            Ok(url) => Ok(url),
        }
    }
}
