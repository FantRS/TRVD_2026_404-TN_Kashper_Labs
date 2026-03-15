use anyhow::Result;
use std::net::TcpListener;

use crate::app::redis::client::RedisClient;
use crate::core::app_data::AppData;
use crate::core::config::AppConfig;
use crate::core::logger::{self, LogLevel};
use crate::core::server;
use crate::core::{pg_connector, redis_connector};

pub async fn start() -> Result<()> {
    dotenvy::dotenv().ok();
    logger::init_logger(LogLevel::Info);

    let config = AppConfig::configure()?;
    let lst = TcpListener::bind(config.server.addr())?;
    let db_pool = pg_connector::connect(config.postgres.options()).await?;
    let redis_pool = redis_connector::connect(config.redis.addr()).await?;
    let redis_client = RedisClient::new(redis_pool);

    let app_data = AppData::builder()
        .with_db_pool(db_pool)
        .with_redis_client(redis_client)
        .with_jwt(config.jwt_secret)
        .with_business_hours(config.business_hours)
        .build()?;

    server::start(lst, app_data).await
}
