use deadpool_redis::{redis::AsyncCommands, Config as RedisConfig, Pool, PoolError};
use std::fmt::Display;
use uuid::Uuid;

pub enum RedisKey {
    FeatureStore,
}

impl Display for RedisKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            RedisKey::FeatureStore => f.write_str("feature_store"),
        }
    }
}

#[derive(Clone)]
pub struct RedisBackend {
    pool: Pool,
}

impl RedisBackend {
    /**
    This will create a connection to the backend on the given host.
    */
    pub fn new(host: &String) -> Self {
        let redis_config = RedisConfig {
            url: Some(format!("redis://{}", host)),
            pool: None,
        };
        let redis_pool = redis_config.create_pool().unwrap();

        Self { pool: redis_pool }
    }

    pub async fn get_flag(&self, key: String, tenant_id: Uuid) -> Result<bool, PoolError> {
        let mut conn = self.pool.get().await?;
        let val: bool = conn.hget(Self::get_flags_id(tenant_id), key).await?;

        Ok(val)
    }

    pub async fn add_or_update_flag(
        &self,
        key: String,
        val: bool,
        tenant_id: Uuid,
    ) -> Result<(), PoolError> {
        let mut conn = self.pool.get().await?;

        conn.hset(Self::get_flags_id(tenant_id), key, val).await?;

        Ok(())
    }

    pub async fn remove_flag(&self, key: String, tenant_id: Uuid) -> Result<(), PoolError> {
        let mut conn = self.pool.get().await?;

	conn.hdel(Self::get_flags_id(tenant_id), key).await?;

        Ok(())
    }

    /// Get all the key values flag pairs for the given tenant.
    pub async fn get_all_flags(&self, tenant_id: Uuid) -> Result<Vec<(String, bool)>, PoolError> {
        let mut conn = self.pool.get().await?;
        let kvs: Vec<(String, bool)> = conn.hgetall(Self::get_flags_id(tenant_id)).await?;

        Ok(kvs)
    }

    fn get_flags_id(tenant_id: Uuid) -> String {
        format!("{}:{}", RedisKey::FeatureStore.to_string(), tenant_id)
    }
}
