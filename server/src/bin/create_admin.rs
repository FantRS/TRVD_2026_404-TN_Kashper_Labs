use std::collections::HashMap;
use std::process::ExitCode;

use anyhow::{Context, Result, anyhow, bail};
use argon2::Argon2;
use argon2::password_hash::{PasswordHasher, SaltString};
use rand::rngs::OsRng;
use server::core::config::AppConfig;
use server::core::logger::{self, LogLevel};
use server::core::pg_connector;
use sqlx::Row;

#[tokio::main]
async fn main() -> ExitCode {
    if let Err(error) = run().await {
        eprintln!("create_admin failed: {error}");
        return ExitCode::FAILURE;
    }

    ExitCode::SUCCESS
}

async fn run() -> Result<()> {
    dotenvy::dotenv().ok();
    logger::init_logger(LogLevel::Info);

    let args = parse_args(std::env::args().skip(1))?;
    let email = required_arg(&args, "email")?;
    let password = required_arg(&args, "password")?;
    let full_name = required_arg(&args, "full-name")?;
    let phone = optional_arg(&args, "phone");

    validate_inputs(email, password, full_name)?;

    let config = AppConfig::configure()?;
    let pool = pg_connector::connect(config.postgres.options()).await?;

    if user_exists(email, &pool).await? {
        bail!("user with email '{email}' already exists");
    }

    let password_hash = hash_password(password)?;
    let created = sqlx::query(
        r#"
        INSERT INTO users (
            role_id,
            email,
            password_hash,
            full_name,
            phone
        )
        VALUES (
            (SELECT id FROM roles WHERE code = 'admin'),
            $1,
            $2,
            $3,
            $4
        )
        RETURNING id, wallet_balance::DOUBLE PRECISION AS wallet_balance
        "#,
    )
    .bind(email)
    .bind(password_hash)
    .bind(full_name)
    .bind(phone)
    .fetch_one(&pool)
    .await
    .context("failed to insert admin user")?;

    let id: uuid::Uuid = created.get("id");
    let wallet_balance: f64 = created.get("wallet_balance");

    println!("admin account created successfully");
    println!("id: {id}");
    println!("email: {email}");
    println!("full_name: {full_name}");
    println!("wallet_balance: {wallet_balance}");

    Ok(())
}

fn parse_args(args: impl Iterator<Item = String>) -> Result<HashMap<String, String>> {
    let mut parsed = HashMap::new();
    let mut pending_key: Option<String> = None;

    for arg in args {
        if let Some(key) = pending_key.take() {
            parsed.insert(key, arg);
            continue;
        }

        if arg == "--help" || arg == "-h" {
            print_usage();
            std::process::exit(0);
        }

        if let Some(key) = arg.strip_prefix("--") {
            pending_key = Some(key.to_owned());
            continue;
        }

        bail!("unexpected argument '{arg}'");
    }

    if let Some(key) = pending_key {
        bail!("missing value for '--{key}'");
    }

    Ok(parsed)
}

fn required_arg<'a>(args: &'a HashMap<String, String>, key: &str) -> Result<&'a str> {
    args.get(key)
        .map(String::as_str)
        .ok_or_else(|| anyhow!("missing required argument '--{key}'"))
}

fn optional_arg<'a>(args: &'a HashMap<String, String>, key: &str) -> Option<&'a str> {
    args.get(key).map(String::as_str).filter(|value| !value.trim().is_empty())
}

fn validate_inputs(email: &str, password: &str, full_name: &str) -> Result<()> {
    let email = email.trim();
    let full_name = full_name.trim();

    if email.len() < 3 || !email.contains('@') {
        bail!("email must be valid");
    }

    if password.trim().len() < 8 || password.trim().len() > 128 {
        bail!("password must contain from 8 to 128 characters");
    }

    if full_name.len() < 3 || full_name.len() > 255 {
        bail!("full-name must contain from 3 to 255 characters");
    }

    Ok(())
}

fn hash_password(password: &str) -> Result<String> {
    let salt = SaltString::generate(&mut OsRng);

    Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .map(|hash| hash.to_string())
        .map_err(|error| anyhow!(error.to_string()))
}

async fn user_exists(email: &str, pool: &sqlx::PgPool) -> Result<bool> {
    let existing = sqlx::query_scalar::<_, i64>(
        r#"
        SELECT COUNT(*)
        FROM users
        WHERE email = $1
        "#,
    )
    .bind(email)
    .fetch_one(pool)
    .await?;

    Ok(existing > 0)
}

fn print_usage() {
    println!(
        "Usage:\n  cargo run --bin create_admin -- --email admin@example.com --password secret123 --full-name \"Admin User\" [--phone \"+380...\"]"
    );
}
