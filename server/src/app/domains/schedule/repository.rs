use chrono::{DateTime, Utc};
use sqlx::PgExecutor;
use uuid::Uuid;

use crate::app::RequestResult;
use crate::app::domains::schedule::models::AppointmentRow;

pub async fn find_appointments_in_range<'c, E>(
    date_from: DateTime<Utc>,
    date_to: DateTime<Utc>,
    executor: E,
) -> RequestResult<Vec<AppointmentRow>>
where
    E: PgExecutor<'c>,
{
    sqlx::query_as::<_, AppointmentRow>(
        r#"
        SELECT
            id,
            order_id,
            employee_user_id,
            scheduled_at,
            location,
            appointment_status
        FROM appointments
        WHERE scheduled_at >= $1
          AND scheduled_at <= $2
        ORDER BY scheduled_at
        "#,
    )
    .bind(date_from)
    .bind(date_to)
    .fetch_all(executor)
    .await
    .map_err(Into::into)
}

pub async fn create_appointment<'c, E>(
    order_id: Uuid,
    scheduled_at: DateTime<Utc>,
    location: &str,
    executor: E,
) -> RequestResult<AppointmentRow>
where
    E: PgExecutor<'c>,
{
    sqlx::query_as::<_, AppointmentRow>(
        r#"
        INSERT INTO appointments (
            order_id,
            scheduled_at,
            location,
            appointment_status
        )
        VALUES ($1, $2, $3, 'reserved')
        RETURNING
            id,
            order_id,
            employee_user_id,
            scheduled_at,
            location,
            appointment_status
        "#,
    )
    .bind(order_id)
    .bind(scheduled_at)
    .bind(location)
    .fetch_one(executor)
    .await
    .map_err(Into::into)
}

pub async fn assign_employee<'c, E>(
    id: Uuid,
    employee_user_id: Uuid,
    executor: E,
) -> RequestResult<AppointmentRow>
where
    E: PgExecutor<'c>,
{
    sqlx::query_as::<_, AppointmentRow>(
        r#"
        UPDATE appointments
        SET employee_user_id = $2,
            appointment_status = 'confirmed',
            updated_at = NOW()
        WHERE id = $1
        RETURNING
            id,
            order_id,
            employee_user_id,
            scheduled_at,
            location,
            appointment_status
        "#,
    )
    .bind(id)
    .bind(employee_user_id)
    .fetch_one(executor)
    .await
    .map_err(Into::into)
}

pub async fn get_employee_day_plan<'c, E>(
    employee_user_id: Uuid,
    starts_at: DateTime<Utc>,
    ends_at: DateTime<Utc>,
    executor: E,
) -> RequestResult<Vec<AppointmentRow>>
where
    E: PgExecutor<'c>,
{
    sqlx::query_as::<_, AppointmentRow>(
        r#"
        SELECT
            id,
            order_id,
            employee_user_id,
            scheduled_at,
            location,
            appointment_status
        FROM appointments
        WHERE employee_user_id = $1
          AND scheduled_at >= $2
          AND scheduled_at < $3
        ORDER BY scheduled_at
        "#,
    )
    .bind(employee_user_id)
    .bind(starts_at)
    .bind(ends_at)
    .fetch_all(executor)
    .await
    .map_err(Into::into)
}
