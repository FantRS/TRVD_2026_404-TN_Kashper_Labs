use actix_web::web;

use crate::app::domains::schedule::controller;
use crate::app::middlewares::auth_middleware::AuthMiddlewareFactory;
use crate::app::middlewares::role_middleware::RoleGuardFactory;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/schedule")
            .route("/slots", web::get().to(controller::get_available_slots))
            .service(
                web::scope("")
                    .wrap(RoleGuardFactory::all_employees())
                    .wrap(AuthMiddlewareFactory)
                    .route(
                        "/appointments/{id}/assign",
                        web::patch().to(controller::assign_employee),
                    )
                    .route(
                        "/employees/{id}/plan",
                        web::get().to(controller::get_employee_day_plan),
                    ),
            ),
    )
    .service(
        web::scope("/orders")
            .wrap(RoleGuardFactory::user_only())
            .wrap(AuthMiddlewareFactory)
            .route(
                "/{id}/appointment",
                web::post().to(controller::create_appointment),
            ),
    );
}
