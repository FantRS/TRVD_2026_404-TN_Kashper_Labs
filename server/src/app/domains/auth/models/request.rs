use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use super::UserRole;
use crate::app::RequestError;
use crate::app::utils::validation::{normalized_email, phone_number, trimmed_required};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: Uuid,      // User ID
    pub role: UserRole, // User role
    pub exp: usize,     // Expiration time
    pub jti: String,    // JWT ID (UUID v4)
}

impl Claims {
    /// Перевірити чи є у користувача одна з вказаних ролей
    pub fn has_any_role(&self, roles: &[UserRole]) -> bool {
        roles.contains(&self.role)
    }

    /// Перевірити наявність конкретної ролі
    pub fn has_role(&self, role: UserRole) -> bool {
        self.role.has_role(role)
    }
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct RegisterRequest {
    pub email: String,
    pub password: String,
    pub full_name: String,
    pub phone: Option<String>,
}

#[derive(Debug)]
pub struct RegisterRequestValid {
    pub email: String,
    pub password: String,
    pub full_name: String,
    pub phone: Option<String>,
}

impl TryFrom<RegisterRequest> for RegisterRequestValid {
    type Error = RequestError;

    fn try_from(value: RegisterRequest) -> Result<Self, Self::Error> {
        let password = value.password.trim().to_owned();
        if password.len() < 8 || password.len() > 128 {
            return Err(RequestError::unprocessable_entity(
                "password must contain from 8 to 128 characters",
            ));
        }

        Ok(Self {
            email: normalized_email(&value.email, "email")?,
            password,
            full_name: trimmed_required(&value.full_name, "full_name", 3, 255)?,
            phone: match value.phone {
                Some(phone) => Some(phone_number(&phone, "phone")?),
                None => None,
            },
        })
    }
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug)]
pub struct LoginRequestValid {
    pub email: String,
    pub password: String,
}

impl TryFrom<LoginRequest> for LoginRequestValid {
    type Error = RequestError;

    fn try_from(value: LoginRequest) -> Result<Self, Self::Error> {
        let password = value.password.trim().to_owned();
        if password.len() < 8 || password.len() > 128 {
            return Err(RequestError::unprocessable_entity(
                "password must contain from 8 to 128 characters",
            ));
        }

        Ok(Self {
            email: normalized_email(&value.email, "email")?,
            password,
        })
    }
}
