use serde_json::to_string;

use crate::app::events::domain_event::DomainEvent;
use crate::app::redis::client::RedisClient;
use crate::app::{RequestError, RequestResult};

const DOMAIN_EVENTS_STREAM: &str = "domain:events";

pub async fn publish(event: DomainEvent, redis: &RedisClient) -> RequestResult<String> {
    let payload = to_string(&event)
        .map_err(|error| RequestError::internal_server_error(error.to_string()))?;

    redis
        .xadd(
            DOMAIN_EVENTS_STREAM,
            &[("event_name", event.event_name()), ("payload", &payload)],
        )
        .await
}
