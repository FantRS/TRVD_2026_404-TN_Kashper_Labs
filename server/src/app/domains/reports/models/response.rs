use serde::Serialize;
use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema)]
pub struct OrdersReportResponse {
    pub total_orders: i64,
    pub total_amount: f64,
    pub draft_orders: i64,
    pub awaiting_payment_orders: i64,
    pub completed_orders: i64,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct PaymentsReportResponse {
    pub total_payments: i64,
    pub paid_amount: f64,
    pub failed_payments: i64,
}
