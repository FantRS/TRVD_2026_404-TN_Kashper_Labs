use actix_web::{HttpResponse, Responder, web};
use uuid::Uuid;

use crate::app::domains::auth::models::Claims;
use crate::app::domains::payments::models::CreatePaymentRequest;
use crate::app::domains::payments::service;
use crate::app::{AppData, RequestResult, ServiceContext};

/// Оплачує замовлення з внутрішнього гаманця користувача (`user`).
#[utoipa::path(
    post,
    path = "/api/orders/{id}/payments",
    params(("id" = Uuid, Path, description = "Order id")),
    request_body = CreatePaymentRequest,
    responses((status = 200, body = crate::app::domains::payments::models::PaymentCheckoutResponse)),
    security(("bearer_auth" = [])),
    tag = "Payments"
)]
#[tracing::instrument(name = "create_payment", skip_all, fields(request_id = %Uuid::new_v4(), order_id = %id))]
pub async fn create_payment(
    id: web::Path<Uuid>,
    request: web::Json<CreatePaymentRequest>,
    claims: web::ReqData<Claims>,
    app_data: web::Data<AppData>,
) -> RequestResult<impl Responder> {
    let id = id.into_inner();
    let ctx = ServiceContext::from(app_data.get_ref());
    let response = service::create_payment(id, claims.sub, request.into_inner(), &ctx).await;

    match &response {
        Ok(_) => tracing::info!("Payment created successfully"),
        Err(error) => tracing::error!("Payment creation failed: {error}"),
    }

    Ok(HttpResponse::Ok().json(response?))
}
