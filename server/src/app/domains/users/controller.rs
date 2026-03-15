use actix_web::{HttpResponse, Responder, web};
use uuid::Uuid;

use crate::app::domains::auth::models::Claims;
use crate::app::domains::users::models::{
    UserBlockRequest, UserRoleUpdateRequest, UsersFilterParams,
};
use crate::app::domains::users::service;
use crate::app::{AppData, RequestResult, ServiceContext};

/// Повертає сторінку користувачів для адміністрування (`admin`).
#[utoipa::path(
    get,
    path = "/api/users",
    params(UsersFilterParams),
    responses((status = 200, body = crate::app::utils::pagination::PaginatedResponse<crate::app::domains::users::models::UserAdminResponse>)),
    security(("bearer_auth" = [])),
    tag = "Users"
)]
#[tracing::instrument(name = "get_users", skip_all, fields(request_id = %Uuid::new_v4()))]
pub async fn get_users(
    query: web::Query<UsersFilterParams>,
    app_data: web::Data<AppData>,
) -> RequestResult<impl Responder> {
    let ctx = ServiceContext::from(app_data.get_ref());
    let response = service::get_users(&query.into_inner(), &ctx).await;

    match &response {
        Ok(_) => tracing::info!("Users page received successfully"),
        Err(error) => tracing::error!("Users page receive failed: {error}"),
    }

    Ok(HttpResponse::Ok().json(response?))
}

/// Змінює роль конкретного користувача (`admin`).
#[utoipa::path(
    patch,
    path = "/api/users/{id}/role",
    params(("id" = Uuid, Path, description = "User id")),
    request_body = UserRoleUpdateRequest,
    responses((status = 200, body = crate::app::domains::users::models::UserAdminResponse)),
    security(("bearer_auth" = [])),
    tag = "Users"
)]
#[tracing::instrument(name = "update_user_role", skip_all, fields(request_id = %Uuid::new_v4(), user_id = %id))]
pub async fn update_user_role(
    id: web::Path<Uuid>,
    claims: web::ReqData<Claims>,
    request: web::Json<UserRoleUpdateRequest>,
    app_data: web::Data<AppData>,
) -> RequestResult<impl Responder> {
    let id = id.into_inner();
    let request = request.into_inner();
    let ctx = ServiceContext::from(app_data.get_ref());
    let response = service::update_user_role(claims.sub, id, request.role, &ctx).await;

    match &response {
        Ok(_) => tracing::info!("User role updated successfully"),
        Err(error) => tracing::error!("User role update failed: {error}"),
    }

    Ok(HttpResponse::Ok().json(response?))
}

/// Активує або блокує конкретного користувача (`admin`).
#[utoipa::path(
    patch,
    path = "/api/users/{id}/active-state",
    params(("id" = Uuid, Path, description = "User id")),
    request_body = UserBlockRequest,
    responses((status = 200, body = crate::app::domains::users::models::UserAdminResponse)),
    security(("bearer_auth" = [])),
    tag = "Users"
)]
#[tracing::instrument(name = "block_user", skip_all, fields(request_id = %Uuid::new_v4(), user_id = %id))]
pub async fn block_user(
    id: web::Path<Uuid>,
    claims: web::ReqData<Claims>,
    request: web::Json<UserBlockRequest>,
    app_data: web::Data<AppData>,
) -> RequestResult<impl Responder> {
    let id = id.into_inner();
    let request = request.into_inner();
    let ctx = ServiceContext::from(app_data.get_ref());
    let response = service::set_user_active_state(claims.sub, id, request.is_active, &ctx).await;

    match &response {
        Ok(_) => tracing::info!("User active state updated successfully"),
        Err(error) => tracing::error!("User active state update failed: {error}"),
    }

    Ok(HttpResponse::Ok().json(response?))
}
