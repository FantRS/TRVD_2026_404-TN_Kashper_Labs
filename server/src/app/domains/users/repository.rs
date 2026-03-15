use serde_json::Value;
use sqlx::{PgExecutor, Postgres, QueryBuilder};
use uuid::Uuid;

use crate::app::RequestResult;
use crate::app::domains::auth::models::UserRole;
use crate::app::domains::users::models::{UserAdminResponse, UserAdminRow, UsersFilterParams};

pub async fn find_users_paginated<'c, E>(
    page: u32,
    per_page: u32,
    filters: &UsersFilterParams,
    executor: E,
) -> RequestResult<Vec<UserAdminRow>>
where
    E: PgExecutor<'c>,
{
    let mut qb = QueryBuilder::<Postgres>::new(
        r#"
        SELECT
            users.id,
            users.email,
            users.full_name,
            users.phone,
            users.wallet_balance::DOUBLE PRECISION AS wallet_balance,
            roles.code AS role_code,
            users.is_active,
            COUNT(*) OVER() AS total_count
        FROM users
        INNER JOIN roles ON roles.id = users.role_id
        WHERE 1 = 1
        "#,
    );

    if let Some(search) = filters.search.as_ref().map(|search| search.trim()) {
        if !search.is_empty() {
            qb.push(" AND (users.email ILIKE ");
            qb.push_bind(format!("%{search}%"));
            qb.push(" OR users.full_name ILIKE ");
            qb.push_bind(format!("%{search}%"));
            qb.push(")");
        }
    }

    if let Some(role) = filters.role {
        qb.push(" AND roles.code = ");
        qb.push_bind(role.as_str());
    }

    if let Some(is_active) = filters.is_active {
        qb.push(" AND users.is_active = ");
        qb.push_bind(is_active);
    }

    qb.push(" ORDER BY users.created_at DESC LIMIT ");
    qb.push_bind(i64::from(per_page));
    qb.push(" OFFSET ");
    qb.push_bind(i64::from(page.saturating_sub(1) * per_page));

    qb.build_query_as::<UserAdminRow>()
        .fetch_all(executor)
        .await
        .map_err(Into::into)
}

pub async fn update_user_role<'c, E>(
    id: Uuid,
    role: UserRole,
    executor: E,
) -> RequestResult<UserAdminResponse>
where
    E: PgExecutor<'c>,
{
    sqlx::query_as::<_, UserAdminRow>(
        r#"
        UPDATE users
        SET role_id = (
                SELECT id
                FROM roles
                WHERE code = $2
            ),
            updated_at = NOW()
        WHERE users.id = $1
        RETURNING
            users.id,
            users.email,
            users.full_name,
            users.phone,
            users.wallet_balance::DOUBLE PRECISION AS wallet_balance,
            $2 AS role_code,
            users.is_active,
            NULL::BIGINT AS total_count
        "#,
    )
    .bind(id)
    .bind(role.as_str())
    .fetch_one(executor)
    .await?
    .try_into()
}

pub async fn set_user_active_state<'c, E>(
    id: Uuid,
    is_active: bool,
    executor: E,
) -> RequestResult<UserAdminResponse>
where
    E: PgExecutor<'c>,
{
    sqlx::query_as::<_, UserAdminRow>(
        r#"
        UPDATE users
        SET is_active = $2,
            updated_at = NOW()
        FROM roles
        WHERE users.id = $1
          AND roles.id = users.role_id
        RETURNING
            users.id,
            users.email,
            users.full_name,
            users.phone,
            users.wallet_balance::DOUBLE PRECISION AS wallet_balance,
            roles.code AS role_code,
            users.is_active,
            NULL::BIGINT AS total_count
        "#,
    )
    .bind(id)
    .bind(is_active)
    .fetch_one(executor)
    .await?
    .try_into()
}

pub async fn create_audit_log<'c, E>(
    actor_user_id: Option<Uuid>,
    entity_name: &str,
    entity_id: Option<Uuid>,
    action: &str,
    details: &Value,
    executor: E,
) -> RequestResult<()>
where
    E: PgExecutor<'c>,
{
    sqlx::query(
        r#"
        INSERT INTO audit_logs (
            actor_user_id,
            entity_name,
            entity_id,
            action,
            details
        )
        VALUES ($1, $2, $3, $4, $5)
        "#,
    )
    .bind(actor_user_id)
    .bind(entity_name)
    .bind(entity_id)
    .bind(action)
    .bind(details)
    .execute(executor)
    .await?;

    Ok(())
}
