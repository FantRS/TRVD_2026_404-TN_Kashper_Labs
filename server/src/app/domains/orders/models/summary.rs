use serde::Serialize;
use sqlx::FromRow;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Serialize, ToSchema, FromRow)]
pub struct OrderSummary {
    pub id: Uuid,
    pub order_number: String,
    pub current_status_code: String,
    pub total_amount: f64,
}
