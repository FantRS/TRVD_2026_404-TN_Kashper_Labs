use chrono::Utc;
use uuid::Uuid;

use super::keys::RedisKey;

use crate::app::domains::auth::models::Claims;
use crate::app::redis::client::RedisClient;
use crate::app::ServiceContext;
use crate::app::{RequestError, RequestResult};

/// Додати токен до whitelist
pub async fn add_to_whitelist(claims: &Claims, ctx: &ServiceContext<'_>) -> RequestResult<()> {
    let ttl_secs = claims.exp as i64 - Utc::now().timestamp();
    if ttl_secs <= 0 {
        return Err(RequestError::internal_server_error("Token already expired"));
    }
    let ttl = ttl_secs as usize;

    let token_key = RedisKey::WhiteList(claims.jti.clone()).to_string();
    let user_key = RedisKey::UserTokens(claims.sub).to_string();
    let value = claims.sub.to_string();

    let mut pipeline = ctx.redis.get_pipe();
    pipeline
        .set_ex(&token_key, value, ttl as u64)
        .sadd(&user_key, &claims.jti)
        .expire(&user_key, ttl as i64);
    ctx.redis.exec_pipe::<()>(&pipeline).await?;

    Ok(())
}

/// Перевірити, чи токен у whitelist
pub async fn verify_in_whitelist(claims: &Claims, redis: &RedisClient) -> RequestResult<bool> {
    let key = RedisKey::WhiteList(claims.jti.clone()).to_string();
    let stored: Option<String> = redis.get(&key).await?;
    Ok(stored.is_some())
}

/// Інвалідувати всі токени користувача
pub async fn invalidate_user_tokens(
    user_id: Uuid,
    ctx: &ServiceContext<'_>,
) -> RequestResult<usize> {
    let user_key = RedisKey::UserTokens(user_id).to_string();

    let jtis: Vec<String> = ctx.redis.smembers(&user_key).await?;
    let count = jtis.len();

    if !jtis.is_empty() {
        let mut keys_to_delete: Vec<String> = jtis
            .iter()
            .map(|jti| RedisKey::WhiteList(jti.clone()).to_string())
            .collect();
        keys_to_delete.push(user_key);

        let mut pipeline = ctx.redis.get_pipe();
        for key in &keys_to_delete {
            pipeline.del(key);
        }
        ctx.redis.exec_pipe::<()>(&pipeline).await?;
    }

    tracing::info!("Invalidated {} tokens for user {}", count, user_id);

    Ok(count)
}

/// Інвалідувати конкретний токен
pub async fn invalidate_token(
    jti: &str,
    user_id: Uuid,
    ctx: &ServiceContext<'_>,
) -> RequestResult<()> {
    let key = RedisKey::WhiteList(jti.to_owned()).to_string();
    let user_key = RedisKey::UserTokens(user_id).to_string();

    let mut pipeline = ctx.redis.get_pipe();
    pipeline.del(&key).srem(&user_key, jti);
    ctx.redis.exec_pipe::<()>(&pipeline).await?;

    tracing::info!("Invalidated token {} for user {}", jti, user_id);

    Ok(())
}
