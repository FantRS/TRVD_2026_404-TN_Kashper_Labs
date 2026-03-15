use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Serialize, ToSchema)]
#[serde(tag = "event_type", content = "payload")]
pub enum DomainEvent {
    OrderCreated {
        order_id: Uuid,
        order_number: String,
    },
    OrderStatusChanged {
        order_id: Uuid,
        status_code: String,
    },
    PaymentCreated {
        order_id: Uuid,
        payment_id: Uuid,
    },
    PaymentUpdated {
        order_id: Uuid,
        payment_id: Uuid,
        status: String,
    },
    AppointmentConfirmed {
        order_id: Uuid,
        appointment_id: Uuid,
    },
    UserRoleChanged {
        user_id: Uuid,
        role: String,
    },
}

impl DomainEvent {
    pub fn event_name(&self) -> &'static str {
        match self {
            DomainEvent::OrderCreated { .. } => "order.created",
            DomainEvent::OrderStatusChanged { .. } => "order.status_changed",
            DomainEvent::PaymentCreated { .. } => "payment.created",
            DomainEvent::PaymentUpdated { .. } => "payment.updated",
            DomainEvent::AppointmentConfirmed { .. } => "appointment.confirmed",
            DomainEvent::UserRoleChanged { .. } => "user.role_changed",
        }
    }
}
