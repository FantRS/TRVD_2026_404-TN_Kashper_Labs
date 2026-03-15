use serde::Deserialize;
use utoipa::{IntoParams, ToSchema};

use crate::app::domains::auth::models::UserRole;

#[derive(Debug, Deserialize, ToSchema)]
pub struct UserRoleUpdateRequest {
    pub role: UserRole,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UserBlockRequest {
    pub is_active: bool,
}

#[derive(Debug, Clone, Deserialize, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct UsersFilterParams {
    pub page: Option<u32>,
    pub per_page: Option<u32>,
    pub search: Option<String>,
    pub role: Option<UserRole>,
    pub is_active: Option<bool>,
}

impl UsersFilterParams {
    pub fn page(&self) -> u32 {
        self.page.unwrap_or(1)
    }

    pub fn per_page(&self) -> u32 {
        self.per_page.unwrap_or(20)
    }
}
