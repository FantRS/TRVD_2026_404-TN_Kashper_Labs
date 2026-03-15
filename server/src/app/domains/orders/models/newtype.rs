use chrono::Utc;

use crate::app::RequestError;
use crate::app::utils::validation::{normalized_email, phone_number, trimmed_required};

#[derive(Debug, Clone)]
pub struct ContactFullName(pub String);

impl TryFrom<String> for ContactFullName {
    type Error = RequestError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Ok(Self(trimmed_required(&value, "contact_name", 3, 255)?))
    }
}

#[derive(Debug, Clone)]
pub struct PhoneNumber(pub String);

impl TryFrom<String> for PhoneNumber {
    type Error = RequestError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Ok(Self(phone_number(&value, "contact_phone")?))
    }
}

#[derive(Debug, Clone)]
pub struct EmailAddress(pub String);

impl TryFrom<String> for EmailAddress {
    type Error = RequestError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Ok(Self(normalized_email(&value, "contact_email")?))
    }
}

#[derive(Debug, Clone)]
pub struct PostalAddress(pub String);

impl TryFrom<String> for PostalAddress {
    type Error = RequestError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Ok(Self(trimmed_required(&value, "delivery_address", 5, 500)?))
    }
}

#[derive(Debug, Clone)]
pub struct OrderNumber(pub String);

impl OrderNumber {
    pub fn generate() -> Self {
        let timestamp = Utc::now().format("%Y%m%d%H%M%S");
        let suffix = uuid::Uuid::now_v7().simple().to_string();
        Self(format!("ORD-{timestamp}-{}", &suffix[..8]))
    }
}

#[derive(Debug, Clone)]
pub struct OrderStatusCode(pub String);

impl TryFrom<String> for OrderStatusCode {
    type Error = RequestError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Ok(Self(trimmed_required(&value, "status_code", 3, 64)?))
    }
}
