use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

/// Query parameters for paginated requests
#[derive(Debug, Deserialize, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct PaginationParams {
    #[serde(default = "default_page")]
    pub page: u32,
    #[serde(default = "default_per_page")]
    pub per_page: u32,
    pub search: Option<String>,
}

fn default_page() -> u32 {
    1
}

fn default_per_page() -> u32 {
    20
}

impl PaginationParams {
    /// Клієнт не може запросити більше 100 записів
    pub fn per_page_capped(&self) -> u32 {
        self.per_page.min(100)
    }

    /// Розраховує зміщення для SQL-запиту
    pub fn offset(&self) -> u32 {
        (self.page.saturating_sub(1)) * self.per_page_capped()
    }

    /// Готує рядок для SQL-оператора ILIKE
    pub fn search_pattern(&self) -> Option<String> {
        self.search
            .as_ref()
            .filter(|s| !s.trim().is_empty())
            .map(|s| format!("%{}%", s.trim()))
    }
}

/// Пагіновані метадані
#[derive(Debug, Serialize, ToSchema)]
pub struct PaginationMeta {
    pub current_page: u32,
    pub per_page: u32,
    pub total_items: i64,
    pub total_pages: u32,
}

impl PaginationMeta {
    pub fn new(current_page: u32, per_page: u32, total_items: i64) -> Self {
        let total_pages = if total_items == 0 {
            1
        } else {
            ((total_items as f64) / (per_page as f64)).ceil() as u32
        };

        Self {
            current_page,
            per_page,
            total_items,
            total_pages,
        }
    }
}

#[derive(Debug, Serialize, ToSchema)]
pub struct PaginatedResponse<T: Serialize> {
    pub data: Vec<T>,
    pub meta: PaginationMeta,
}

impl<T: Serialize> PaginatedResponse<T> {
    pub fn new(data: Vec<T>, params: &PaginationParams, total_items: i64) -> Self {
        Self {
            data,
            meta: PaginationMeta::new(params.page, params.per_page_capped(), total_items),
        }
    }
}

pub trait HasTotalCount {
    fn total_count(&self) -> Option<i64>;
}

pub fn create_paginated_response<T>(rows: Vec<T>, params: &PaginationParams) -> PaginatedResponse<T>
where
    T: HasTotalCount + Serialize,
{
    let total = rows.first().and_then(|r| r.total_count()).unwrap_or(0);
    PaginatedResponse::new(rows, params, total)
}
