use sqlx::{PgExecutor, Postgres, QueryBuilder};
use uuid::Uuid;

use crate::app::RequestResult;
use crate::app::domains::catalog::models::{
    CatalogFilterParams, CategoryCreateRequestValid, CategoryRow, CategoryUpdateRequestValid,
    ProductCreateRequestValid, ProductResponse, ProductRow, ProductUpdateRequestValid,
    ServiceCreateRequestValid, ServiceResponse, ServiceRow, ServiceUpdateRequestValid,
};

pub async fn find_services_paginated<'c, E>(
    page: u32,
    per_page: u32,
    filters: &CatalogFilterParams,
    executor: E,
) -> RequestResult<Vec<ServiceRow>>
where
    E: PgExecutor<'c>,
{
    let mut qb = QueryBuilder::<Postgres>::new(
        r#"
        SELECT
            services.id,
            services.category_id,
            services.name,
            services.description,
            services.base_price::DOUBLE PRECISION AS base_price,
            services.duration_minutes,
            services.is_active,
            COUNT(*) OVER() AS total_count
        FROM services
        WHERE 1 = 1
        "#,
    );

    if let Some(search) = filters.search.as_ref().map(|search| search.trim()) {
        if !search.is_empty() {
            qb.push(" AND (services.name ILIKE ");
            qb.push_bind(format!("%{search}%"));
            qb.push(" OR COALESCE(services.description, '') ILIKE ");
            qb.push_bind(format!("%{search}%"));
            qb.push(")");
        }
    }

    if let Some(category_id) = filters.category_id {
        qb.push(" AND services.category_id = ");
        qb.push_bind(category_id);
    }

    if let Some(min_price) = filters.min_price {
        qb.push(" AND services.base_price >= ");
        qb.push_bind(min_price);
    }

    if let Some(max_price) = filters.max_price {
        qb.push(" AND services.base_price <= ");
        qb.push_bind(max_price);
    }

    if let Some(only_active) = filters.only_active {
        qb.push(" AND services.is_active = ");
        qb.push_bind(only_active);
    }

    qb.push(" ORDER BY services.created_at DESC LIMIT ");
    qb.push_bind(i64::from(per_page));
    qb.push(" OFFSET ");
    qb.push_bind(i64::from(page.saturating_sub(1) * per_page));

    qb.build_query_as::<ServiceRow>()
        .fetch_all(executor)
        .await
        .map_err(Into::into)
}

pub async fn find_service_by_id<'c, E>(id: Uuid, executor: E) -> RequestResult<ServiceResponse>
where
    E: PgExecutor<'c>,
{
    sqlx::query_as::<_, ServiceRow>(
        r#"
        SELECT
            services.id,
            services.category_id,
            services.name,
            services.description,
            services.base_price::DOUBLE PRECISION AS base_price,
            services.duration_minutes,
            services.is_active,
            NULL::BIGINT AS total_count
        FROM services
        WHERE services.id = $1
        "#,
    )
    .bind(id)
    .fetch_one(executor)
    .await
    .map(ServiceResponse::from)
    .map_err(Into::into)
}

pub async fn create_service<'c, E>(
    request: &ServiceCreateRequestValid,
    executor: E,
) -> RequestResult<ServiceResponse>
where
    E: PgExecutor<'c>,
{
    sqlx::query_as::<_, ServiceRow>(
        r#"
        INSERT INTO services (
            category_id,
            name,
            description,
            base_price,
            duration_minutes
        )
        VALUES ($1, $2, $3, $4, $5)
        RETURNING
            id,
            category_id,
            name,
            description,
            base_price::DOUBLE PRECISION AS base_price,
            duration_minutes,
            is_active,
            NULL::BIGINT AS total_count
        "#,
    )
    .bind(request.category_id)
    .bind(&request.name)
    .bind(&request.description)
    .bind(request.base_price)
    .bind(request.duration_minutes)
    .fetch_one(executor)
    .await
    .map(ServiceResponse::from)
    .map_err(Into::into)
}

pub async fn update_service<'c, E>(
    id: Uuid,
    request: &ServiceUpdateRequestValid,
    executor: E,
) -> RequestResult<ServiceResponse>
where
    E: PgExecutor<'c>,
{
    let mut qb = QueryBuilder::<Postgres>::new("UPDATE services SET ");
    let mut separated = qb.separated(", ");

    if let Some(category_id) = request.category_id {
        separated.push("category_id = ").push_bind(category_id);
    }
    if let Some(name) = request.name.as_ref() {
        separated.push("name = ").push_bind(name);
    }
    if let Some(description) = request.description.as_ref() {
        separated.push("description = ").push_bind(description);
    }
    if let Some(base_price) = request.base_price {
        separated.push("base_price = ").push_bind(base_price);
    }
    if let Some(duration_minutes) = request.duration_minutes {
        separated
            .push("duration_minutes = ")
            .push_bind(duration_minutes);
    }
    if let Some(is_active) = request.is_active {
        separated.push("is_active = ").push_bind(is_active);
    }

    separated.push("updated_at = NOW()");
    qb.push(
        r#"
        WHERE id = 
        "#,
    );
    qb.push_bind(id);
    qb.push(
        r#"
        RETURNING
            id,
            category_id,
            name,
            description,
            base_price::DOUBLE PRECISION AS base_price,
            duration_minutes,
            is_active,
            NULL::BIGINT AS total_count
        "#,
    );

    qb.build_query_as::<ServiceRow>()
        .fetch_one(executor)
        .await
        .map(ServiceResponse::from)
        .map_err(Into::into)
}

pub async fn find_products_paginated<'c, E>(
    page: u32,
    per_page: u32,
    filters: &CatalogFilterParams,
    executor: E,
) -> RequestResult<Vec<ProductRow>>
where
    E: PgExecutor<'c>,
{
    let mut qb = QueryBuilder::<Postgres>::new(
        r#"
        SELECT
            products.id,
            products.category_id,
            products.sku,
            products.name,
            products.description,
            products.unit_price::DOUBLE PRECISION AS unit_price,
            products.stock_qty,
            products.is_active,
            COUNT(*) OVER() AS total_count
        FROM products
        WHERE 1 = 1
        "#,
    );

    if let Some(search) = filters.search.as_ref().map(|search| search.trim()) {
        if !search.is_empty() {
            qb.push(" AND (products.name ILIKE ");
            qb.push_bind(format!("%{search}%"));
            qb.push(" OR COALESCE(products.description, '') ILIKE ");
            qb.push_bind(format!("%{search}%"));
            qb.push(" OR products.sku ILIKE ");
            qb.push_bind(format!("%{search}%"));
            qb.push(")");
        }
    }

    if let Some(category_id) = filters.category_id {
        qb.push(" AND products.category_id = ");
        qb.push_bind(category_id);
    }

    if let Some(min_price) = filters.min_price {
        qb.push(" AND products.unit_price >= ");
        qb.push_bind(min_price);
    }

    if let Some(max_price) = filters.max_price {
        qb.push(" AND products.unit_price <= ");
        qb.push_bind(max_price);
    }

    if let Some(only_active) = filters.only_active {
        qb.push(" AND products.is_active = ");
        qb.push_bind(only_active);
    }

    qb.push(" ORDER BY products.created_at DESC LIMIT ");
    qb.push_bind(i64::from(per_page));
    qb.push(" OFFSET ");
    qb.push_bind(i64::from(page.saturating_sub(1) * per_page));

    qb.build_query_as::<ProductRow>()
        .fetch_all(executor)
        .await
        .map_err(Into::into)
}

pub async fn find_product_by_id<'c, E>(id: Uuid, executor: E) -> RequestResult<ProductResponse>
where
    E: PgExecutor<'c>,
{
    sqlx::query_as::<_, ProductRow>(
        r#"
        SELECT
            products.id,
            products.category_id,
            products.sku,
            products.name,
            products.description,
            products.unit_price::DOUBLE PRECISION AS unit_price,
            products.stock_qty,
            products.is_active,
            NULL::BIGINT AS total_count
        FROM products
        WHERE products.id = $1
        "#,
    )
    .bind(id)
    .fetch_one(executor)
    .await
    .map(ProductResponse::from)
    .map_err(Into::into)
}

pub async fn create_product<'c, E>(
    request: &ProductCreateRequestValid,
    executor: E,
) -> RequestResult<ProductResponse>
where
    E: PgExecutor<'c>,
{
    sqlx::query_as::<_, ProductRow>(
        r#"
        INSERT INTO products (
            category_id,
            sku,
            name,
            description,
            unit_price,
            stock_qty
        )
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING
            id,
            category_id,
            sku,
            name,
            description,
            unit_price::DOUBLE PRECISION AS unit_price,
            stock_qty,
            is_active,
            NULL::BIGINT AS total_count
        "#,
    )
    .bind(request.category_id)
    .bind(&request.sku)
    .bind(&request.name)
    .bind(&request.description)
    .bind(request.unit_price)
    .bind(request.stock_qty)
    .fetch_one(executor)
    .await
    .map(ProductResponse::from)
    .map_err(Into::into)
}

pub async fn update_product<'c, E>(
    id: Uuid,
    request: &ProductUpdateRequestValid,
    executor: E,
) -> RequestResult<ProductResponse>
where
    E: PgExecutor<'c>,
{
    let mut qb = QueryBuilder::<Postgres>::new("UPDATE products SET ");
    let mut separated = qb.separated(", ");

    if let Some(category_id) = request.category_id {
        separated.push("category_id = ").push_bind(category_id);
    }
    if let Some(sku) = request.sku.as_ref() {
        separated.push("sku = ").push_bind(sku);
    }
    if let Some(name) = request.name.as_ref() {
        separated.push("name = ").push_bind(name);
    }
    if let Some(description) = request.description.as_ref() {
        separated.push("description = ").push_bind(description);
    }
    if let Some(unit_price) = request.unit_price {
        separated.push("unit_price = ").push_bind(unit_price);
    }
    if let Some(stock_qty) = request.stock_qty {
        separated.push("stock_qty = ").push_bind(stock_qty);
    }
    if let Some(is_active) = request.is_active {
        separated.push("is_active = ").push_bind(is_active);
    }

    separated.push("updated_at = NOW()");
    qb.push(" WHERE id = ");
    qb.push_bind(id);
    qb.push(
        r#"
        RETURNING
            id,
            category_id,
            sku,
            name,
            description,
            unit_price::DOUBLE PRECISION AS unit_price,
            stock_qty,
            is_active,
            NULL::BIGINT AS total_count
        "#,
    );

    qb.build_query_as::<ProductRow>()
        .fetch_one(executor)
        .await
        .map(ProductResponse::from)
        .map_err(Into::into)
}

pub async fn find_service_categories<'c, E>(executor: E) -> RequestResult<Vec<CategoryRow>>
where
    E: PgExecutor<'c>,
{
    sqlx::query_as::<_, CategoryRow>(
        r#"
        SELECT id, name, description
        FROM service_categories
        ORDER BY name
        "#,
    )
    .fetch_all(executor)
    .await
    .map_err(Into::into)
}

pub async fn create_service_category<'c, E>(
    request: &CategoryCreateRequestValid,
    executor: E,
) -> RequestResult<CategoryRow>
where
    E: PgExecutor<'c>,
{
    sqlx::query_as::<_, CategoryRow>(
        r#"
        INSERT INTO service_categories (name, description)
        VALUES ($1, $2)
        RETURNING id, name, description
        "#,
    )
    .bind(&request.name)
    .bind(&request.description)
    .fetch_one(executor)
    .await
    .map_err(Into::into)
}

pub async fn update_service_category<'c, E>(
    id: Uuid,
    request: &CategoryUpdateRequestValid,
    executor: E,
) -> RequestResult<CategoryRow>
where
    E: PgExecutor<'c>,
{
    let mut qb = QueryBuilder::<Postgres>::new("UPDATE service_categories SET ");
    let mut separated = qb.separated(", ");

    if let Some(name) = request.name.as_ref() {
        separated.push("name = ").push_bind(name);
    }
    if let Some(description) = request.description.as_ref() {
        separated.push("description = ").push_bind(description);
    }

    separated.push("created_at = created_at");
    qb.push(" WHERE id = ");
    qb.push_bind(id);
    qb.push(" RETURNING id, name, description");

    qb.build_query_as::<CategoryRow>()
        .fetch_one(executor)
        .await
        .map_err(Into::into)
}

pub async fn find_product_categories<'c, E>(executor: E) -> RequestResult<Vec<CategoryRow>>
where
    E: PgExecutor<'c>,
{
    sqlx::query_as::<_, CategoryRow>(
        r#"
        SELECT id, name, description
        FROM product_categories
        ORDER BY name
        "#,
    )
    .fetch_all(executor)
    .await
    .map_err(Into::into)
}

pub async fn create_product_category<'c, E>(
    request: &CategoryCreateRequestValid,
    executor: E,
) -> RequestResult<CategoryRow>
where
    E: PgExecutor<'c>,
{
    sqlx::query_as::<_, CategoryRow>(
        r#"
        INSERT INTO product_categories (name, description)
        VALUES ($1, $2)
        RETURNING id, name, description
        "#,
    )
    .bind(&request.name)
    .bind(&request.description)
    .fetch_one(executor)
    .await
    .map_err(Into::into)
}

pub async fn update_product_category<'c, E>(
    id: Uuid,
    request: &CategoryUpdateRequestValid,
    executor: E,
) -> RequestResult<CategoryRow>
where
    E: PgExecutor<'c>,
{
    let mut qb = QueryBuilder::<Postgres>::new("UPDATE product_categories SET ");
    let mut separated = qb.separated(", ");

    if let Some(name) = request.name.as_ref() {
        separated.push("name = ").push_bind(name);
    }
    if let Some(description) = request.description.as_ref() {
        separated.push("description = ").push_bind(description);
    }

    separated.push("created_at = created_at");
    qb.push(" WHERE id = ");
    qb.push_bind(id);
    qb.push(" RETURNING id, name, description");

    qb.build_query_as::<CategoryRow>()
        .fetch_one(executor)
        .await
        .map_err(Into::into)
}
