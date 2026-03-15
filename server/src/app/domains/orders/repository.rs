use sqlx::PgExecutor;
use uuid::Uuid;

use crate::app::domains::orders::models::{
    CheckoutOrderRequestValid, OrderItemResponse, OrderItemRow, OrderRow, OrderSummary,
};
use crate::app::{RequestError, RequestResult};

pub async fn find_status_id_by_code<'c, E>(code: &str, executor: E) -> RequestResult<Uuid>
where
    E: PgExecutor<'c>,
{
    let row = sqlx::query_scalar::<_, Uuid>(
        r#"
        SELECT id
        FROM order_statuses
        WHERE code = $1
        "#,
    )
    .bind(code)
    .fetch_one(executor)
    .await?;

    Ok(row)
}

pub async fn find_draft_order_id_by_user_id<'c, E>(
    user_id: Uuid,
    executor: E,
) -> RequestResult<Option<Uuid>>
where
    E: PgExecutor<'c>,
{
    sqlx::query_scalar::<_, Uuid>(
        r#"
        SELECT orders.id
        FROM orders
        INNER JOIN order_statuses
            ON order_statuses.id = orders.current_status_id
        WHERE orders.user_id = $1
          AND order_statuses.code = 'draft'
        ORDER BY orders.created_at DESC
        LIMIT 1
        "#,
    )
    .bind(user_id)
    .fetch_optional(executor)
    .await
    .map_err(Into::into)
}

pub async fn create_draft_order<'c, E>(user_id: Uuid, executor: E) -> RequestResult<Uuid>
where
    E: PgExecutor<'c>,
{
    sqlx::query_scalar::<_, Uuid>(
        r#"
        INSERT INTO orders (
            user_id,
            current_status_id,
            order_number,
            contact_name,
            contact_phone,
            contact_email,
            delivery_address
        )
        VALUES (
            $1,
            (
                SELECT id
                FROM order_statuses
                WHERE code = 'draft'
            ),
            $2,
            '',
            '',
            '',
            ''
        )
        RETURNING id
        "#,
    )
    .bind(user_id)
    .bind(format!("DRAFT-{}", Uuid::now_v7()))
    .fetch_one(executor)
    .await
    .map_err(Into::into)
}

pub async fn create_status_history_entry<'c, E>(
    order_id: Uuid,
    status_id: Uuid,
    changed_by_user_id: Option<Uuid>,
    comment: Option<&str>,
    executor: E,
) -> RequestResult<()>
where
    E: PgExecutor<'c>,
{
    sqlx::query(
        r#"
        INSERT INTO order_status_history (
            order_id,
            status_id,
            changed_by_user_id,
            comment
        )
        VALUES ($1, $2, $3, $4)
        "#,
    )
    .bind(order_id)
    .bind(status_id)
    .bind(changed_by_user_id)
    .bind(comment)
    .execute(executor)
    .await?;

    Ok(())
}

pub async fn upsert_service_item<'c, E>(
    order_id: Uuid,
    service_id: Uuid,
    quantity: i32,
    executor: E,
) -> RequestResult<()>
where
    E: PgExecutor<'c>,
{
    sqlx::query(
        r#"
        INSERT INTO order_service_items (order_id, service_id, quantity, unit_price)
        SELECT
            $1,
            services.id,
            $2,
            services.base_price
        FROM services
        WHERE services.id = $3
          AND services.is_active = TRUE
        ON CONFLICT (order_id, service_id)
        DO UPDATE SET quantity = EXCLUDED.quantity
        "#,
    )
    .bind(order_id)
    .bind(quantity)
    .bind(service_id)
    .execute(executor)
    .await?;

    Ok(())
}

pub async fn upsert_product_item<'c, E>(
    order_id: Uuid,
    product_id: Uuid,
    quantity: i32,
    executor: E,
) -> RequestResult<()>
where
    E: PgExecutor<'c>,
{
    sqlx::query(
        r#"
        INSERT INTO order_product_items (order_id, product_id, quantity, unit_price)
        SELECT
            $1,
            products.id,
            $2,
            products.unit_price
        FROM products
        WHERE products.id = $3
          AND products.is_active = TRUE
          AND products.stock_qty >= $2
        ON CONFLICT (order_id, product_id)
        DO UPDATE SET quantity = EXCLUDED.quantity
        "#,
    )
    .bind(order_id)
    .bind(quantity)
    .bind(product_id)
    .execute(executor)
    .await?;

    Ok(())
}

pub async fn refresh_order_total<'c, E>(order_id: Uuid, executor: E) -> RequestResult<f64>
where
    E: PgExecutor<'c>,
{
    sqlx::query_scalar::<_, f64>(
        r#"
        UPDATE orders
        SET total_amount = (
                COALESCE((
                    SELECT SUM(quantity * unit_price)
                    FROM order_service_items
                    WHERE order_id = $1
                ), 0)
                +
                COALESCE((
                    SELECT SUM(quantity * unit_price)
                    FROM order_product_items
                    WHERE order_id = $1
                ), 0)
            ),
            updated_at = NOW()
        WHERE id = $1
        RETURNING total_amount::DOUBLE PRECISION
        "#,
    )
    .bind(order_id)
    .fetch_one(executor)
    .await
    .map_err(Into::into)
}

pub async fn finalize_checkout<'c, E>(
    order_id: Uuid,
    request: &CheckoutOrderRequestValid,
    total_amount: f64,
    status_code: &str,
    order_number: &str,
    executor: E,
) -> RequestResult<()>
where
    E: PgExecutor<'c>,
{
    sqlx::query(
        r#"
        UPDATE orders
        SET current_status_id = (
                SELECT id
                FROM order_statuses
                WHERE code = $2
            ),
            order_number = $3,
            contact_name = $4,
            contact_phone = $5,
            contact_email = $6,
            delivery_address = $7,
            total_amount = $8,
            updated_at = NOW()
        WHERE id = $1
        "#,
    )
    .bind(order_id)
    .bind(status_code)
    .bind(order_number)
    .bind(&request.contact_name.0)
    .bind(&request.contact_phone.0)
    .bind(&request.contact_email.0)
    .bind(&request.delivery_address.0)
    .bind(total_amount)
    .execute(executor)
    .await?;

    Ok(())
}

pub async fn update_order_status<'c, E>(
    order_id: Uuid,
    status_code: &str,
    executor: E,
) -> RequestResult<Uuid>
where
    E: PgExecutor<'c>,
{
    sqlx::query_scalar::<_, Uuid>(
        r#"
        UPDATE orders
        SET current_status_id = (
                SELECT id
                FROM order_statuses
                WHERE code = $2
            ),
            updated_at = NOW()
        WHERE id = $1
        RETURNING current_status_id
        "#,
    )
    .bind(order_id)
    .bind(status_code)
    .fetch_one(executor)
    .await
    .map_err(Into::into)
}

pub async fn get_order_row<'c, E>(order_id: Uuid, executor: E) -> RequestResult<OrderRow>
where
    E: PgExecutor<'c>,
{
    sqlx::query_as::<_, OrderRow>(
        r#"
        SELECT
            orders.id,
            orders.order_number,
            orders.user_id,
            order_statuses.code AS current_status_code,
            orders.contact_name,
            orders.contact_phone,
            orders.contact_email,
            orders.delivery_address,
            orders.total_amount::DOUBLE PRECISION AS total_amount
        FROM orders
        INNER JOIN order_statuses
            ON order_statuses.id = orders.current_status_id
        WHERE orders.id = $1
        "#,
    )
    .bind(order_id)
    .fetch_one(executor)
    .await
    .map_err(Into::into)
}

pub async fn get_order_summaries_for_user<'c, E>(
    user_id: Uuid,
    executor: E,
) -> RequestResult<Vec<OrderSummary>>
where
    E: PgExecutor<'c>,
{
    sqlx::query_as::<_, OrderSummary>(
        r#"
        SELECT
            orders.id,
            orders.order_number,
            order_statuses.code AS current_status_code,
            orders.total_amount::DOUBLE PRECISION AS total_amount
        FROM orders
        INNER JOIN order_statuses
            ON order_statuses.id = orders.current_status_id
        WHERE orders.user_id = $1
          AND order_statuses.code <> 'draft'
        ORDER BY orders.created_at DESC
        "#,
    )
    .bind(user_id)
    .fetch_all(executor)
    .await
    .map_err(Into::into)
}

pub async fn get_order_items<'c, E>(
    order_id: Uuid,
    executor: E,
) -> RequestResult<Vec<OrderItemResponse>>
where
    E: PgExecutor<'c>,
{
    let mut items = sqlx::query_as::<_, OrderItemRow>(
        r#"
        SELECT
            order_service_items.id AS item_id,
            'service' AS item_type,
            order_service_items.service_id AS reference_id,
            services.name AS title,
            order_service_items.quantity,
            order_service_items.unit_price::DOUBLE PRECISION AS unit_price
        FROM order_service_items
        INNER JOIN services ON services.id = order_service_items.service_id
        WHERE order_service_items.order_id = $1

        UNION ALL

        SELECT
            order_product_items.id AS item_id,
            'product' AS item_type,
            order_product_items.product_id AS reference_id,
            products.name AS title,
            order_product_items.quantity,
            order_product_items.unit_price::DOUBLE PRECISION AS unit_price
        FROM order_product_items
        INNER JOIN products ON products.id = order_product_items.product_id
        WHERE order_product_items.order_id = $1
        "#,
    )
    .bind(order_id)
    .fetch_all(executor)
    .await?
    .into_iter()
    .map(OrderItemResponse::from)
    .collect::<Vec<_>>();

    items.sort_by(|left, right| left.title.cmp(&right.title));
    Ok(items)
}

pub async fn remove_service_item<'c, E>(
    order_id: Uuid,
    service_id: Uuid,
    executor: E,
) -> RequestResult<()>
where
    E: PgExecutor<'c>,
{
    sqlx::query(
        r#"
        DELETE FROM order_service_items
        WHERE order_id = $1
          AND service_id = $2
        "#,
    )
    .bind(order_id)
    .bind(service_id)
    .execute(executor)
    .await?;

    Ok(())
}

pub async fn remove_product_item<'c, E>(
    order_id: Uuid,
    product_id: Uuid,
    executor: E,
) -> RequestResult<()>
where
    E: PgExecutor<'c>,
{
    sqlx::query(
        r#"
        DELETE FROM order_product_items
        WHERE order_id = $1
          AND product_id = $2
        "#,
    )
    .bind(order_id)
    .bind(product_id)
    .execute(executor)
    .await?;

    Ok(())
}

pub async fn ensure_order_owner<'c, E>(
    order_id: Uuid,
    user_id: Uuid,
    executor: E,
) -> RequestResult<()>
where
    E: PgExecutor<'c>,
{
    let owner = sqlx::query_scalar::<_, Uuid>(
        r#"
        SELECT user_id
        FROM orders
        WHERE id = $1
        "#,
    )
    .bind(order_id)
    .fetch_one(executor)
    .await?;

    if owner != user_id {
        return Err(RequestError::forbidden(
            "order does not belong to the current user",
        ));
    }

    Ok(())
}
