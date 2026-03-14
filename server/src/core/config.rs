use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_with::{DisplayFromStr, serde_as};

#[derive(Debug, Deserialize, Serialize)]
pub struct AppConfig {
    #[serde(flatten)]
    pub server: ServerSettings,
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
