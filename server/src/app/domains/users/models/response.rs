use serde::Serialize;
use sqlx::FromRow;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::app::RequestError;
use crate::app::domains::auth::models::UserRole;

#[derive(Debug, Serialize, ToSchema)]
pub struct UserAdminResponse {
    pub id: Uuid,
    pub email: String,
    pub full_name: String,
    pub phone: Option<String>,
    pub wallet_balance: f64,
    pub role: UserRole,
    pub is_active: bool,
}

#[derive(Debug, FromRow)]
pub struct UserAdminRow {
    pub id: Uuid,
    pub email: String,
    pub full_name: String,
    pub phone: Option<String>,
    pub wallet_balance: f64,
    pub role_code: String,
    pub is_active: bool,
    pub total_count: Option<i64>,
}

impl TryFrom<UserAdminRow> for UserAdminResponse {
    type Error = RequestError;

    fn try_from(value: UserAdminRow) -> Result<Self, Self::Error> {
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
