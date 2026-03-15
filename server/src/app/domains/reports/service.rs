use crate::app::domains::reports::models::{
    OrdersReportResponse, PaymentsReportResponse, ReportPeriodParams,
};
use crate::app::domains::reports::repository;
use crate::app::{RequestResult, ServiceContext};

pub async fn build_orders_report(
    params: &ReportPeriodParams,
    ctx: &ServiceContext<'_>,
) -> RequestResult<OrdersReportResponse> {
    let (date_from, date_to) = params.parse()?;
    repository::build_orders_report(date_from, date_to, ctx.db_pool).await
}

pub async fn build_payments_report(
    params: &ReportPeriodParams,
    ctx: &ServiceContext<'_>,
) -> RequestResult<PaymentsReportResponse> {
    let (date_from, date_to) = params.parse()?;
    repository::build_payments_report(date_from, date_to, ctx.db_pool).await
}
