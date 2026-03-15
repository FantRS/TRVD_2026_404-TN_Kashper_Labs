use chrono::{DateTime, Utc};
use serde::Deserialize;
use utoipa::IntoParams;

use crate::app::RequestError;

#[derive(Debug, Deserialize, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct ReportPeriodParams {
    pub date_from: String,
    pub date_to: String,
}

impl ReportPeriodParams {
    pub fn parse(&self) -> Result<(DateTime<Utc>, DateTime<Utc>), RequestError> {
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
