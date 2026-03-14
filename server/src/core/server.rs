use actix_cors::Cors;
use actix_web::middleware::NormalizePath;
use actix_web::{App, HttpServer};
use anyhow::Result;
use std::net::TcpListener;
use std::time::Duration;
use tracing_actix_web::TracingLogger;

pub async fn start(lst: TcpListener) -> Result<()> {
    tracing::info!("Starting server...");

    HttpServer::new(move || {
        App::new()
            .wrap(configure_cors())
            .wrap(NormalizePath::trim())
            .wrap(TracingLogger::default())
    })
    .keep_alive(Duration::from_secs(75))
    .listen(lst)?
    .run()
    .await?;

    Ok(())
}

fn configure_cors() -> Cors {
    Cors::default()
        .allow_any_origin()
        .allow_any_method()
        .allow_any_header()
        .max_age(3600)
}
