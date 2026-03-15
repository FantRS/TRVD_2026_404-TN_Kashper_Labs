use serde::Serialize;
use sqlx::FromRow;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Serialize, ToSchema)]
pub struct ServiceResponse {
    pub id: Uuid,
    pub category_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub base_price: f64,
    pub duration_minutes: i32,
    pub is_active: bool,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ProductResponse {
    pub id: Uuid,
    pub category_id: Uuid,
    pub sku: String,
    pub name: String,
    pub description: Option<String>,
    pub unit_price: f64,
    pub stock_qty: i32,
    pub is_active: bool,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ServiceCategoryResponse {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ProductCategoryResponse {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, FromRow)]
pub struct ServiceRow {
    pub id: Uuid,
    pub category_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub base_price: f64,
    pub duration_minutes: i32,
    pub is_active: bool,
    pub total_count: Option<i64>,
}

#[derive(Debug, FromRow)]
pub struct ProductRow {
    pub id: Uuid,
    pub category_id: Uuid,
    pub sku: String,
    pub name: String,
    pub description: Option<String>,
    pub unit_price: f64,
    pub stock_qty: i32,
    pub is_active: bool,
    pub total_count: Option<i64>,
}

#[derive(Debug, FromRow)]
pub struct CategoryRow {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
}

impl From<ServiceRow> for ServiceResponse {
    fn from(value: ServiceRow) -> Self {
        Self {
            id: value.id,
            category_id: value.category_id,
            name: value.name,
            description: value.description,
            base_price: value.base_price,
            duration_minutes: value.duration_minutes,
            is_active: value.is_active,
        }
    }
}

impl From<ProductRow> for ProductResponse {
    fn from(value: ProductRow) -> Self {
        Self {
            id: value.id,
            category_id: value.category_id,
            sku: value.sku,
            name: value.name,
            description: value.description,
            unit_price: value.unit_price,
            stock_qty: value.stock_qty,
            is_active: value.is_active,
        }
    }
}

impl From<CategoryRow> for ServiceCategoryResponse {
    fn from(value: CategoryRow) -> Self {
        Self {
            id: value.id,
            name: value.name,
            description: value.description,
        }
    }
}

impl From<CategoryRow> for ProductCategoryResponse {
    fn from(value: CategoryRow) -> Self {
        Self {
            id: value.id,
            name: value.name,
            description: value.description,
        }
    }
}
