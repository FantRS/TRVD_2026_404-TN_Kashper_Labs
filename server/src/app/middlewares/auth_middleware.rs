use std::future::{ready, Ready};
use std::rc::Rc;

use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    http::header,
    Error, HttpMessage,
};
use futures_util::future::LocalBoxFuture;

use crate::app::redis::token_wl_service;
use crate::app::utils::jwt::decode_jwt;
use crate::app::RequestError;

pub struct AuthMiddlewareFactory;

impl<S, B> Transform<S, ServiceRequest> for AuthMiddlewareFactory
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = AuthMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthMiddleware {
            service: Rc::new(service),
        }))
    }
}

pub struct AuthMiddleware<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for AuthMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let auth_header = req.headers().get(header::AUTHORIZATION);

        if auth_header.is_none() {
            let err = RequestError::Unauthorized("Missing Authorization header".to_string());
            return Box::pin(async move { Err(Error::from(err)) });
        }

        let auth_str = match auth_header.unwrap().to_str() {
            Ok(s) => s,
            Err(_) => {
                let err = RequestError::Unauthorized("Invalid Authorization header".to_string());
                return Box::pin(async move { Err(Error::from(err)) });
            }
        };

        if !auth_str.starts_with("Bearer ") {
            let err = RequestError::Unauthorized("Invalid Authorization scheme".to_string());
            return Box::pin(async move { Err(Error::from(err)) });
        }

        let token = &auth_str[7..];

        let config = req.app_data::<actix_web::web::Data<crate::core::app_data::AppData>>();

        let (secret, redis_pool) = if let Some(app_data) = config {
            (&app_data.jwt_secret, app_data.redis.clone())
        } else {
            let err = RequestError::InternalServerError("AuthConfig missing".to_string());
            return Box::pin(async move { Err(Error::from(err)) });
        };

        let claims = match decode_jwt(token, secret) {
            Ok(claims) => claims,
            Err(e) => {
                return Box::pin(async move { Err(Error::from(e)) });
            }
        };

        let service = self.service.clone();
        Box::pin(async move {
            let allowed =
                match token_wl_service::verify_in_whitelist(&claims, &redis_pool).await {
                    Ok(v) => v,
                    Err(e) => {
                        tracing::error!(
                            "Whitelist check failed: {}. DENYING request (fail-closed)",
                            e
                        );
                        false
                    }
                };

            if !allowed {
                tracing::warn!("Token {} not in whitelist", claims.jti);
                return Err(Error::from(RequestError::Unauthorized(
                    "Token revoked".to_string(),
                )));
            }

            req.extensions_mut().insert(claims);
            let res = service.call(req).await?;
            Ok(res)
        })
    }
}
