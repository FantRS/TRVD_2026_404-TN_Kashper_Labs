use serde::Deserialize;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::app::RequestError;
use crate::app::domains::orders::models::{
    ContactFullName, EmailAddress, OrderStatusCode, PhoneNumber, PostalAddress,
};
use crate::app::utils::validation::{positive_i32, trimmed_optional};

#[derive(Debug, Deserialize, ToSchema)]
pub struct AddServiceToCartRequest {
    pub service_id: Uuid,
    pub quantity: i32,
}

impl AddServiceToCartRequest {
    pub fn validate(self) -> Result<Self, RequestError> {
        positive_i32(self.quantity, "quantity")?;
        Ok(self)
    }
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct AddProductToCartRequest {
    pub product_id: Uuid,
    pub quantity: i32,
}

impl AddProductToCartRequest {
    pub fn validate(self) -> Result<Self, RequestError> {
        positive_i32(self.quantity, "quantity")?;
        Ok(self)
    }
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CheckoutOrderRequest {
    pub contact_name: String,
    pub contact_phone: String,
    pub contact_email: String,
    pub delivery_address: String,
}

#[derive(Debug)]
pub struct CheckoutOrderRequestValid {
    pub contact_name: ContactFullName,
    pub contact_phone: PhoneNumber,
    pub contact_email: EmailAddress,
    pub delivery_address: PostalAddress,
}

impl TryFrom<CheckoutOrderRequest> for CheckoutOrderRequestValid {
    type Error = RequestError;

    fn try_from(value: CheckoutOrderRequest) -> Result<Self, Self::Error> {
        Ok(Self {
            contact_name: value.contact_name.try_into()?,
            contact_phone: value.contact_phone.try_into()?,
            contact_email: value.contact_email.try_into()?,
            delivery_address: value.delivery_address.try_into()?,
        })
    }
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct EmployeeOrderStatusUpdateRequest {
    pub status_code: String,
    pub comment: Option<String>,
}

#[derive(Debug)]
pub struct EmployeeOrderStatusUpdateRequestValid {
    pub status_code: OrderStatusCode,
    pub comment: Option<String>,
}

impl TryFrom<EmployeeOrderStatusUpdateRequest> for EmployeeOrderStatusUpdateRequestValid {
    type Error = RequestError;

    fn try_from(value: EmployeeOrderStatusUpdateRequest) -> Result<Self, Self::Error> {
        Ok(Self {
            status_code: value.status_code.try_into()?,
            comment: trimmed_optional(value.comment, "comment", 2_000)?,
        })
    }
}
