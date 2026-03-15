use std::future::{Ready, ready};
use std::rc::Rc;

use actix_web::{
    Error, HttpMessage,
    dev::{Service, ServiceRequest, ServiceResponse, Transform, forward_ready},
};
use futures_util::future::LocalBoxFuture;

use crate::app::RequestError;
use crate::app::domains::auth::models::{Claims, UserRole};

pub struct RoleGuardFactory {
    allowed_roles: Rc<Vec<UserRole>>,
}

impl RoleGuardFactory {
    pub fn new(roles: Vec<UserRole>) -> Self {
        Self {
            allowed_roles: Rc::new(roles),
        }
    }

    pub fn admin_only() -> Self {
        Self::new(vec![UserRole::Admin])
    }

    pub fn all_employees() -> Self {
        Self::new(vec![UserRole::Employee, UserRole::Admin])
    }

    pub fn user_only() -> Self {
        Self::new(vec![UserRole::User])
    }
}

impl<S, B> Transform<S, ServiceRequest> for RoleGuardFactory
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = RoleGuard<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(RoleGuard {
            service,
            allowed_roles: self.allowed_roles.clone(),
        }))
    }
}

pub struct RoleGuard<S> {
    service: S,
    allowed_roles: Rc<Vec<UserRole>>,
}

impl<S, B> Service<ServiceRequest> for RoleGuard<S>
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
        let claims = req.extensions().get::<Claims>().cloned();

        match claims {
            Some(claims) => {
                if claims.has_any_role(self.allowed_roles.as_ref()) {
                    let fut = self.service.call(req);
                    Box::pin(async move {
                        let res = fut.await?;
                        Ok(res)
                    })
                } else {
                    tracing::warn!(
                        "Access denied for user {} with role '{:?}'. Allowed roles: {:?}",
                        claims.sub,
                        claims.role,
                        self.allowed_roles
                    );
                    let err = RequestError::Forbidden(format!(
                        "Access denied. Required roles: {:?}",
                        self.allowed_roles.as_ref()
                    ));
                    Box::pin(async move { Err(Error::from(err)) })
                }
            }
            None => {
                tracing::error!(
                    "RoleGuard: No claims found in request extensions. Ensure AuthMiddleware runs first."
                );
                let err = RequestError::Unauthorized("Authentication required".to_string());
                Box::pin(async move { Err(Error::from(err)) })
            }
        }
    }
}
