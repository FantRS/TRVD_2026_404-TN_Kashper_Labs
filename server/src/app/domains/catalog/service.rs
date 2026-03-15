use crate::app::domains::catalog::models::{
    CatalogFilterParams, CategoryCreateRequestValid, CategoryUpdateRequestValid,
    ProductCategoryResponse, ProductCreateRequestValid, ProductResponse, ProductUpdateRequestValid,
    ServiceCategoryResponse, ServiceCreateRequestValid, ServiceResponse, ServiceUpdateRequestValid,
};
use crate::app::domains::catalog::repository;
use crate::app::utils::pagination::{PaginatedResponse, PaginationParams};
use crate::app::{RequestResult, ServiceContext};
use uuid::Uuid;

pub async fn get_services(
    filters: &CatalogFilterParams,
    ctx: &ServiceContext<'_>,
) -> RequestResult<PaginatedResponse<ServiceResponse>> {
    let params = PaginationParams {
        page: filters.page(),
        per_page: filters.per_page(),
        search: filters.search.clone(),
    };
    let rows = repository::find_services_paginated(
        params.page,
        params.per_page_capped(),
        filters,
        ctx.db_pool,
    )
    .await?;
    let total = rows.first().and_then(|row| row.total_count).unwrap_or(0);
    let data = rows.into_iter().map(ServiceResponse::from).collect();

    Ok(PaginatedResponse::new(data, &params, total))
}

pub async fn get_service(id: Uuid, ctx: &ServiceContext<'_>) -> RequestResult<ServiceResponse> {
    repository::find_service_by_id(id, ctx.db_pool).await
}

pub async fn create_service(
    request: ServiceCreateRequestValid,
    ctx: &ServiceContext<'_>,
) -> RequestResult<ServiceResponse> {
    repository::create_service(&request, ctx.db_pool).await
}

pub async fn update_service(
    id: Uuid,
    request: ServiceUpdateRequestValid,
    ctx: &ServiceContext<'_>,
) -> RequestResult<ServiceResponse> {
    repository::update_service(id, &request, ctx.db_pool).await
}

pub async fn get_products(
    filters: &CatalogFilterParams,
    ctx: &ServiceContext<'_>,
) -> RequestResult<PaginatedResponse<ProductResponse>> {
    let params = PaginationParams {
        page: filters.page(),
        per_page: filters.per_page(),
        search: filters.search.clone(),
    };
    let rows = repository::find_products_paginated(
        params.page,
        params.per_page_capped(),
        filters,
        ctx.db_pool,
    )
    .await?;
    let total = rows.first().and_then(|row| row.total_count).unwrap_or(0);
    let data = rows.into_iter().map(ProductResponse::from).collect();

    Ok(PaginatedResponse::new(data, &params, total))
}

pub async fn get_product(id: Uuid, ctx: &ServiceContext<'_>) -> RequestResult<ProductResponse> {
    repository::find_product_by_id(id, ctx.db_pool).await
}

pub async fn create_product(
    request: ProductCreateRequestValid,
    ctx: &ServiceContext<'_>,
) -> RequestResult<ProductResponse> {
    repository::create_product(&request, ctx.db_pool).await
}

pub async fn update_product(
    id: Uuid,
    request: ProductUpdateRequestValid,
    ctx: &ServiceContext<'_>,
) -> RequestResult<ProductResponse> {
    repository::update_product(id, &request, ctx.db_pool).await
}

pub async fn get_service_categories(
    ctx: &ServiceContext<'_>,
) -> RequestResult<Vec<ServiceCategoryResponse>> {
    repository::find_service_categories(ctx.db_pool)
        .await
        .map(|rows| {
            rows.into_iter()
                .map(ServiceCategoryResponse::from)
                .collect()
        })
}

pub async fn create_service_category(
    request: CategoryCreateRequestValid,
    ctx: &ServiceContext<'_>,
) -> RequestResult<ServiceCategoryResponse> {
    repository::create_service_category(&request, ctx.db_pool)
        .await
        .map(ServiceCategoryResponse::from)
}

pub async fn update_service_category(
    id: Uuid,
    request: CategoryUpdateRequestValid,
    ctx: &ServiceContext<'_>,
) -> RequestResult<ServiceCategoryResponse> {
    repository::update_service_category(id, &request, ctx.db_pool)
        .await
        .map(ServiceCategoryResponse::from)
}

pub async fn get_product_categories(
    ctx: &ServiceContext<'_>,
) -> RequestResult<Vec<ProductCategoryResponse>> {
    repository::find_product_categories(ctx.db_pool)
        .await
        .map(|rows| {
            rows.into_iter()
                .map(ProductCategoryResponse::from)
                .collect()
        })
}

pub async fn create_product_category(
    request: CategoryCreateRequestValid,
    ctx: &ServiceContext<'_>,
) -> RequestResult<ProductCategoryResponse> {
    repository::create_product_category(&request, ctx.db_pool)
        .await
        .map(ProductCategoryResponse::from)
}

pub async fn update_product_category(
    id: Uuid,
    request: CategoryUpdateRequestValid,
    ctx: &ServiceContext<'_>,
) -> RequestResult<ProductCategoryResponse> {
    repository::update_product_category(id, &request, ctx.db_pool)
        .await
        .map(ProductCategoryResponse::from)
}
