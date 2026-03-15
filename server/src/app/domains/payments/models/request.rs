use serde::Deserialize;
use utoipa::ToSchema;

use crate::app::RequestError;
use crate::app::utils::validation::trimmed_optional;

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreatePaymentRequest {
    pub comment: Option<String>,
}

impl CreatePaymentRequest {
    pub fn validate(self) -> Result<Self, RequestError> {
        Ok(Self {
            comment: trimmed_optional(self.comment, "comment", 2_000)?,
        })
    }
}
