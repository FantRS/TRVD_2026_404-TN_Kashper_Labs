use actix_web::{HttpResponse, Responder, web};
use uuid::Uuid;

use crate::app::domains::catalog::models::{
    CatalogFilterParams, CategoryCreateRequest, CategoryUpdateRequest, ProductCreateRequest,
    ProductResponse, ProductUpdateRequest, ServiceCreateRequest, ServiceResponse,
    ServiceUpdateRequest,
};
use crate::app::domains::catalog::service;
use crate::app::{AppData, RequestResult, ServiceContext};

/// Повертає сторінку послуг каталогу з фільтрами та пошуком (`гості`, `user`, `employee`, `admin`).
#[utoipa::path(
    get,
    path = "/api/catalog/services",
    params(CatalogFilterParams),
    responses((status = 200, body = crate::app::utils::pagination::PaginatedResponse<ServiceResponse>)),
    tag = "Catalog"
)]
#[tracing::instrument(name = "get_services", skip_all, fields(request_id = %Uuid::new_v4()))]
pub async fn get_services(
    query: web::Query<CatalogFilterParams>,
    app_data: web::Data<AppData>,
) -> RequestResult<impl Responder> {
    let ctx = ServiceContext::from(app_data.get_ref());
    let response = service::get_services(&query.into_inner(), &ctx).await;

    match &response {
        Ok(_) => tracing::info!("Services page received successfully"),
        Err(error) => tracing::error!("Services page receive failed: {error}"),
    }

    Ok(HttpResponse::Ok().json(response?))
}

/// Повертає деталі конкретної послуги каталогу (`гості`, `user`, `employee`, `admin`).
#[utoipa::path(
    get,
    path = "/api/catalog/services/{id}",
    params(("id" = Uuid, Path, description = "Service id")),
    responses((status = 200, body = ServiceResponse), (status = 404, description = "Service not found")),
    tag = "Catalog"
)]
#[tracing::instrument(name = "get_service", skip_all, fields(request_id = %Uuid::new_v4(), service_id = %id))]
pub async fn get_service(
    id: web::Path<Uuid>,
    app_data: web::Data<AppData>,
) -> RequestResult<impl Responder> {
    let id = id.into_inner();
    let ctx = ServiceContext::from(app_data.get_ref());
    let response = service::get_service(id, &ctx).await;

    match &response {
        Ok(_) => tracing::info!("Service received successfully"),
        Err(error) => tracing::error!("Service receive failed: {error}"),
    }

    Ok(HttpResponse::Ok().json(response?))
}

/// Створює нову послугу в каталозі (`admin`).
#[utoipa::path(
    post,
    path = "/api/catalog/services",
    request_body = ServiceCreateRequest,
    responses((status = 201, body = ServiceResponse)),
    security(("bearer_auth" = [])),
    tag = "Catalog"
)]
#[tracing::instrument(name = "create_service", skip_all, fields(request_id = %Uuid::new_v4()))]
pub async fn create_service(
    request: web::Json<ServiceCreateRequest>,
    app_data: web::Data<AppData>,
) -> RequestResult<impl Responder> {
    let request = request.into_inner().try_into()?;
    let ctx = ServiceContext::from(app_data.get_ref());
    let response = service::create_service(request, &ctx).await;

    match &response {
        Ok(_) => tracing::info!("Service created successfully"),
        Err(error) => tracing::error!("Service creation failed: {error}"),
    }

    Ok(HttpResponse::Created().json(response?))
}

/// Оновлює дані існуючої послуги каталогу (`admin`).
#[utoipa::path(
    patch,
    path = "/api/catalog/services/{id}",
    params(("id" = Uuid, Path, description = "Service id")),
    request_body = ServiceUpdateRequest,
    responses((status = 200, body = ServiceResponse)),
    security(("bearer_auth" = [])),
    tag = "Catalog"
)]
#[tracing::instrument(name = "update_service", skip_all, fields(request_id = %Uuid::new_v4(), service_id = %id))]
pub async fn update_service(
    id: web::Path<Uuid>,
    request: web::Json<ServiceUpdateRequest>,
    app_data: web::Data<AppData>,
) -> RequestResult<impl Responder> {
    let id = id.into_inner();
    let request = request.into_inner().try_into()?;
    let ctx = ServiceContext::from(app_data.get_ref());
    let response = service::update_service(id, request, &ctx).await;

    match &response {
        Ok(_) => tracing::info!("Service updated successfully"),
        Err(error) => tracing::error!("Service update failed: {error}"),
    }

    Ok(HttpResponse::Ok().json(response?))
}

/// Повертає сторінку товарів каталогу з фільтрами та пошуком (`гості`, `user`, `employee`, `admin`).
#[utoipa::path(
    get,
    path = "/api/catalog/products",
    params(CatalogFilterParams),
    responses((status = 200, body = crate::app::utils::pagination::PaginatedResponse<ProductResponse>)),
    tag = "Catalog"
)]
#[tracing::instrument(name = "get_products", skip_all, fields(request_id = %Uuid::new_v4()))]
pub async fn get_products(
    query: web::Query<CatalogFilterParams>,
    app_data: web::Data<AppData>,
) -> RequestResult<impl Responder> {
    let ctx = ServiceContext::from(app_data.get_ref());
    let response = service::get_products(&query.into_inner(), &ctx).await;

    match &response {
        Ok(_) => tracing::info!("Products page received successfully"),
        Err(error) => tracing::error!("Products page receive failed: {error}"),
    }

    Ok(HttpResponse::Ok().json(response?))
}

/// Повертає деталі конкретного товару каталогу (`гості`, `user`, `employee`, `admin`).
#[utoipa::path(
    get,
    path = "/api/catalog/products/{id}",
    params(("id" = Uuid, Path, description = "Product id")),
    responses((status = 200, body = ProductResponse), (status = 404, description = "Product not found")),
    tag = "Catalog"
)]
#[tracing::instrument(name = "get_product", skip_all, fields(request_id = %Uuid::new_v4(), product_id = %id))]
pub async fn get_product(
    id: web::Path<Uuid>,
    app_data: web::Data<AppData>,
) -> RequestResult<impl Responder> {
    let id = id.into_inner();
    let ctx = ServiceContext::from(app_data.get_ref());
    let response = service::get_product(id, &ctx).await;

    match &response {
        Ok(_) => tracing::info!("Product received successfully"),
        Err(error) => tracing::error!("Product receive failed: {error}"),
    }

    Ok(HttpResponse::Ok().json(response?))
}

/// Створює новий товар у каталозі (`admin`).
#[utoipa::path(
    post,
    path = "/api/catalog/products",
    request_body = ProductCreateRequest,
    responses((status = 201, body = ProductResponse)),
    security(("bearer_auth" = [])),
    tag = "Catalog"
)]
#[tracing::instrument(name = "create_product", skip_all, fields(request_id = %Uuid::new_v4()))]
pub async fn create_product(
    request: web::Json<ProductCreateRequest>,
    app_data: web::Data<AppData>,
) -> RequestResult<impl Responder> {
    let request = request.into_inner().try_into()?;
    let ctx = ServiceContext::from(app_data.get_ref());
    let response = service::create_product(request, &ctx).await;

    match &response {
        Ok(_) => tracing::info!("Product created successfully"),
        Err(error) => tracing::error!("Product creation failed: {error}"),
    }

    Ok(HttpResponse::Created().json(response?))
}

/// Оновлює дані існуючого товару каталогу (`admin`).
#[utoipa::path(
    patch,
    path = "/api/catalog/products/{id}",
    params(("id" = Uuid, Path, description = "Product id")),
    request_body = ProductUpdateRequest,
    responses((status = 200, body = ProductResponse)),
    security(("bearer_auth" = [])),
    tag = "Catalog"
)]
#[tracing::instrument(name = "update_product", skip_all, fields(request_id = %Uuid::new_v4(), product_id = %id))]
pub async fn update_product(
    id: web::Path<Uuid>,
    request: web::Json<ProductUpdateRequest>,
    app_data: web::Data<AppData>,
) -> RequestResult<impl Responder> {
    let id = id.into_inner();
    let request = request.into_inner().try_into()?;
    let ctx = ServiceContext::from(app_data.get_ref());
    let response = service::update_product(id, request, &ctx).await;

    match &response {
        Ok(_) => tracing::info!("Product updated successfully"),
        Err(error) => tracing::error!("Product update failed: {error}"),
    }

    Ok(HttpResponse::Ok().json(response?))
}

/// Повертає список категорій послуг (`гості`, `user`, `employee`, `admin`).
#[utoipa::path(
    get,
    path = "/api/catalog/categories/services",
    responses((status = 200, body = [crate::app::domains::catalog::models::ServiceCategoryResponse])),
    tag = "Catalog"
)]
#[tracing::instrument(name = "get_service_categories", skip_all, fields(request_id = %Uuid::new_v4()))]
pub async fn get_service_categories(app_data: web::Data<AppData>) -> RequestResult<impl Responder> {
    let ctx = ServiceContext::from(app_data.get_ref());
    let response = service::get_service_categories(&ctx).await;

    match &response {
        Ok(_) => tracing::info!("Service categories received successfully"),
        Err(error) => tracing::error!("Service categories receive failed: {error}"),
    }

    Ok(HttpResponse::Ok().json(response?))
}

/// Створює нову категорію послуг (`admin`).
#[utoipa::path(
    post,
    path = "/api/catalog/categories/services",
    request_body = CategoryCreateRequest,
    responses((status = 201, body = crate::app::domains::catalog::models::ServiceCategoryResponse)),
    security(("bearer_auth" = [])),
    tag = "Catalog"
)]
#[tracing::instrument(name = "create_service_category", skip_all, fields(request_id = %Uuid::new_v4()))]
pub async fn create_service_category(
    request: web::Json<CategoryCreateRequest>,
    app_data: web::Data<AppData>,
) -> RequestResult<impl Responder> {
    let request = request.into_inner().try_into()?;
    let ctx = ServiceContext::from(app_data.get_ref());
    let response = service::create_service_category(request, &ctx).await;

    match &response {
        Ok(_) => tracing::info!("Service category created successfully"),
        Err(error) => tracing::error!("Service category creation failed: {error}"),
    }

    Ok(HttpResponse::Created().json(response?))
}

/// Оновлює існуючу категорію послуг (`admin`).
#[utoipa::path(
    patch,
    path = "/api/catalog/categories/services/{id}",
    params(("id" = Uuid, Path, description = "Service category id")),
    request_body = CategoryUpdateRequest,
    responses((status = 200, body = crate::app::domains::catalog::models::ServiceCategoryResponse)),
    security(("bearer_auth" = [])),
    tag = "Catalog"
)]
#[tracing::instrument(name = "update_service_category", skip_all, fields(request_id = %Uuid::new_v4(), category_id = %id))]
pub async fn update_service_category(
    id: web::Path<Uuid>,
    request: web::Json<CategoryUpdateRequest>,
    app_data: web::Data<AppData>,
) -> RequestResult<impl Responder> {
    let id = id.into_inner();
    let request = request.into_inner().try_into()?;
    let ctx = ServiceContext::from(app_data.get_ref());
    let response = service::update_service_category(id, request, &ctx).await;

    match &response {
        Ok(_) => tracing::info!("Service category updated successfully"),
        Err(error) => tracing::error!("Service category update failed: {error}"),
    }

    Ok(HttpResponse::Ok().json(response?))
}

/// Повертає список категорій товарів (`гості`, `user`, `employee`, `admin`).
#[utoipa::path(
    get,
    path = "/api/catalog/categories/products",
    responses((status = 200, body = [crate::app::domains::catalog::models::ProductCategoryResponse])),
    tag = "Catalog"
)]
#[tracing::instrument(name = "get_product_categories", skip_all, fields(request_id = %Uuid::new_v4()))]
pub async fn get_product_categories(app_data: web::Data<AppData>) -> RequestResult<impl Responder> {
    let ctx = ServiceContext::from(app_data.get_ref());
    let response = service::get_product_categories(&ctx).await;

    match &response {
        Ok(_) => tracing::info!("Product categories received successfully"),
        Err(error) => tracing::error!("Product categories receive failed: {error}"),
    }

    Ok(HttpResponse::Ok().json(response?))
}

/// Створює нову категорію товарів (`admin`).
#[utoipa::path(
    post,
    path = "/api/catalog/categories/products",
    request_body = CategoryCreateRequest,
    responses((status = 201, body = crate::app::domains::catalog::models::ProductCategoryResponse)),
    security(("bearer_auth" = [])),
    tag = "Catalog"
)]
#[tracing::instrument(name = "create_product_category", skip_all, fields(request_id = %Uuid::new_v4()))]
pub async fn create_product_category(
    request: web::Json<CategoryCreateRequest>,
    app_data: web::Data<AppData>,
) -> RequestResult<impl Responder> {
    let request = request.into_inner().try_into()?;
    let ctx = ServiceContext::from(app_data.get_ref());
    let response = service::create_product_category(request, &ctx).await;

    match &response {
        Ok(_) => tracing::info!("Product category created successfully"),
        Err(error) => tracing::error!("Product category creation failed: {error}"),
    }

    Ok(HttpResponse::Created().json(response?))
}

/// Оновлює існуючу категорію товарів (`admin`).
#[utoipa::path(
    patch,
    path = "/api/catalog/categories/products/{id}",
    params(("id" = Uuid, Path, description = "Product category id")),
    request_body = CategoryUpdateRequest,
    responses((status = 200, body = crate::app::domains::catalog::models::ProductCategoryResponse)),
    security(("bearer_auth" = [])),
    tag = "Catalog"
)]
#[tracing::instrument(name = "update_product_category", skip_all, fields(request_id = %Uuid::new_v4(), category_id = %id))]
pub async fn update_product_category(
    id: web::Path<Uuid>,
    request: web::Json<CategoryUpdateRequest>,
    app_data: web::Data<AppData>,
) -> RequestResult<impl Responder> {
    let id = id.into_inner();
    let request = request.into_inner().try_into()?;
    let ctx = ServiceContext::from(app_data.get_ref());
    let response = service::update_product_category(id, request, &ctx).await;

    match &response {
        Ok(_) => tracing::info!("Product category updated successfully"),
        Err(error) => tracing::error!("Product category update failed: {error}"),
    }

    Ok(HttpResponse::Ok().json(response?))
}
