use actix_web::{
    HttpResponse, ResponseError,
    http::{StatusCode, header::ContentType},
};

pub type RequestResult<T> = Result<T, RequestError>;

#[derive(Debug, thiserror::Error)]
pub enum RequestError {
    /// 400 Bad Request
    #[error("400 Bad Request. Context: {0}")]
    BadRequest(String),

    /// 401 Unauthorized
    #[error("401 Unauthorized. Context: {0}")]
    Unauthorized(String),

    /// 403 Forbidden
    #[error("403 Forbidden. Context: {0}")]
    Forbidden(String),

    /// 404 Not Found
    #[error("404 Not Found. Context: {0}")]
    NotFound(String),

    /// 405 Method Not Allowed
    #[error("405 Method Not Allowed. Context: {0}")]
    MethodNotAllowed(String),

    /// 408 Request Timeout
    #[error("408 Request Timeout. Context: {0}")]
    RequestTimeout(String),

    /// 409 Conflict
    #[error("409 Conflict. Context: {0}")]
    Conflict(String),

    /// 413 Payload Too Large
    #[error("413 Payload Too Large. Context: {0}")]
    PayloadTooLarge(String),

    /// 414 URI Too Long
    #[error("414 URI Too Long. Context: {0}")]
    UriTooLong(String),

    /// 415 Unsupported Media Type
    #[error("415 Unsupported Media Type. Context: {0}")]
    UnsupportedMediaType(String),

    /// 422 Unprocessable Entity
    #[error("422 Unprocessable Entity. Context: {0}")]
    UnprocessableEntity(String),

    /// 429 Too Many Requests
    #[error("429 Too Many Requests. Context: {0}")]
    TooManyRequests(String),

    /// 500 Internal Server Error
    #[error("500 Internal Server Error. Context: {0}")]
    InternalServerError(String),

    /// 501 Not Implemented
    #[error("501 Not Implemented. Context: {0}")]
    NotImplemented(String),

    /// 502 Bad Gateway
    #[error("502 Bad Gateway. Context: {0}")]
    BadGateway(String),

    /// 503 Service Unavailable
    #[error("503 Service Unavailable. Context: {0}")]
    ServiceUnavailable(String),

    /// 504 Gateway Timeout
    #[error("504 Gateway Timeout. Context: {0}")]
    GatewayTimeout(String),
}

impl RequestError {
    /// 400 Bad Request
    pub fn bad_request(msg: impl Into<String>) -> Self {
        RequestError::BadRequest(msg.into())
    }

    /// 401 Unauthorized
    pub fn unauthorized(msg: impl Into<String>) -> Self {
        RequestError::Unauthorized(msg.into())
    }

    /// 403 Forbidden
    pub fn forbidden(msg: impl Into<String>) -> Self {
        RequestError::Forbidden(msg.into())
    }

    /// 404 Not Found
    pub fn not_found(msg: impl Into<String>) -> Self {
        RequestError::NotFound(msg.into())
    }

    /// 405 Method Not Allowed
    pub fn method_not_allowed(msg: impl Into<String>) -> Self {
        RequestError::MethodNotAllowed(msg.into())
    }

    /// 408 Request Timeout
    pub fn request_timeout(msg: impl Into<String>) -> Self {
        RequestError::RequestTimeout(msg.into())
    }

    /// 409 Conflict
    pub fn conflict(msg: impl Into<String>) -> Self {
        RequestError::Conflict(msg.into())
    }

    /// 413 Payload Too Large
    pub fn payload_to_large(msg: impl Into<String>) -> Self {
        RequestError::PayloadTooLarge(msg.into())
    }

    /// 414 URI Too Long
    pub fn uri_to_long(msg: impl Into<String>) -> Self {
        RequestError::UriTooLong(msg.into())
    }

    /// 415 Unsupported Media Type
    pub fn unsupported_media_type(msg: impl Into<String>) -> Self {
        RequestError::UnsupportedMediaType(msg.into())
    }

    /// 422 Unprocessable Entity
    pub fn unprocessable_entity(msg: impl Into<String>) -> Self {
        RequestError::UnprocessableEntity(msg.into())
    }

    /// 429 Too Many Requests
    pub fn to_many_requests(msg: impl Into<String>) -> Self {
        RequestError::TooManyRequests(msg.into())
    }

    /// 500 Internal Server Error
    pub fn internal_server_error(msg: impl Into<String>) -> Self {
        RequestError::InternalServerError(msg.into())
    }

    /// 501 Not Implemented
    pub fn not_implemented(msg: impl Into<String>) -> Self {
        RequestError::NotImplemented(msg.into())
    }

    /// 502 Bad Gateway
    pub fn bad_gateway(msg: impl Into<String>) -> Self {
        RequestError::BadGateway(msg.into())
    }

    /// 503 Service Unavailable
    pub fn service_unavailable(msg: impl Into<String>) -> Self {
        RequestError::ServiceUnavailable(msg.into())
    }

    /// 504 Gateway Timeout
    pub fn gateway_timeout(msg: impl Into<String>) -> Self {
        RequestError::GatewayTimeout(msg.into())
    }
}

impl ResponseError for RequestError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::plaintext())
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            RequestError::BadRequest(_) => StatusCode::BAD_REQUEST,
            RequestError::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            RequestError::Forbidden(_) => StatusCode::FORBIDDEN,
            RequestError::NotFound(_) => StatusCode::NOT_FOUND,
            RequestError::MethodNotAllowed(_) => StatusCode::METHOD_NOT_ALLOWED,
            RequestError::RequestTimeout(_) => StatusCode::REQUEST_TIMEOUT,
            RequestError::Conflict(_) => StatusCode::CONFLICT,
            RequestError::PayloadTooLarge(_) => StatusCode::PAYLOAD_TOO_LARGE,
            RequestError::UriTooLong(_) => StatusCode::URI_TOO_LONG,
            RequestError::UnsupportedMediaType(_) => StatusCode::UNSUPPORTED_MEDIA_TYPE,
            RequestError::UnprocessableEntity(_) => StatusCode::UNPROCESSABLE_ENTITY,
            RequestError::TooManyRequests(_) => StatusCode::TOO_MANY_REQUESTS,

            RequestError::InternalServerError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            RequestError::NotImplemented(_) => StatusCode::NOT_IMPLEMENTED,
            RequestError::BadGateway(_) => StatusCode::BAD_GATEWAY,
            RequestError::ServiceUnavailable(_) => StatusCode::SERVICE_UNAVAILABLE,
            RequestError::GatewayTimeout(_) => StatusCode::GATEWAY_TIMEOUT,
        }
    }
}

impl From<sqlx::Error> for RequestError {
    fn from(error: sqlx::Error) -> Self {
        match &error {
            sqlx::Error::RowNotFound => RequestError::NotFound(error.to_string()),
            sqlx::Error::Database(db_error) => {
                let db_code = db_error.code().unwrap_or_default();
                let error_message = db_error.message();

                match db_code.as_ref() {
                    // Constraint violations
                    "23502" => {
                        RequestError::BadRequest(format!("NOT NULL violation: {}", error_message))
                    }
                    "23503" => RequestError::BadRequest(format!(
                        "Foreign key violation: {}",
                        error_message
                    )),
                    "23505" => RequestError::Conflict(format!(
                        "Unique constraint violation: {}",
                        error_message
                    )),
                    "23514" => RequestError::BadRequest(format!(
                        "Check constraint violation: {}",
                        error_message
                    )),

                    // Syntax errors
                    "42601" => RequestError::InternalServerError(format!(
                        "SQL syntax error: {}",
                        error_message
                    )),
                    "42P01" => RequestError::InternalServerError(format!(
                        "Undefined table: {}",
                        error_message
                    )),
                    "42P02" => RequestError::InternalServerError(format!(
                        "Undefined parameter: {}",
                        error_message
                    )),
                    "42703" => RequestError::InternalServerError(format!(
                        "Undefined column: {}",
                        error_message
                    )),

                    // Connection errors
                    "08003" => RequestError::ServiceUnavailable(format!(
                        "Connection does not exist: {}",
                        error_message
                    )),
                    "08006" => RequestError::ServiceUnavailable(format!(
                        "Connection failure: {}",
                        error_message
                    )),
                    "08001" => RequestError::ServiceUnavailable(format!(
                        "SQL client unable to establish connection: {}",
                        error_message
                    )),
                    "08004" => RequestError::ServiceUnavailable(format!(
                        "SQL server rejected connection: {}",
                        error_message
                    )),
                    "08P01" => {
                        RequestError::BadGateway(format!("Protocol violation: {}", error_message))
                    }

                    // Transaction errors
                    "25P02" => RequestError::InternalServerError(format!(
                        "Transaction in failed state: {}",
                        error_message
                    )),
                    "40001" => {
                        RequestError::Conflict(format!("Serialization failure: {}", error_message))
                    }
                    "40P01" => {
                        RequestError::Conflict(format!("Deadlock detected: {}", error_message))
                    }

                    _ => RequestError::InternalServerError(format!(
                        "Database error [{}]: {}",
                        db_code, error_message
                    )),
                }
            }
            sqlx::Error::PoolTimedOut => {
                RequestError::ServiceUnavailable("Database connection pool timeout".to_string())
            }
            sqlx::Error::PoolClosed => {
                RequestError::ServiceUnavailable("Database connection pool is closed".to_string())
            }
            sqlx::Error::WorkerCrashed => {
                RequestError::ServiceUnavailable("Database worker crashed".to_string())
            }
            _ => RequestError::InternalServerError(error.to_string()),
        }
    }
}

impl From<argon2::password_hash::Error> for RequestError {
    fn from(error: argon2::password_hash::Error) -> Self {
        Self::InternalServerError(error.to_string())
    }
}

impl From<jsonwebtoken::errors::Error> for RequestError {
    fn from(error: jsonwebtoken::errors::Error) -> Self {
        Self::Unauthorized(error.to_string())
    }
}

impl From<validator::ValidationErrors> for RequestError {
    fn from(error: validator::ValidationErrors) -> Self {
        Self::BadRequest(error.to_string())
    }
}

impl From<serde_json::Error> for RequestError {
    fn from(error: serde_json::Error) -> Self {
        Self::InternalServerError(error.to_string())
    }
}

impl From<reqwest::Error> for RequestError {
    fn from(error: reqwest::Error) -> Self {
        let context = error.to_string();

        if let Some(status) = error.status() {
            return map_upstream_status(status, context);
        }

        if error.is_timeout() {
            return RequestError::GatewayTimeout(format!("Upstream request timed out: {context}"));
        }

        if error.is_connect() {
            return RequestError::ServiceUnavailable(format!(
                "Cannot connect to upstream service: {context}"
            ));
        }

        if error.is_redirect() {
            return RequestError::BadGateway(format!("Upstream redirect error: {context}"));
        }

        if error.is_body() {
            return RequestError::BadGateway(format!(
                "Error while in/out upstream body: {context}"
            ));
        }

        if error.is_decode() {
            return RequestError::BadGateway(format!("Cannot decode upstream response: {context}"));
        }

        if error.is_upgrade() {
            return RequestError::BadGateway(format!(
                "Upstream connection upgrade failed: {context}"
            ));
        }

        if error.is_builder() {
            return RequestError::InternalServerError(format!(
                "Failed to build upstream request: {context}"
            ));
        }

        if error.is_request() {
            return RequestError::BadGateway(format!("Failed to send upstream request: {context}"));
        }

        RequestError::BadGateway(format!("Unexpected upstream HTTP error: {context}"))
    }
}

fn map_upstream_status(status: reqwest::StatusCode, context: String) -> RequestError {
    let error_text = format!("External API error: {}", context);

    match status.as_u16() {
        400 => RequestError::BadRequest(error_text),
        401 => RequestError::Unauthorized(error_text),
        403 => RequestError::Forbidden(error_text),
        404 => RequestError::NotFound(error_text),
        405 => RequestError::MethodNotAllowed(error_text),
        408 => RequestError::RequestTimeout(error_text),
        409 => RequestError::Conflict(error_text),
        414 => RequestError::UriTooLong(error_text),
        415 => RequestError::UnsupportedMediaType(error_text),
        422 => RequestError::UnprocessableEntity(error_text),
        429 => RequestError::TooManyRequests(error_text),
        503 => RequestError::ServiceUnavailable(error_text),
        504 => RequestError::GatewayTimeout(error_text),
        _ if status.is_client_error() => RequestError::BadRequest(format!(
            "External API error unexpected {} client error: {context}",
            status.as_u16()
        )),
        _ if status.is_server_error() => RequestError::BadGateway(format!(
            "External API error {} server error: {context}",
            status.as_u16()
        )),
        _ => RequestError::InternalServerError(format!(
            "Unexpected upstream status {}: {context}",
            status.as_u16()
        )),
    }
}
