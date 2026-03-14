pub mod domains;
pub mod middlewares;
pub mod routes;
pub mod utils;
pub mod redis;

pub use crate::core::app_data::AppData;
pub use utils::request_error::{RequestError, RequestResult};
pub use utils::service_context::ServiceContext;
