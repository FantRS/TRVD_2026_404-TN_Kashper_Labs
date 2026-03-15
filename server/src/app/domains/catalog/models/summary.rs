use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Serialize, ToSchema)]
pub struct CatalogItemSummary {
    pub id: Uuid,
    pub item_type: String,
    pub title: String,
    pub price: f64,
    pub is_active: bool,
}
