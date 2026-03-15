use argon2::Argon2;
use argon2::password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString};
use rand::rngs::OsRng;
use uuid::Uuid;

use crate::app::domains::auth::models::{
    AuthUserResponse, Claims, LoginRequestValid, LoginResponse, RegisterRequestValid,
};
use crate::app::domains::auth::repository;
use crate::app::domains::payments::repository as payments_repository;
use crate::app::redis::token_wl_service;
use crate::app::utils::jwt::create_jwt;
use crate::app::{RequestError, RequestResult, ServiceContext};
use crate::constants::wallet::SIGNUP_BONUS;

pub async fn register(
    request: RegisterRequestValid,
    ctx: &ServiceContext<'_>,
) -> RequestResult<LoginResponse> {
    let mut tx = ctx.db_pool.begin().await?;

    if repository::find_user_by_email(&request.email, &mut *tx)
        .await
        .is_ok()
    {
        return Err(RequestError::conflict(
            "user with this email already exists",
        ));
    }

    let password_hash = hash_password(&request.password)?;
    let user = repository::create_user(&request, &password_hash, &mut *tx).await?;
    payments_repository::create_wallet_transaction(
        user.id,
        None,
        "signup_bonus",
        SIGNUP_BONUS,
        0.0,
        user.wallet_balance,
        Some("Initial signup bonus"),
        &mut *tx,
    )
    .await?;
    tx.commit().await?;

    let (token, claims) = create_jwt(user.id, user.role, ctx.jwt_secret)?;
    token_wl_service::add_to_whitelist(&claims, ctx).await?;

    Ok(LoginResponse { token, user })
}

pub async fn login(
    request: LoginRequestValid,
    ctx: &ServiceContext<'_>,
) -> RequestResult<LoginResponse> {
    let user_row = repository::find_user_by_email(&request.email, ctx.db_pool).await?;

    if !user_row.is_active {
        return Err(RequestError::forbidden("user account is inactive"));
    }

    verify_password(&request.password, &user_row.password_hash)?;
    let user = AuthUserResponse::try_from(user_row)?;
    let (token, claims) = create_jwt(user.id, user.role, ctx.jwt_secret)?;
    token_wl_service::add_to_whitelist(&claims, ctx).await?;

    Ok(LoginResponse { token, user })
}

pub async fn logout(claims: &Claims, ctx: &ServiceContext<'_>) -> RequestResult<()> {
    token_wl_service::invalidate_token(&claims.jti, claims.sub, ctx).await
}

pub async fn logout_all(claims: &Claims, ctx: &ServiceContext<'_>) -> RequestResult<usize> {
    token_wl_service::invalidate_user_tokens(claims.sub, ctx).await
}

pub async fn get_current_user(
    id: Uuid,
    ctx: &ServiceContext<'_>,
) -> RequestResult<AuthUserResponse> {
    repository::find_user_by_id(id, ctx.db_pool)
        .await?
        .try_into()
}

fn hash_password(password: &str) -> RequestResult<String> {
    let salt = SaltString::generate(&mut OsRng);
    Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .map(|hash| hash.to_string())
        .map_err(|error| RequestError::internal_server_error(error.to_string()))
}

fn verify_password(password: &str, password_hash: &str) -> RequestResult<()> {
    let parsed_hash = PasswordHash::new(password_hash)
        .map_err(|error| RequestError::internal_server_error(error.to_string()))?;

    Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .map_err(|_| RequestError::unauthorized("invalid credentials"))
}
