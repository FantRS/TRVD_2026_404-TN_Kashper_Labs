use actix_web::{HttpResponse, Responder, web};
use uuid::Uuid;

use crate::app::domains::auth::models::Claims;
use crate::app::domains::orders::models::{
    AddProductToCartRequest, AddServiceToCartRequest, CheckoutOrderRequest,
    EmployeeOrderStatusUpdateRequest, OrderResponse,
};
use crate::app::domains::orders::service;
use crate::app::{AppData, RequestResult, ServiceContext};

/// Повертає поточний кошик користувача або створює draft-замовлення (`user`).
#[utoipa::path(
    get,
    path = "/api/orders/cart",
    responses((status = 200, body = OrderResponse)),
    security(("bearer_auth" = [])),
    tag = "Orders"
)]
#[tracing::instrument(name = "get_cart", skip_all, fields(request_id = %Uuid::new_v4()))]
pub async fn get_cart(
    claims: web::ReqData<Claims>,
    app_data: web::Data<AppData>,
) -> RequestResult<impl Responder> {
    let ctx = ServiceContext::from(app_data.get_ref());
    let response = service::get_or_create_draft_order(claims.sub, &ctx).await;

    match &response {
        Ok(_) => tracing::info!("Cart received successfully"),
        Err(error) => tracing::error!("Cart receive failed: {error}"),
    }

    Ok(HttpResponse::Ok().json(response?))
}

/// Додає або оновлює послугу в кошику користувача (`user`).
#[utoipa::path(
    post,
    path = "/api/orders/cart/services",
    request_body = AddServiceToCartRequest,
    responses((status = 200, body = OrderResponse)),
    security(("bearer_auth" = [])),
    tag = "Orders"
)]
#[tracing::instrument(name = "add_service_to_cart", skip_all, fields(request_id = %Uuid::new_v4()))]
pub async fn add_service_to_cart(
    claims: web::ReqData<Claims>,
    request: web::Json<AddServiceToCartRequest>,
    app_data: web::Data<AppData>,
) -> RequestResult<impl Responder> {
    let ctx = ServiceContext::from(app_data.get_ref());
    let response = service::add_service_to_cart(claims.sub, request.into_inner(), &ctx).await;

    match &response {
        Ok(_) => tracing::info!("Service added to cart successfully"),
        Err(error) => tracing::error!("Adding service to cart failed: {error}"),
    }

    Ok(HttpResponse::Ok().json(response?))
}

/// Додає або оновлює товар у кошику користувача (`user`).
#[utoipa::path(
    post,
    path = "/api/orders/cart/products",
    request_body = AddProductToCartRequest,
    responses((status = 200, body = OrderResponse)),
    security(("bearer_auth" = [])),
    tag = "Orders"
)]
#[tracing::instrument(name = "add_product_to_cart", skip_all, fields(request_id = %Uuid::new_v4()))]
pub async fn add_product_to_cart(
    claims: web::ReqData<Claims>,
    request: web::Json<AddProductToCartRequest>,
    app_data: web::Data<AppData>,
) -> RequestResult<impl Responder> {
    let ctx = ServiceContext::from(app_data.get_ref());
    let response = service::add_product_to_cart(claims.sub, request.into_inner(), &ctx).await;

    match &response {
        Ok(_) => tracing::info!("Product added to cart successfully"),
        Err(error) => tracing::error!("Adding product to cart failed: {error}"),
    }

    Ok(HttpResponse::Ok().json(response?))
}

/// Підтверджує оформлення draft-замовлення та переводить його в checkout (`user`).
#[utoipa::path(
    post,
    path = "/api/orders/checkout",
    request_body = CheckoutOrderRequest,
    responses((status = 200, body = OrderResponse)),
    security(("bearer_auth" = [])),
    tag = "Orders"
)]
#[tracing::instrument(name = "checkout", skip_all, fields(request_id = %Uuid::new_v4()))]
pub async fn checkout(
    claims: web::ReqData<Claims>,
    request: web::Json<CheckoutOrderRequest>,
    app_data: web::Data<AppData>,
) -> RequestResult<impl Responder> {
    let request = request.into_inner().try_into()?;
    let ctx = ServiceContext::from(app_data.get_ref());
    let response = service::checkout(claims.sub, request, &ctx).await;

    match &response {
        Ok(_) => tracing::info!("Checkout completed successfully"),
        Err(error) => tracing::error!("Checkout failed: {error}"),
    }

    Ok(HttpResponse::Ok().json(response?))
}

/// Повертає деталі конкретного замовлення поточного користувача (`user`).
#[utoipa::path(
    get,
    path = "/api/orders/{id}",
    params(("id" = Uuid, Path, description = "Order id")),
    responses((status = 200, body = OrderResponse)),
    security(("bearer_auth" = [])),
    tag = "Orders"
)]
#[tracing::instrument(name = "get_order", skip_all, fields(request_id = %Uuid::new_v4(), order_id = %id))]
pub async fn get_order(
    id: web::Path<Uuid>,
    claims: web::ReqData<Claims>,
    app_data: web::Data<AppData>,
) -> RequestResult<impl Responder> {
    let id = id.into_inner();
    let ctx = ServiceContext::from(app_data.get_ref());
    let response = service::get_order_for_user(claims.sub, id, &ctx).await;

    match &response {
        Ok(_) => tracing::info!("Order received successfully"),
        Err(error) => tracing::error!("Order receive failed: {error}"),
    }

    Ok(HttpResponse::Ok().json(response?))
}

/// Змінює статус замовлення працівником або адміністратором (`employee`, `admin`).
#[utoipa::path(
    patch,
    path = "/api/orders/{id}/status",
    params(("id" = Uuid, Path, description = "Order id")),
    request_body = EmployeeOrderStatusUpdateRequest,
    responses((status = 200, body = OrderResponse)),
    security(("bearer_auth" = [])),
    tag = "Orders"
)]
#[tracing::instrument(name = "change_order_status", skip_all, fields(request_id = %Uuid::new_v4(), order_id = %id))]
pub async fn change_order_status(
    id: web::Path<Uuid>,
    claims: web::ReqData<Claims>,
    request: web::Json<EmployeeOrderStatusUpdateRequest>,
    app_data: web::Data<AppData>,
) -> RequestResult<impl Responder> {
    let id = id.into_inner();
    let request = request.into_inner().try_into()?;
    let ctx = ServiceContext::from(app_data.get_ref());
    let response = service::change_order_status(claims.sub, id, request, &ctx).await;

    match &response {
        Ok(_) => tracing::info!("Order status changed successfully"),
        Err(error) => tracing::error!("Order status change failed: {error}"),
    }

    Ok(HttpResponse::Ok().json(response?))
}
