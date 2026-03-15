use actix_web::{HttpResponse, Responder, web};
use uuid::Uuid;

use crate::app::domains::auth::models::Claims;
use crate::app::domains::schedule::models::{
    AssignEmployeeRequest, AvailableSlotsQuery, CreateAppointmentRequest, EmployeeDayPlanQuery,
};
use crate::app::domains::schedule::service;
use crate::app::{AppData, RequestResult, ServiceContext};

/// Повертає список доступних і зайнятих слотів у заданому діапазоні (`гості`, `user`, `employee`, `admin`).
#[utoipa::path(
    get,
    path = "/api/schedule/slots",
    params(AvailableSlotsQuery),
    responses((status = 200, body = [crate::app::domains::schedule::models::AvailableSlotResponse])),
    tag = "Schedule"
)]
#[tracing::instrument(name = "get_available_slots", skip_all, fields(request_id = %Uuid::new_v4()))]
pub async fn get_available_slots(
    query: web::Query<AvailableSlotsQuery>,
    app_data: web::Data<AppData>,
) -> RequestResult<impl Responder> {
    let ctx = ServiceContext::from(app_data.get_ref());
    let response = service::get_available_slots(&query.into_inner(), &ctx).await;

    match &response {
        Ok(_) => tracing::info!("Available slots received successfully"),
        Err(error) => tracing::error!("Available slots receive failed: {error}"),
    }

    Ok(HttpResponse::Ok().json(response?))
}

/// Створює бронювання візиту для замовлення користувача (`user`).
#[utoipa::path(
    post,
    path = "/api/orders/{id}/appointment",
    params(("id" = Uuid, Path, description = "Order id")),
    request_body = CreateAppointmentRequest,
    responses((status = 201, body = crate::app::domains::schedule::models::AppointmentResponse)),
    security(("bearer_auth" = [])),
    tag = "Schedule"
)]
#[tracing::instrument(name = "create_appointment", skip_all, fields(request_id = %Uuid::new_v4(), order_id = %id))]
pub async fn create_appointment(
    id: web::Path<Uuid>,
    request: web::Json<CreateAppointmentRequest>,
    app_data: web::Data<AppData>,
) -> RequestResult<impl Responder> {
    let id = id.into_inner();
    let request = request.into_inner().try_into()?;
    let ctx = ServiceContext::from(app_data.get_ref());
    let response = service::create_appointment(id, request, &ctx).await;

    match &response {
        Ok(_) => tracing::info!("Appointment created successfully"),
        Err(error) => tracing::error!("Appointment creation failed: {error}"),
    }

    Ok(HttpResponse::Created().json(response?))
}

/// Призначає працівника на вже створений запис (`employee`, `admin`).
#[utoipa::path(
    patch,
    path = "/api/schedule/appointments/{id}/assign",
    params(("id" = Uuid, Path, description = "Appointment id")),
    request_body = AssignEmployeeRequest,
    responses((status = 200, body = crate::app::domains::schedule::models::AppointmentResponse)),
    security(("bearer_auth" = [])),
    tag = "Schedule"
)]
#[tracing::instrument(name = "assign_employee", skip_all, fields(request_id = %Uuid::new_v4(), appointment_id = %id))]
pub async fn assign_employee(
    id: web::Path<Uuid>,
    claims: web::ReqData<Claims>,
    request: web::Json<AssignEmployeeRequest>,
    app_data: web::Data<AppData>,
) -> RequestResult<impl Responder> {
    let id = id.into_inner();
    let request = request.into_inner();
    let ctx = ServiceContext::from(app_data.get_ref());
    let response = service::assign_employee(id, request.employee_user_id, claims.sub, &ctx).await;

    match &response {
        Ok(_) => tracing::info!("Employee assigned successfully"),
        Err(error) => tracing::error!("Employee assignment failed: {error}"),
    }

    Ok(HttpResponse::Ok().json(response?))
}

/// Повертає денний план записів для конкретного працівника (`employee`, `admin`).
#[utoipa::path(
    get,
    path = "/api/schedule/employees/{id}/plan",
    params(
        ("id" = Uuid, Path, description = "Employee user id"),
        EmployeeDayPlanQuery
    ),
    responses((status = 200, body = [crate::app::domains::schedule::models::AppointmentResponse])),
    security(("bearer_auth" = [])),
    tag = "Schedule"
)]
#[tracing::instrument(name = "get_employee_day_plan", skip_all, fields(request_id = %Uuid::new_v4(), employee_user_id = %id))]
pub async fn get_employee_day_plan(
    id: web::Path<Uuid>,
    query: web::Query<EmployeeDayPlanQuery>,
    app_data: web::Data<AppData>,
) -> RequestResult<impl Responder> {
    let id = id.into_inner();
    let day = query.parse_day()?;
    let ctx = ServiceContext::from(app_data.get_ref());
    let response = service::get_employee_day_plan(id, day, &ctx).await;

    match &response {
        Ok(_) => tracing::info!("Employee day plan received successfully"),
        Err(error) => tracing::error!("Employee day plan receive failed: {error}"),
    }

    Ok(HttpResponse::Ok().json(response?))
}
