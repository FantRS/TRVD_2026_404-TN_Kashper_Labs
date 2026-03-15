use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::FromRow;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Serialize, ToSchema)]
pub struct AvailableSlotResponse {
    pub scheduled_at: String,
    pub is_available: bool,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct AppointmentResponse {
    pub id: Uuid,
    pub order_id: Uuid,
    pub employee_user_id: Option<Uuid>,
    pub scheduled_at: String,
    pub location: String,
    pub appointment_status: String,
}

#[derive(Debug, FromRow)]
pub struct AppointmentRow {
    pub id: Uuid,
    pub order_id: Uuid,
    pub employee_user_id: Option<Uuid>,
    pub scheduled_at: DateTime<Utc>,
    pub location: String,
    pub appointment_status: String,
}

impl From<AppointmentRow> for AppointmentResponse {
    fn from(value: AppointmentRow) -> Self {
        Self {
            id: value.id,
            order_id: value.order_id,
            employee_user_id: value.employee_user_id,
            scheduled_at: value.scheduled_at.to_rfc3339(),
            location: value.location,
            appointment_status: value.appointment_status,
        }
    }
}
