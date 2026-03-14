use serde::{Deserialize, Serialize};
use std::fmt;
use utoipa::ToSchema;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize, ToSchema)]
#[serde(rename_all = "lowercase")]
pub enum UserRole {
    User,
    Employee,
    Admin,
}

impl UserRole {
    pub fn has_role(&self, role: UserRole) -> bool {
        *self == role
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            UserRole::User => "user",
            UserRole::Employee => "employee",
            UserRole::Admin => "admin",
        }
    }
}

impl fmt::Display for UserRole {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl TryFrom<&str> for UserRole {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "user" => Ok(UserRole::User),
            "employee" => Ok(UserRole::Employee),
            "admin" => Ok(UserRole::Admin),
            _ => Err(format!("Unknown role: {}", value)),
        }
    }
}

impl From<UserRole> for String {
    fn from(role: UserRole) -> Self {
        role.as_str().to_string()
    }
}
