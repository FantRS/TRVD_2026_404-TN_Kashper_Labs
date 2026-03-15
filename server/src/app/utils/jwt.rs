use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use uuid::Uuid;

use crate::app::domains::auth::models::{Claims, UserRole};
use crate::app::{RequestError, RequestResult};
use crate::constants::jwt::TIME_DELTA_HOURS;

pub fn create_jwt(user_id: Uuid, role: UserRole, secret: &str) -> RequestResult<(String, Claims)> {
    let jti = Uuid::new_v4().to_string();

    let expiration = Utc::now()
        .checked_add_signed(Duration::hours(TIME_DELTA_HOURS))
        .ok_or_else(|| RequestError::internal_server_error("failed to compute jwt expiration"))?
        .timestamp();

    let claims = Claims {
        sub: user_id,
        role,
        exp: expiration as usize,
        jti,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .map_err(|e| RequestError::internal_server_error(e.to_string()))?;

    Ok((token, claims))
}

pub fn decode_jwt(token: &str, secret: &str) -> RequestResult<Claims> {
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )
    .map_err(|_| RequestError::unauthorized("Invalid token"))?;

    Ok(token_data.claims)
}
