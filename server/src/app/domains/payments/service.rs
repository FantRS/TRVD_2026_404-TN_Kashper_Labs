use uuid::Uuid;

use crate::app::domains::orders::repository as orders_repository;
use crate::app::domains::payments::models::{CreatePaymentRequest, PaymentCheckoutResponse};
use crate::app::domains::payments::repository;
use crate::app::events::{DomainEvent, publish};
use crate::app::{RequestError, RequestResult, ServiceContext};
use crate::constants::wallet::CURRENCY_CODE;

pub async fn create_payment(
    order_id: Uuid,
    user_id: Uuid,
    request: CreatePaymentRequest,
    ctx: &ServiceContext<'_>,
) -> RequestResult<PaymentCheckoutResponse> {
    let request = request.validate()?;
    let mut tx = ctx.db_pool.begin().await?;
    let payment_context =
        repository::get_order_payment_context(order_id, user_id, &mut *tx).await?;

    if payment_context.total_amount <= 0.0 {
        return Err(RequestError::unprocessable_entity(
            "order total amount must be greater than zero",
        ));
    }

    match payment_context.status_code.as_str() {
        "awaiting_payment" | "new" => {}
        "confirmed" | "in_progress" | "completed" => {
            return Err(RequestError::conflict(
                "order is already paid or being processed",
            ));
        }
        _ => {
            return Err(RequestError::conflict("order is not ready for payment"));
        }
    }

    if payment_context.wallet_balance < payment_context.total_amount {
        let failure_comment = match request.comment.as_ref() {
            Some(comment) => format!("{comment}. Payment rejected: insufficient wallet balance"),
            None => "Payment rejected: insufficient wallet balance".to_owned(),
        };

        let _failed_payment = repository::create_payment(
            payment_context.order_id,
            payment_context.user_id,
            "failed",
            payment_context.total_amount,
            CURRENCY_CODE,
            Some(&failure_comment),
            &mut *tx,
        )
        .await?;

        tx.commit().await?;
        return Err(RequestError::conflict("insufficient wallet balance"));
    }

    let payment = repository::create_payment(
        payment_context.order_id,
        payment_context.user_id,
        "paid",
        payment_context.total_amount,
        CURRENCY_CODE,
        request.comment.as_deref(),
        &mut *tx,
    )
    .await?;
    let (balance_before, balance_after) = repository::debit_wallet_balance(
        payment_context.user_id,
        payment_context.total_amount,
        &mut *tx,
    )
    .await?;
    repository::create_wallet_transaction(
        payment_context.user_id,
        Some(payment.id),
        "payment_debit",
        payment_context.total_amount,
        balance_before,
        balance_after,
        request.comment.as_deref(),
        &mut *tx,
    )
    .await?;
    repository::update_order_status_by_code(payment_context.order_id, "confirmed", &mut *tx)
        .await?;
    let confirmed_status_id =
        orders_repository::find_status_id_by_code("confirmed", &mut *tx).await?;
    orders_repository::create_status_history_entry(
        payment_context.order_id,
        confirmed_status_id,
        Some(user_id),
        Some("Paid with internal wallet"),
        &mut *tx,
    )
    .await?;
    tx.commit().await?;

    let _ = publish(
        DomainEvent::PaymentCreated {
            order_id: payment_context.order_id,
            payment_id: payment.id,
        },
        ctx.redis,
    )
    .await;

    Ok(PaymentCheckoutResponse {
        payment,
        wallet_balance_after: balance_after,
    })
}
