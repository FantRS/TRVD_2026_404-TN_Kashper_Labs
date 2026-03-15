use chrono::{Days, Duration, NaiveTime, TimeZone, Utc};
use uuid::Uuid;

use crate::app::domains::schedule::models::{
    AppointmentResponse, AvailableSlotResponse, AvailableSlotsQuery, CreateAppointmentRequestValid,
};
use crate::app::domains::schedule::repository;
use crate::app::events::{DomainEvent, publish};
use crate::app::{RequestError, RequestResult, ServiceContext};

pub async fn get_available_slots(
    query: &AvailableSlotsQuery,
    ctx: &ServiceContext<'_>,
) -> RequestResult<Vec<AvailableSlotResponse>> {
    let (date_from, date_to) = query.parse_range()?;
    let booked = repository::find_appointments_in_range(date_from, date_to, ctx.db_pool).await?;
    let booked_slots = booked
        .into_iter()
        .map(|appointment| appointment.scheduled_at)
        .collect::<std::collections::HashSet<_>>();

    let mut current_day = date_from.date_naive();
    let end_day = date_to.date_naive();
    let mut slots = Vec::new();

    while current_day <= end_day {
        let start_time = NaiveTime::from_hms_opt(ctx.business_hours.start_hour, 0, 0)
            .ok_or_else(|| RequestError::internal_server_error("invalid business start hour"))?;
        let end_time = NaiveTime::from_hms_opt(ctx.business_hours.end_hour, 0, 0)
            .ok_or_else(|| RequestError::internal_server_error("invalid business end hour"))?;
        let mut slot = Utc.from_utc_datetime(&current_day.and_time(start_time));
        let end_slot = Utc.from_utc_datetime(&current_day.and_time(end_time));

        while slot < end_slot {
            if slot >= date_from && slot <= date_to {
                slots.push(AvailableSlotResponse {
                    scheduled_at: slot.to_rfc3339(),
                    is_available: !booked_slots.contains(&slot),
                });
            }

            slot += Duration::minutes(ctx.business_hours.slot_minutes);
        }

        current_day = current_day
            .checked_add_days(Days::new(1))
            .ok_or_else(|| RequestError::internal_server_error("failed to increment date"))?;
    }

    Ok(slots)
}

pub async fn create_appointment(
    order_id: Uuid,
    request: CreateAppointmentRequestValid,
    ctx: &ServiceContext<'_>,
) -> RequestResult<AppointmentResponse> {
    let appointment = repository::create_appointment(
        order_id,
        request.scheduled_at,
        &request.location,
        ctx.db_pool,
    )
    .await?;

    Ok(appointment.into())
}

pub async fn assign_employee(
    id: Uuid,
    employee_user_id: Uuid,
    _actor_user_id: Uuid,
    ctx: &ServiceContext<'_>,
) -> RequestResult<AppointmentResponse> {
    let appointment = repository::assign_employee(id, employee_user_id, ctx.db_pool).await?;

    let _ = publish(
        DomainEvent::AppointmentConfirmed {
            order_id: appointment.order_id,
            appointment_id: appointment.id,
        },
        ctx.redis,
    )
    .await;

    Ok(appointment.into())
}

pub async fn get_employee_day_plan(
    employee_user_id: Uuid,
    day: chrono::NaiveDate,
    ctx: &ServiceContext<'_>,
) -> RequestResult<Vec<AppointmentResponse>> {
    let starts_at = Utc.from_utc_datetime(
        &day.and_hms_opt(0, 0, 0)
            .ok_or_else(|| RequestError::internal_server_error("invalid start of day"))?,
    );
    let ends_at = starts_at + Duration::days(1);

    repository::get_employee_day_plan(employee_user_id, starts_at, ends_at, ctx.db_pool)
        .await
        .map(|rows| rows.into_iter().map(AppointmentResponse::from).collect())
}
