use chrono::{DateTime, NaiveDate, Utc};
use serde::Deserialize;
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;

use crate::app::RequestError;
use crate::app::utils::validation::trimmed_required;

#[derive(Debug, Deserialize, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct AvailableSlotsQuery {
    pub date_from: String,
    pub date_to: String,
}

impl AvailableSlotsQuery {
    pub fn parse_range(&self) -> Result<(DateTime<Utc>, DateTime<Utc>), RequestError> {
        let date_from = DateTime::parse_from_rfc3339(self.date_from.trim())
            .map_err(|_| RequestError::unprocessable_entity("date_from must be RFC3339"))?
            .with_timezone(&Utc);
        let date_to = DateTime::parse_from_rfc3339(self.date_to.trim())
            .map_err(|_| RequestError::unprocessable_entity("date_to must be RFC3339"))?
            .with_timezone(&Utc);

        if date_to < date_from {
            return Err(RequestError::unprocessable_entity(
                "date_to must be greater than or equal to date_from",
            ));
        }

        Ok((date_from, date_to))
    }
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateAppointmentRequest {
    pub scheduled_at: String,
    pub location: String,
}

#[derive(Debug)]
pub struct CreateAppointmentRequestValid {
    pub scheduled_at: DateTime<Utc>,
    pub location: String,
}

impl TryFrom<CreateAppointmentRequest> for CreateAppointmentRequestValid {
    type Error = RequestError;

    fn try_from(value: CreateAppointmentRequest) -> Result<Self, Self::Error> {
        Ok(Self {
            scheduled_at: DateTime::parse_from_rfc3339(value.scheduled_at.trim())
                .map_err(|_| RequestError::unprocessable_entity("scheduled_at must be RFC3339"))?
                .with_timezone(&Utc),
            location: trimmed_required(&value.location, "location", 3, 255)?,
        })
    }
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct AssignEmployeeRequest {
    pub employee_user_id: Uuid,
}

#[derive(Debug, Deserialize, IntoParams, ToSchema)]
#[into_params(parameter_in = Query)]
pub struct EmployeeDayPlanQuery {
    pub day: String,
}

impl EmployeeDayPlanQuery {
    pub fn parse_day(&self) -> Result<NaiveDate, RequestError> {
        NaiveDate::parse_from_str(self.day.trim(), "%Y-%m-%d")
            .map_err(|_| RequestError::unprocessable_entity("day must be YYYY-MM-DD"))
    }
}
