use anyhow::{Error, Result};
use sqlx::PgPool;

#[derive(Clone)]
pub struct AppData {
    pub db_pool: PgPool,
    pub jwt_secret: String,
}

impl AppData {
    pub fn builder() -> AppDataBuilder {
        AppDataBuilder::default()
    }
}

#[derive(Default)]
pub struct AppDataBuilder {
    db_pool: Option<PgPool>,
    jwt_secret: Option<String>,
}

impl AppDataBuilder {
    pub fn build(self) -> Result<AppData> {
        let app_data = AppData {
            db_pool: self
                .db_pool
                .ok_or(Error::msg("AppData building error (db_pool)"))?,
            jwt_secret: self
                .jwt_secret
                .ok_or(Error::msg("AppData building error (jwt)"))?,
        };

        Ok(app_data)
    }

    pub fn with_db_pool(mut self, db_pool: PgPool) -> Self {
        self.db_pool = Some(db_pool);
        self
    }

    pub fn with_jwt(mut self, jwt: String) -> Self {
        self.jwt_secret = Some(jwt);
        self
    }
}
