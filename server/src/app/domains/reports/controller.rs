use actix_web::{HttpResponse, Responder, web};
use uuid::Uuid;

use crate::app::domains::reports::models::ReportPeriodParams;
use crate::app::domains::reports::service;
use crate::app::{AppData, RequestResult, ServiceContext};

/// Повертає агрегований звіт по замовленнях за період (`admin`).
#[utoipa::path(
    get,
    path = "/api/reports/orders",
    params(ReportPeriodParams),
    responses((status = 200, body = crate::app::domains::reports::models::OrdersReportResponse)),
    security(("bearer_auth" = [])),
    tag = "Reports"
)]
#[tracing::instrument(name = "get_orders_report", skip_all, fields(request_id = %Uuid::new_v4()))]
pub async fn get_orders_report(
    query: web::Query<ReportPeriodParams>,
    app_data: web::Data<AppData>,
) -> RequestResult<impl Responder> {
    let ctx = ServiceContext::from(app_data.get_ref());
    let response = service::build_orders_report(&query.into_inner(), &ctx).await;

    match &response {
        Ok(_) => tracing::info!("Orders report received successfully"),
        Err(error) => tracing::error!("Orders report receive failed: {error}"),
    }

    Ok(HttpResponse::Ok().json(response?))
}

/// Повертає агрегований звіт по оплатах за період (`admin`).
#[utoipa::path(
    get,
    path = "/api/reports/payments",
    params(ReportPeriodParams),
    responses((status = 200, body = crate::app::domains::reports::models::PaymentsReportResponse)),
    security(("bearer_auth" = [])),
    tag = "Reports"
)]
#[tracing::instrument(name = "get_payments_report", skip_all, fields(request_id = %Uuid::new_v4()))]
pub async fn get_payments_report(
    query: web::Query<ReportPeriodParams>,
    app_data: web::Data<AppData>,
) -> RequestResult<impl Responder> {
    let ctx = ServiceContext::from(app_data.get_ref());
    let response = service::build_payments_report(&query.into_inner(), &ctx).await;

    match &response {
        Ok(_) => tracing::info!("Payments report received successfully"),
        Err(error) => tracing::error!("Payments report receive failed: {error}"),
    }

    Ok(HttpResponse::Ok().json(response?))
}
