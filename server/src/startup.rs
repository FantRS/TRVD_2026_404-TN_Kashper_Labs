use std::net::TcpListener;

use anyhow::Result;

use crate::core::logger::{self, LogLevel};

pub async fn start() -> Result<()> {
    logger::init_logger(LogLevel::Info);

    let lst = TcpListener::bind(("127.0.0.1", 8080)).unwrap();

    Ok(())
}
