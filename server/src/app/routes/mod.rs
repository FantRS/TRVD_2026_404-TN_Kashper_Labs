mod auth_routes;
mod catalog_routes;
mod health_routes;
mod orders_routes;
mod payments_routes;
mod reports_routes;
mod schedule_routes;
mod swagger;
mod users_routes;

use actix_web::web;

pub fn configure_all_routes(cfg: &mut web::ServiceConfig) {
    health_routes::configure(cfg);
    swagger::configure(cfg);

    cfg.service(
        web::scope("/api")
            .configure(auth_routes::configure)
            .configure(catalog_routes::configure)
            .configure(orders_routes::configure)
            .configure(schedule_routes::configure)
            .configure(payments_routes::configure)
            .configure(users_routes::configure)
            .configure(reports_routes::configure),
    );
}
