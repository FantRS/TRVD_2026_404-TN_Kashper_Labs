use serde::Serialize;
use sqlx::FromRow;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::app::RequestError;
use crate::app::domains::auth::models::UserRole;

#[derive(Debug, Serialize, ToSchema)]
pub struct AuthUserResponse {
    pub id: Uuid,
    pub email: String,
    pub full_name: String,
    pub phone: Option<String>,
    pub wallet_balance: f64,
    pub role: UserRole,
    pub is_active: bool,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct CurrentUserResponse {
    pub user: AuthUserResponse,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct LoginResponse {
    pub token: String,
    pub user: AuthUserResponse,
}

#[derive(Debug, FromRow)]
pub struct UserRow {
    pub id: Uuid,
    pub email: String,
    pub password_hash: String,
    pub full_name: String,
    pub phone: Option<String>,
    pub wallet_balance: f64,
    pub role_id: Uuid,
    pub role_code: String,
    pub is_active: bool,
}

impl TryFrom<UserRow> for AuthUserResponse {
    type Error = RequestError;

    fn try_from(value: UserRow) -> Result<Self, Self::Error> {
        Ok(Self {
            id: value.id,
            email: value.email,
            full_name: value.full_name,
            phone: value.phone,
            wallet_balance: value.wallet_balance,
            role: UserRole::try_from(value.role_code.as_str())
                .map_err(RequestError::unprocessable_entity)?,
            is_active: value.is_active,
        })
    }
}
