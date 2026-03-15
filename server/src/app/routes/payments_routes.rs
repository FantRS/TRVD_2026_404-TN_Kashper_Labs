use actix_web::web;

use crate::app::domains::payments::controller;
use crate::app::middlewares::auth_middleware::AuthMiddlewareFactory;
use crate::app::middlewares::role_middleware::RoleGuardFactory;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/orders")
            .wrap(RoleGuardFactory::user_only())
            .wrap(AuthMiddlewareFactory)
            .route("/{id}/payments", web::post().to(controller::create_payment)),
    );
}
