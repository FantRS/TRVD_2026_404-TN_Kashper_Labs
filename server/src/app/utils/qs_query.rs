use actix_web::{FromRequest, HttpRequest};
use futures_util::future::{Ready, ready};
use serde::de::DeserializeOwned;
use std::ops::{Deref, DerefMut};

/// Кастомна обгортка QsQuery, яка юзає non-strict mode
#[derive(Debug)]
pub struct QsQuery<T>(pub T);

impl<T> Deref for QsQuery<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for QsQuery<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> QsQuery<T> {
    pub fn into_inner(self) -> T {
        self.0
    }
}

impl<T: DeserializeOwned> FromRequest for QsQuery<T> {
    type Error = actix_web::Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
        let query_str = req.query_string();

        let config = serde_qs::Config::new(5, false); // depth=5, strict=false

        match config.deserialize_str::<T>(query_str) {
            Ok(value) => ready(Ok(QsQuery(value))),
            Err(e) => {
                tracing::error!("Query string parse error: {}", e);
                ready(Err(actix_web::error::ErrorBadRequest(format!(
                    "Query parse error: {}",
                    e
                ))))
            }
        }
    }
}
