use uuid::Uuid;

use crate::app::domains::orders::models::{
    AddProductToCartRequest, AddServiceToCartRequest, CheckoutOrderRequestValid,
    EmployeeOrderStatusUpdateRequestValid, OrderNumber, OrderResponse, OrderSummary,
};
use crate::app::domains::orders::repository;
use crate::app::events::{DomainEvent, publish};
use crate::app::{RequestError, RequestResult, ServiceContext};
use sqlx::{Postgres, Transaction};

pub async fn get_or_create_draft_order(
    user_id: Uuid,
    ctx: &ServiceContext<'_>,
) -> RequestResult<OrderResponse> {
    let mut tx = ctx.db_pool.begin().await?;
    let order_id = match repository::find_draft_order_id_by_user_id(user_id, &mut *tx).await? {
        Some(order_id) => order_id,
        None => {
            let order_id = repository::create_draft_order(user_id, &mut *tx).await?;
            let draft_status_id = repository::find_status_id_by_code("draft", &mut *tx).await?;
            repository::create_status_history_entry(
                order_id,
                draft_status_id,
                Some(user_id),
                None,
                &mut *tx,
            )
            .await?;
            order_id
        }
    };
    let response = load_order_response(order_id, &mut tx).await?;
    tx.commit().await?;

    Ok(response)
}

pub async fn get_orders_for_user(
    user_id: Uuid,
    ctx: &ServiceContext<'_>,
) -> RequestResult<Vec<OrderSummary>> {
    repository::get_order_summaries_for_user(user_id, ctx.db_pool).await
}

pub async fn add_service_to_cart(
    user_id: Uuid,
    request: AddServiceToCartRequest,
    ctx: &ServiceContext<'_>,
) -> RequestResult<OrderResponse> {
    let request = request.validate()?;
    let mut tx = ctx.db_pool.begin().await?;
    let order_id = ensure_draft_order(user_id, &mut tx).await?;
    repository::upsert_service_item(order_id, request.service_id, request.quantity, &mut *tx)
        .await?;
    repository::refresh_order_total(order_id, &mut *tx).await?;
    let response = load_order_response(order_id, &mut tx).await?;
    tx.commit().await?;

    Ok(response)
}

pub async fn add_product_to_cart(
    user_id: Uuid,
    request: AddProductToCartRequest,
    ctx: &ServiceContext<'_>,
) -> RequestResult<OrderResponse> {
    let request = request.validate()?;
    let mut tx = ctx.db_pool.begin().await?;
    let order_id = ensure_draft_order(user_id, &mut tx).await?;
    repository::upsert_product_item(order_id, request.product_id, request.quantity, &mut *tx)
        .await?;
    repository::refresh_order_total(order_id, &mut *tx).await?;
    let response = load_order_response(order_id, &mut tx).await?;
    tx.commit().await?;

    Ok(response)
}

pub async fn remove_service_from_cart(
    user_id: Uuid,
    service_id: Uuid,
    ctx: &ServiceContext<'_>,
) -> RequestResult<OrderResponse> {
    let mut tx = ctx.db_pool.begin().await?;
    let order_id = repository::find_draft_order_id_by_user_id(user_id, &mut *tx)
        .await?
        .ok_or_else(|| RequestError::not_found("draft order not found"))?;
    repository::remove_service_item(order_id, service_id, &mut *tx).await?;
    repository::refresh_order_total(order_id, &mut *tx).await?;
    let response = load_order_response(order_id, &mut tx).await?;
    tx.commit().await?;

    Ok(response)
}

pub async fn remove_product_from_cart(
    user_id: Uuid,
    product_id: Uuid,
    ctx: &ServiceContext<'_>,
) -> RequestResult<OrderResponse> {
    let mut tx = ctx.db_pool.begin().await?;
    let order_id = repository::find_draft_order_id_by_user_id(user_id, &mut *tx)
        .await?
        .ok_or_else(|| RequestError::not_found("draft order not found"))?;
    repository::remove_product_item(order_id, product_id, &mut *tx).await?;
    repository::refresh_order_total(order_id, &mut *tx).await?;
    let response = load_order_response(order_id, &mut tx).await?;
    tx.commit().await?;

    Ok(response)
}

pub async fn checkout(
    user_id: Uuid,
    request: CheckoutOrderRequestValid,
    ctx: &ServiceContext<'_>,
) -> RequestResult<OrderResponse> {
    let mut tx = ctx.db_pool.begin().await?;
    let order_id = repository::find_draft_order_id_by_user_id(user_id, &mut *tx)
        .await?
        .ok_or_else(|| RequestError::not_found("draft order not found"))?;
    let total_amount = repository::refresh_order_total(order_id, &mut *tx).await?;

    if total_amount <= 0.0 {
        return Err(RequestError::unprocessable_entity(
            "draft order must contain at least one item",
        ));
    }

    let order_number = OrderNumber::generate();
    repository::finalize_checkout(
        order_id,
        &request,
        total_amount,
        "awaiting_payment",
        &order_number.0,
        &mut *tx,
    )
    .await?;
    let awaiting_payment_status_id =
        repository::find_status_id_by_code("awaiting_payment", &mut *tx).await?;
    repository::create_status_history_entry(
        order_id,
        awaiting_payment_status_id,
        Some(user_id),
        None,
        &mut *tx,
    )
    .await?;
    let response = load_order_response(order_id, &mut tx).await?;
    tx.commit().await?;

    let _ = publish(
        DomainEvent::OrderCreated {
            order_id: response.id,
            order_number: response.order_number.clone(),
        },
        ctx.redis,
    )
    .await;

    Ok(response)
}

pub async fn change_order_status(
    actor_user_id: Uuid,
    id: Uuid,
    request: EmployeeOrderStatusUpdateRequestValid,
    ctx: &ServiceContext<'_>,
) -> RequestResult<OrderResponse> {
    let mut tx = ctx.db_pool.begin().await?;
    let status_id = repository::update_order_status(id, &request.status_code.0, &mut *tx).await?;
    repository::create_status_history_entry(
        id,
        status_id,
        Some(actor_user_id),
        request.comment.as_deref(),
        &mut *tx,
    )
    .await?;
    let response = load_order_response(id, &mut tx).await?;
    tx.commit().await?;

    let _ = publish(
        DomainEvent::OrderStatusChanged {
            order_id: response.id,
            status_code: response.current_status_code.clone(),
        },
        ctx.redis,
    )
    .await;

    Ok(response)
}

pub async fn get_order_for_user(
    user_id: Uuid,
    order_id: Uuid,
    ctx: &ServiceContext<'_>,
) -> RequestResult<OrderResponse> {
    let mut tx = ctx.db_pool.begin().await?;
    repository::ensure_order_owner(order_id, user_id, &mut *tx).await?;
    let response = load_order_response(order_id, &mut tx).await?;
    tx.commit().await?;
    Ok(response)
}

async fn ensure_draft_order(
    user_id: Uuid,
    tx: &mut Transaction<'_, Postgres>,
) -> RequestResult<Uuid> {
    match repository::find_draft_order_id_by_user_id(user_id, &mut **tx).await? {
        Some(order_id) => Ok(order_id),
        None => {
            let order_id = repository::create_draft_order(user_id, &mut **tx).await?;
            let status_id = repository::find_status_id_by_code("draft", &mut **tx).await?;
            repository::create_status_history_entry(
                order_id,
                status_id,
                Some(user_id),
                None,
                &mut **tx,
            )
            .await?;
            Ok(order_id)
        }
    }
}

async fn load_order_response(
    order_id: Uuid,
    tx: &mut Transaction<'_, Postgres>,
) -> RequestResult<OrderResponse> {
    let order = repository::get_order_row(order_id, &mut **tx).await?;
    let items = repository::get_order_items(order_id, &mut **tx).await?;

    Ok(OrderResponse::from_parts(order, items))
}
