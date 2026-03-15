use actix_web::{HttpResponse, Responder, web};
use uuid::Uuid;

use crate::app::domains::auth::models::{
    Claims, CurrentUserResponse, LoginRequest, RegisterRequest,
};
use crate::app::domains::auth::service;
use crate::app::{AppData, RequestResult, ServiceContext};

/// Реєструє нового користувача та одразу повертає токен доступу (`гості`).
#[utoipa::path(
    post,
    path = "/api/auth/register",
    request_body = RegisterRequest,
    responses(
        (status = 201, description = "User registered", body = crate::app::domains::auth::models::LoginResponse),
        (status = 409, description = "User already exists"),
        (status = 422, description = "Validation error")
    ),
    tag = "Auth"
)]
#[tracing::instrument(
    name = "register",
    skip_all,
    fields(request_id = %Uuid::new_v4())
)]
pub async fn register(
    request: web::Json<RegisterRequest>,
    app_data: web::Data<AppData>,
) -> RequestResult<impl Responder> {
    let request = request.into_inner().try_into()?;
    let ctx = ServiceContext::from(app_data.get_ref());
    let response = service::register(request, &ctx).await;

    match &response {
        Ok(_) => tracing::info!("User registered successfully"),
        Err(error) => tracing::error!("User registration failed: {error}"),
    }

    Ok(HttpResponse::Created().json(response?))
}

/// Автентифікує користувача та повертає токен доступу (`гості`).
#[utoipa::path(
    post,
    path = "/api/auth/login",
    request_body = LoginRequest,
    responses(
        (status = 200, description = "User authenticated", body = crate::app::domains::auth::models::LoginResponse),
        (status = 401, description = "Invalid credentials"),
        (status = 422, description = "Validation error")
    ),
    tag = "Auth"
)]
#[tracing::instrument(
    name = "login",
    skip_all,
    fields(request_id = %Uuid::new_v4())
)]
pub async fn login(
    request: web::Json<LoginRequest>,
    app_data: web::Data<AppData>,
) -> RequestResult<impl Responder> {
    let request = request.into_inner().try_into()?;
    let ctx = ServiceContext::from(app_data.get_ref());
    let response = service::login(request, &ctx).await;

    match &response {
        Ok(_) => tracing::info!("User authenticated successfully"),
        Err(error) => tracing::error!("User authentication failed: {error}"),
    }

    Ok(HttpResponse::Ok().json(response?))
}

/// Інвалідовує поточний access token (`user`, `employee`, `admin`).
#[utoipa::path(
    post,
    path = "/api/auth/logout",
    responses(
        (status = 200, description = "Current token invalidated"),
        (status = 401, description = "Authentication required")
    ),
    security(("bearer_auth" = [])),
    tag = "Auth"
)]
#[tracing::instrument(
    name = "logout",
    skip_all,
    fields(request_id = %Uuid::new_v4())
)]
pub async fn logout(
    claims: web::ReqData<Claims>,
    app_data: web::Data<AppData>,
) -> RequestResult<impl Responder> {
    let ctx = ServiceContext::from(app_data.get_ref());
    let response = service::logout(&claims, &ctx).await;

    match &response {
        Ok(_) => tracing::info!("User logged out successfully"),
        Err(error) => tracing::error!("User logout failed: {error}"),
    }

    response?;
    Ok(HttpResponse::Ok().finish())
}

/// Інвалідовує всі активні токени поточного користувача (`user`, `employee`, `admin`).
#[utoipa::path(
    post,
    path = "/api/auth/logout-all",
    responses(
        (status = 200, description = "All user tokens invalidated"),
        (status = 401, description = "Authentication required")
    ),
    security(("bearer_auth" = [])),
    tag = "Auth"
)]
#[tracing::instrument(
    name = "logout_all",
    skip_all,
    fields(request_id = %Uuid::new_v4())
)]
pub async fn logout_all(
    claims: web::ReqData<Claims>,
    app_data: web::Data<AppData>,
) -> RequestResult<impl Responder> {
    let ctx = ServiceContext::from(app_data.get_ref());
    let response = service::logout_all(&claims, &ctx).await;

    match &response {
        Ok(count) => tracing::info!("All user tokens invalidated: {count}"),
        Err(error) => tracing::error!("Logout-all failed: {error}"),
    }

    Ok(HttpResponse::Ok().json(serde_json::json!({ "invalidated_tokens": response? })))
}

/// Повертає профіль поточного авторизованого користувача (`user`, `employee`, `admin`).
#[utoipa::path(
    get,
    path = "/api/auth/me",
    responses(
        (status = 200, description = "Current user returned", body = CurrentUserResponse),
        (status = 401, description = "Authentication required")
    ),
    security(("bearer_auth" = [])),
    tag = "Auth"
)]
#[tracing::instrument(
    name = "me",
    skip_all,
    fields(request_id = %Uuid::new_v4())
)]
pub async fn me(
    claims: web::ReqData<Claims>,
    app_data: web::Data<AppData>,
) -> RequestResult<impl Responder> {
    let ctx = ServiceContext::from(app_data.get_ref());
    let response = service::get_current_user(claims.sub, &ctx).await;

    match &response {
        Ok(_) => tracing::info!("Current user received successfully"),
        Err(error) => tracing::error!("Current user receive failed: {error}"),
    }

    Ok(HttpResponse::Ok().json(CurrentUserResponse { user: response? }))
}
