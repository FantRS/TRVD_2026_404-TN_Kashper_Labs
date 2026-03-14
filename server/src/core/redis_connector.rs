use anyhow::Result;
use deadpool_redis::{Config, Pool, Runtime};

pub type RedisPool = Pool;

pub async fn connect(redis_url: String) -> Result<RedisPool> {
    let cfg = Config::from_url(redis_url);
    let pool = cfg.create_pool(Some(Runtime::Tokio1))?;

    Ok(pool)
}
