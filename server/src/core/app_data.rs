use anyhow::{Error, Result};
use sqlx::PgPool;

use crate::app::redis::client::RedisClient;
use crate::core::config::BusinessHoursSettings;

#[derive(Clone)]
pub struct AppData {
    pub db_pool: PgPool,
    pub redis: RedisClient,
    pub jwt_secret: String,
    pub business_hours: BusinessHoursSettings,
}

impl AppData {
    pub fn builder() -> AppDataBuilder {
        AppDataBuilder::default()
    }
}

#[derive(Default)]
pub struct AppDataBuilder {
    db_pool: Option<PgPool>,
    redis: Option<RedisClient>,
    jwt_secret: Option<String>,
    business_hours: Option<BusinessHoursSettings>,
}

impl AppDataBuilder {
    pub fn build(self) -> Result<AppData> {
        let app_data = AppData {
            db_pool: self
                .db_pool
                .ok_or(Error::msg("AppData building error (db_pool)"))?,
            redis: self
                .redis
                .ok_or(Error::msg("AppData building error (db_pool)"))?,
            jwt_secret: self
                .jwt_secret
                .ok_or(Error::msg("AppData building error (jwt)"))?,
            business_hours: self
                .business_hours
                .ok_or(Error::msg("AppData building error (business_hours)"))?,
        };

        Ok(app_data)
    }

    pub fn with_db_pool(mut self, db_pool: PgPool) -> Self {
        self.db_pool = Some(db_pool);
        self
    }

    pub fn with_redis_client(mut self, redis_client: RedisClient) -> Self {
        self.redis = Some(redis_client);
        self
    }

    pub fn with_jwt(mut self, jwt: String) -> Self {
        self.jwt_secret = Some(jwt);
        self
    }

    pub fn with_business_hours(mut self, business_hours: BusinessHoursSettings) -> Self {
        self.business_hours = Some(business_hours);
        self
    }
}
