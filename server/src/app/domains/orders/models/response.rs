use serde::Serialize;
use sqlx::FromRow;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Serialize, ToSchema)]
pub struct OrderItemResponse {
    pub item_id: Uuid,
    pub item_type: String,
    pub reference_id: Uuid,
    pub title: String,
    pub quantity: i32,
    pub unit_price: f64,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct OrderResponse {
    pub id: Uuid,
    pub order_number: String,
    pub user_id: Uuid,
    pub current_status_code: String,
    pub contact_name: String,
    pub contact_phone: String,
    pub contact_email: String,
    pub delivery_address: String,
    pub total_amount: f64,
    pub items: Vec<OrderItemResponse>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct OrderStatusHistoryResponse {
    pub id: Uuid,
    pub status_code: String,
    pub comment: Option<String>,
    pub changed_by_user_id: Option<Uuid>,
    pub changed_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, FromRow)]
pub struct OrderRow {
    pub id: Uuid,
    pub order_number: String,
    pub user_id: Uuid,
    pub current_status_code: String,
    pub contact_name: String,
    pub contact_phone: String,
    pub contact_email: String,
    pub delivery_address: String,
    pub total_amount: f64,
}

#[derive(Debug, FromRow)]
pub struct OrderItemRow {
    pub item_id: Uuid,
    pub item_type: String,
    pub reference_id: Uuid,
    pub title: String,
    pub quantity: i32,
    pub unit_price: f64,
}

#[derive(Debug, FromRow)]
pub struct OrderStatusHistoryRow {
    pub id: Uuid,
    pub status_code: String,
    pub comment: Option<String>,
    pub changed_by_user_id: Option<Uuid>,
    pub changed_at: chrono::DateTime<chrono::Utc>,
}

impl From<OrderItemRow> for OrderItemResponse {
    fn from(value: OrderItemRow) -> Self {
        Self {
            item_id: value.item_id,
            item_type: value.item_type,
            reference_id: value.reference_id,
            title: value.title,
            quantity: value.quantity,
            unit_price: value.unit_price,
        }
    }
}

impl From<OrderStatusHistoryRow> for OrderStatusHistoryResponse {
    fn from(value: OrderStatusHistoryRow) -> Self {
        Self {
            id: value.id,
            status_code: value.status_code,
            comment: value.comment,
            changed_by_user_id: value.changed_by_user_id,
            changed_at: value.changed_at,
        }
    }
}

impl OrderResponse {
    pub fn from_parts(order: OrderRow, items: Vec<OrderItemResponse>) -> Self {
        Self {
            id: order.id,
            order_number: order.order_number,
            user_id: order.user_id,
            current_status_code: order.current_status_code,
            contact_name: order.contact_name,
            contact_phone: order.contact_phone,
            contact_email: order.contact_email,
            delivery_address: order.delivery_address,
            total_amount: order.total_amount,
            items,
        }
    }
}
