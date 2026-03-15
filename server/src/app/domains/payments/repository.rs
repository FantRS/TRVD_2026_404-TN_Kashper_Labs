use sqlx::PgExecutor;
use uuid::Uuid;

use crate::app::domains::payments::models::{PaymentResponse, PaymentRow};
use crate::app::{RequestError, RequestResult};

pub struct OrderPaymentContext {
    pub order_id: Uuid,
    pub user_id: Uuid,
    pub total_amount: f64,
    pub status_code: String,
    pub wallet_balance: f64,
}

pub async fn create_payment<'c, E>(
    order_id: Uuid,
    user_id: Uuid,
    payment_status: &str,
    amount: f64,
    currency: &str,
    comment: Option<&str>,
    executor: E,
) -> RequestResult<PaymentResponse>
where
    E: PgExecutor<'c>,
{
    sqlx::query_as::<_, PaymentRow>(
        r#"
        INSERT INTO payments (
            order_id,
            user_id,
            payment_method,
            payment_status,
            amount,
            currency,
            comment,
            paid_at
        )
        VALUES (
            $1,
            $2,
            'internal_wallet',
            $3,
            $4,
            $5,
            $6,
            CASE WHEN $3 = 'paid' THEN NOW() ELSE NULL END
        )
        RETURNING
            id,
            order_id,
            user_id,
            payment_method,
            payment_status,
            amount::DOUBLE PRECISION AS amount,
            currency,
            comment,
            paid_at
        "#,
    )
    .bind(order_id)
    .bind(user_id)
    .bind(payment_status)
    .bind(amount)
    .bind(currency)
    .bind(comment)
    .fetch_one(executor)
    .await
    .map(PaymentResponse::from)
    .map_err(Into::into)
}

pub async fn get_order_payment_context<'c, E>(
    order_id: Uuid,
    user_id: Uuid,
    executor: E,
) -> RequestResult<OrderPaymentContext>
where
    E: PgExecutor<'c>,
{
    sqlx::query_as::<_, (Uuid, Uuid, f64, String, f64)>(
        r#"
        SELECT
            orders.id,
            orders.user_id,
            orders.total_amount::DOUBLE PRECISION AS total_amount,
            order_statuses.code AS status_code,
            users.wallet_balance::DOUBLE PRECISION AS wallet_balance
        FROM orders
        INNER JOIN order_statuses ON order_statuses.id = orders.current_status_id
        INNER JOIN users ON users.id = orders.user_id
        WHERE orders.id = $1
          AND orders.user_id = $2
        "#,
    )
    .bind(order_id)
    .bind(user_id)
    .fetch_one(executor)
    .await
    .map(|row| OrderPaymentContext {
        order_id: row.0,
        user_id: row.1,
        total_amount: row.2,
        status_code: row.3,
        wallet_balance: row.4,
    })
    .map_err(Into::into)
}

pub async fn debit_wallet_balance<'c, E>(
    user_id: Uuid,
    amount: f64,
    executor: E,
) -> RequestResult<(f64, f64)>
where
    E: PgExecutor<'c>,
{
    let result = sqlx::query_as::<_, (f64, f64)>(
        r#"
        WITH current_wallet AS (
            SELECT wallet_balance::DOUBLE PRECISION AS balance_before
            FROM users
            WHERE id = $1
            FOR UPDATE
        ),
        updated_user AS (
            UPDATE users
            SET wallet_balance = users.wallet_balance - $2,
                updated_at = NOW()
            WHERE users.id = $1
              AND users.wallet_balance >= $2
            RETURNING users.wallet_balance::DOUBLE PRECISION AS balance_after
        )
        SELECT current_wallet.balance_before, updated_user.balance_after
        FROM current_wallet
        INNER JOIN updated_user ON TRUE
        "#,
    )
    .bind(user_id)
    .bind(amount)
    .fetch_optional(executor)
    .await?;

    result.ok_or_else(|| RequestError::conflict("insufficient wallet balance"))
}

pub async fn create_wallet_transaction<'c, E>(
    user_id: Uuid,
    payment_id: Option<Uuid>,
    transaction_type: &str,
    amount: f64,
    balance_before: f64,
    balance_after: f64,
    comment: Option<&str>,
    executor: E,
) -> RequestResult<()>
where
    E: PgExecutor<'c>,
{
    sqlx::query(
        r#"
        INSERT INTO wallet_transactions (
            user_id,
            payment_id,
            transaction_type,
            amount,
            balance_before,
            balance_after,
            comment
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        "#,
    )
    .bind(user_id)
    .bind(payment_id)
    .bind(transaction_type)
    .bind(amount)
    .bind(balance_before)
    .bind(balance_after)
    .bind(comment)
    .execute(executor)
    .await?;

    Ok(())
}

pub async fn update_order_status_by_code<'c, E>(
    order_id: Uuid,
    status_code: &str,
    executor: E,
) -> RequestResult<()>
where
    E: PgExecutor<'c>,
{
    let affected = sqlx::query(
        r#"
        UPDATE orders
        SET current_status_id = (
            SELECT id
            FROM order_statuses
            WHERE code = $2
        ),
        updated_at = NOW()
        WHERE id = $1
        "#,
    )
    .bind(order_id)
    .bind(status_code)
    .execute(executor)
    .await?
    .rows_affected();

    if affected == 0 {
        return Err(RequestError::not_found("order not found"));
    }

    Ok(())
}
