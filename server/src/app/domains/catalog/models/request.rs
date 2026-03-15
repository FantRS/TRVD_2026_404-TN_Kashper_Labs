use serde::Deserialize;
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;

use crate::app::RequestError;
use crate::app::utils::validation::{
    non_negative_f64, positive_i32, trimmed_optional, trimmed_required,
};

#[derive(Debug, Clone, Deserialize, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct CatalogFilterParams {
    pub page: Option<u32>,
    pub per_page: Option<u32>,
    pub search: Option<String>,
    pub category_id: Option<Uuid>,
    pub min_price: Option<f64>,
    pub max_price: Option<f64>,
    pub only_active: Option<bool>,
}

impl CatalogFilterParams {
    pub fn page(&self) -> u32 {
        self.page.unwrap_or(1)
    }

    pub fn per_page(&self) -> u32 {
        self.per_page.unwrap_or(20)
    }
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct ServiceCreateRequest {
    pub category_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub base_price: f64,
    pub duration_minutes: i32,
}

#[derive(Debug)]
pub struct ServiceCreateRequestValid {
    pub category_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub base_price: f64,
    pub duration_minutes: i32,
}

impl TryFrom<ServiceCreateRequest> for ServiceCreateRequestValid {
    type Error = RequestError;

    fn try_from(value: ServiceCreateRequest) -> Result<Self, Self::Error> {
        Ok(Self {
            category_id: value.category_id,
            name: trimmed_required(&value.name, "name", 2, 255)?,
            description: trimmed_optional(value.description, "description", 4_000)?,
            base_price: non_negative_f64(value.base_price, "base_price")?,
            duration_minutes: positive_i32(value.duration_minutes, "duration_minutes")?,
        })
    }
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct ServiceUpdateRequest {
    pub category_id: Option<Uuid>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub base_price: Option<f64>,
    pub duration_minutes: Option<i32>,
    pub is_active: Option<bool>,
}

#[derive(Debug)]
pub struct ServiceUpdateRequestValid {
    pub category_id: Option<Uuid>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub base_price: Option<f64>,
    pub duration_minutes: Option<i32>,
    pub is_active: Option<bool>,
}

impl TryFrom<ServiceUpdateRequest> for ServiceUpdateRequestValid {
    type Error = RequestError;

    fn try_from(value: ServiceUpdateRequest) -> Result<Self, Self::Error> {
        Ok(Self {
            category_id: value.category_id,
            name: match value.name {
                Some(name) => Some(trimmed_required(&name, "name", 2, 255)?),
                None => None,
            },
            description: trimmed_optional(value.description, "description", 4_000)?,
            base_price: match value.base_price {
                Some(base_price) => Some(non_negative_f64(base_price, "base_price")?),
                None => None,
            },
            duration_minutes: match value.duration_minutes {
                Some(duration_minutes) => Some(positive_i32(duration_minutes, "duration_minutes")?),
                None => None,
            },
            is_active: value.is_active,
        })
    }
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct ProductCreateRequest {
    pub category_id: Uuid,
    pub sku: String,
    pub name: String,
    pub description: Option<String>,
    pub unit_price: f64,
    pub stock_qty: i32,
}

#[derive(Debug)]
pub struct ProductCreateRequestValid {
    pub category_id: Uuid,
    pub sku: String,
    pub name: String,
    pub description: Option<String>,
    pub unit_price: f64,
    pub stock_qty: i32,
}

impl TryFrom<ProductCreateRequest> for ProductCreateRequestValid {
    type Error = RequestError;

    fn try_from(value: ProductCreateRequest) -> Result<Self, Self::Error> {
        Ok(Self {
            category_id: value.category_id,
            sku: trimmed_required(&value.sku, "sku", 2, 64)?,
            name: trimmed_required(&value.name, "name", 2, 255)?,
            description: trimmed_optional(value.description, "description", 4_000)?,
            unit_price: non_negative_f64(value.unit_price, "unit_price")?,
            stock_qty: value.stock_qty.max(0),
        })
    }
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct ProductUpdateRequest {
    pub category_id: Option<Uuid>,
    pub sku: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub unit_price: Option<f64>,
    pub stock_qty: Option<i32>,
    pub is_active: Option<bool>,
}

#[derive(Debug)]
pub struct ProductUpdateRequestValid {
    pub category_id: Option<Uuid>,
    pub sku: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub unit_price: Option<f64>,
    pub stock_qty: Option<i32>,
    pub is_active: Option<bool>,
}

impl TryFrom<ProductUpdateRequest> for ProductUpdateRequestValid {
    type Error = RequestError;

    fn try_from(value: ProductUpdateRequest) -> Result<Self, Self::Error> {
        Ok(Self {
            category_id: value.category_id,
            sku: match value.sku {
                Some(sku) => Some(trimmed_required(&sku, "sku", 2, 64)?),
                None => None,
            },
            name: match value.name {
                Some(name) => Some(trimmed_required(&name, "name", 2, 255)?),
                None => None,
            },
            description: trimmed_optional(value.description, "description", 4_000)?,
            unit_price: match value.unit_price {
                Some(unit_price) => Some(non_negative_f64(unit_price, "unit_price")?),
                None => None,
            },
            stock_qty: value.stock_qty.map(|stock_qty| stock_qty.max(0)),
            is_active: value.is_active,
        })
    }
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CategoryCreateRequest {
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug)]
pub struct CategoryCreateRequestValid {
    pub name: String,
    pub description: Option<String>,
}

impl TryFrom<CategoryCreateRequest> for CategoryCreateRequestValid {
    type Error = RequestError;

    fn try_from(value: CategoryCreateRequest) -> Result<Self, Self::Error> {
        Ok(Self {
            name: trimmed_required(&value.name, "name", 2, 128)?,
            description: trimmed_optional(value.description, "description", 2_000)?,
        })
    }
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CategoryUpdateRequest {
    pub name: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug)]
pub struct CategoryUpdateRequestValid {
    pub name: Option<String>,
    pub description: Option<String>,
}

impl TryFrom<CategoryUpdateRequest> for CategoryUpdateRequestValid {
    type Error = RequestError;

    fn try_from(value: CategoryUpdateRequest) -> Result<Self, Self::Error> {
        Ok(Self {
            name: match value.name {
                Some(name) => Some(trimmed_required(&name, "name", 2, 128)?),
                None => None,
            },
            description: trimmed_optional(value.description, "description", 2_000)?,
        })
    }
}
