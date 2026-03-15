use actix_web::web;

use crate::app::domains::reports::controller;
use crate::app::middlewares::auth_middleware::AuthMiddlewareFactory;
use crate::app::middlewares::role_middleware::RoleGuardFactory;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/reports")
            .wrap(RoleGuardFactory::admin_only())
            .wrap(AuthMiddlewareFactory)
            .route("/orders", web::get().to(controller::get_orders_report))
            .route("/payments", web::get().to(controller::get_payments_report)),
    );
}
