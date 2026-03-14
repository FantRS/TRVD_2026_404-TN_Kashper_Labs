use std::time::Duration;

use anyhow::Result;
use sqlx::{
    postgres::{PgConnectOptions, PgPoolOptions},
    PgPool,
};

pub async fn connect(options: PgConnectOptions) -> Result<PgPool> {
    tracing::info!("establishing database connection");

    let pool = PgPoolOptions::new()
        .min_connections(5)
        .max_connections(20)
        .acquire_timeout(Duration::from_secs(30))
        .idle_timeout(Duration::from_secs(600))
        .max_lifetime(Duration::from_secs(1800))
        .connect_with(options)
        .await?;

    tracing::info!("connection pool created successfully");

    let is_run_migrate: bool = std::env::var("MIGRATE_RUN")
        .unwrap_or("false".into())
        .parse()?;

    if is_run_migrate {
        tracing::info!("running migrations");
        sqlx::migrate!("./migrations").run(&pool).await?;
    }

    Ok(pool)
}
