use actix_web::web;

use crate::app::domains::users::controller;
use crate::app::middlewares::auth_middleware::AuthMiddlewareFactory;
use crate::app::middlewares::role_middleware::RoleGuardFactory;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
            .wrap(RoleGuardFactory::admin_only())
            .wrap(AuthMiddlewareFactory)
            .route("", web::get().to(controller::get_users))
            .route("/{id}/role", web::patch().to(controller::update_user_role))
            .route(
                "/{id}/active-state",
                web::patch().to(controller::block_user),
            ),
    );
}
