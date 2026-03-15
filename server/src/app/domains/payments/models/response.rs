use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::FromRow;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Serialize, ToSchema)]
pub struct PaymentResponse {
    pub id: Uuid,
    pub order_id: Uuid,
    pub user_id: Uuid,
    pub payment_method: String,
    pub payment_status: String,
    pub amount: f64,
    pub currency: String,
    pub comment: Option<String>,
    pub paid_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct PaymentCheckoutResponse {
    pub payment: PaymentResponse,
    pub wallet_balance_after: f64,
}

#[derive(Debug, FromRow)]
pub struct PaymentRow {
    pub id: Uuid,
    pub order_id: Uuid,
    pub user_id: Uuid,
    pub payment_method: String,
    pub payment_status: String,
    pub amount: f64,
    pub currency: String,
    pub comment: Option<String>,
    pub paid_at: Option<DateTime<Utc>>,
}

impl From<PaymentRow> for PaymentResponse {
    fn from(value: PaymentRow) -> Self {
        Self {
            id: value.id,
            order_id: value.order_id,
            user_id: value.user_id,
            payment_method: value.payment_method,
            payment_status: value.payment_status,
            amount: value.amount,
            currency: value.currency,
            comment: value.comment,
            paid_at: value.paid_at,
        }
    }
}
