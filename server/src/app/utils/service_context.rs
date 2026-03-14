use sqlx::PgPool;

use crate::app::AppData;
use crate::app::redis::client::RedisClient;

pub struct ServiceContext<'a> {
    pub db_pool: &'a PgPool,
    pub redis: &'a RedisClient,
    pub jwt_secret: &'a str,
}

impl<'a> From<&'a AppData> for ServiceContext<'a> {
    fn from(value: &'a AppData) -> Self {
        Self {
            db_pool: &value.db_pool,
            redis: &value.redis,
            jwt_secret: &value.jwt_secret,
        }
    }
}
