use chrono::{DateTime, Utc};
use sqlx::PgExecutor;

use crate::app::RequestResult;
use crate::app::domains::reports::models::{OrdersReportResponse, PaymentsReportResponse};

pub async fn build_orders_report<'c, E>(
    date_from: DateTime<Utc>,
    date_to: DateTime<Utc>,
    executor: E,
) -> RequestResult<OrdersReportResponse>
where
    E: PgExecutor<'c>,
{
    sqlx::query_as::<_, (i64, f64, i64, i64, i64)>(
        r#"
        SELECT
            COUNT(*)::BIGINT AS total_orders,
            COALESCE(SUM(orders.total_amount), 0)::DOUBLE PRECISION AS total_amount,
            COUNT(*) FILTER (WHERE order_statuses.code = 'draft')::BIGINT AS draft_orders,
            COUNT(*) FILTER (WHERE order_statuses.code = 'awaiting_payment')::BIGINT AS awaiting_payment_orders,
            COUNT(*) FILTER (WHERE order_statuses.code = 'completed')::BIGINT AS completed_orders
        FROM orders
        INNER JOIN order_statuses ON order_statuses.id = orders.current_status_id
        WHERE orders.created_at >= $1
          AND orders.created_at <= $2
        "#,
    )
    .bind(date_from)
    .bind(date_to)
    .fetch_one(executor)
    .await
    .map(|row| OrdersReportResponse {
        total_orders: row.0,
        total_amount: row.1,
        draft_orders: row.2,
        awaiting_payment_orders: row.3,
        completed_orders: row.4,
    })
    .map_err(Into::into)
}

pub async fn build_payments_report<'c, E>(
    date_from: DateTime<Utc>,
    date_to: DateTime<Utc>,
    executor: E,
) -> RequestResult<PaymentsReportResponse>
where
    E: PgExecutor<'c>,
{
    sqlx::query_as::<_, (i64, f64, i64)>(
        r#"
        SELECT
            COUNT(*)::BIGINT AS total_payments,
            COALESCE(SUM(amount) FILTER (WHERE payment_status = 'paid'), 0)::DOUBLE PRECISION AS paid_amount,
            COUNT(*) FILTER (WHERE payment_status = 'failed')::BIGINT AS failed_payments
        FROM payments
        WHERE created_at >= $1
          AND created_at <= $2
        "#,
    )
    .bind(date_from)
    .bind(date_to)
    .fetch_one(executor)
    .await
    .map(|row| PaymentsReportResponse {
        total_payments: row.0,
        paid_amount: row.1,
        failed_payments: row.2,
    })
    .map_err(Into::into)
}
