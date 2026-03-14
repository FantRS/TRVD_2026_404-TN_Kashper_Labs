use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::UserRole;

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
