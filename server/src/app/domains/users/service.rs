use serde_json::json;
use uuid::Uuid;

use crate::app::domains::auth::models::UserRole;
use crate::app::domains::users::models::{UserAdminResponse, UsersFilterParams};
use crate::app::domains::users::repository;
use crate::app::events::{DomainEvent, publish};
use crate::app::redis::token_wl_service;
use crate::app::utils::pagination::{PaginatedResponse, PaginationParams};
use crate::app::{RequestResult, ServiceContext};

pub async fn get_users(
    filters: &UsersFilterParams,
    ctx: &ServiceContext<'_>,
) -> RequestResult<PaginatedResponse<UserAdminResponse>> {
    let params = PaginationParams {
        page: filters.page(),
        per_page: filters.per_page(),
        search: filters.search.clone(),
    };
    let rows = repository::find_users_paginated(
        params.page,
        params.per_page_capped(),
        filters,
        ctx.db_pool,
    )
    .await?;
    let total = rows.first().and_then(|row| row.total_count).unwrap_or(0);
    let data = rows
        .into_iter()
        .map(UserAdminResponse::try_from)
        .collect::<Result<Vec<_>, _>>()?;

    Ok(PaginatedResponse::new(data, &params, total))
}

pub async fn update_user_role(
    actor_user_id: Uuid,
    id: Uuid,
    role: UserRole,
    ctx: &ServiceContext<'_>,
) -> RequestResult<UserAdminResponse> {
    let mut tx = ctx.db_pool.begin().await?;
    let user = repository::update_user_role(id, role, &mut *tx).await?;
    repository::create_audit_log(
        Some(actor_user_id),
        "users",
        Some(id),
        "role_updated",
        &json!({ "role": user.role.as_str() }),
        &mut *tx,
    )
    .await?;
    tx.commit().await?;

    let _ = publish(
        DomainEvent::UserRoleChanged {
            user_id: user.id,
            role: user.role.as_str().to_owned(),
        },
        ctx.redis,
    )
    .await;

    Ok(user)
}

pub async fn set_user_active_state(
    actor_user_id: Uuid,
    id: Uuid,
    is_active: bool,
    ctx: &ServiceContext<'_>,
) -> RequestResult<UserAdminResponse> {
    let mut tx = ctx.db_pool.begin().await?;
    let user = repository::set_user_active_state(id, is_active, &mut *tx).await?;
    repository::create_audit_log(
        Some(actor_user_id),
        "users",
        Some(id),
        "active_state_updated",
        &json!({ "is_active": user.is_active }),
        &mut *tx,
    )
    .await?;
    tx.commit().await?;

    if !is_active {
        let _ = token_wl_service::invalidate_user_tokens(id, ctx).await;
    }

    Ok(user)
}
