use anyhow::Result;
use std::net::TcpListener;

use crate::core::config::AppConfig;
use crate::core::logger::{self, LogLevel};
use crate::core::server;

pub async fn start() -> Result<()> {
    dotenvy::dotenv().ok();
    logger::init_logger(LogLevel::Info);

    let config = AppConfig::configure().unwrap();
    let lst = TcpListener::bind(config.server.addr()).unwrap();

    server::start(lst).await
}
