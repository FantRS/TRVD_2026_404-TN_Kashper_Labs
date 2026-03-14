use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_with::{DisplayFromStr, serde_as};
use sqlx::postgres::PgConnectOptions;

#[derive(Debug, Deserialize, Serialize)]
pub struct AppConfig {
    #[serde(flatten)]
    pub server: ServerSettings,
    #[serde(flatten)]
    pub postgres: PostgresSettings,
    pub jwt_secret: String,
}

impl AppConfig {
    pub fn configure() -> Result<Self> {
        let config = config::Config::builder()
            .add_source(config::Environment::default())
            .build()?;

        config.try_deserialize().map_err(From::from)
    }
}

#[serde_as]
#[derive(Debug, Deserialize, Serialize)]
pub struct ServerSettings {
    #[serde(rename = "server_host")]
    pub host: String,

    #[serde_as(as = "DisplayFromStr")]
    #[serde(rename = "server_port")]
    pub port: u16,
}

impl ServerSettings {
    pub fn addr(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}

#[serde_as]
#[derive(Debug, Deserialize, Serialize)]
pub struct PostgresSettings {
    #[serde(rename = "postgres_user")]
    user: String,

    #[serde(rename = "postgres_password")]
    password: String,

    #[serde(rename = "postgres_host")]
    host: String,

    #[serde_as(as = "DisplayFromStr")]
    #[serde(rename = "postgres_port")]
    port: u16,

    #[serde(rename = "postgres_db")]
    db_name: String,
}

impl PostgresSettings {
    pub fn options(&self) -> PgConnectOptions {
        PgConnectOptions::new()
            .username(&self.user)
            .password(&self.password)
            .host(&self.host)
            .port(self.port)
            .database(&self.db_name)
    }
}
