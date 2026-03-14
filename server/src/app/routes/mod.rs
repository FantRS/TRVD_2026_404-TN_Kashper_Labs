mod swagger;

use actix_web::web;

pub fn configure_all_routes(cfg: &mut web::ServiceConfig) {
    swagger::configure(cfg);

    cfg.service(
        web::scope("/api")
            // .configure(auth_routes::configure)
            // .configure(corporate_routes::configure)
            // .configure(groups_routes::configure)
            // .configure(internal_routes::configure)
            // .configure(lesson_balances_routes::configure)
            // .configure(lessons_routes::configure)
            // .configure(orders_routes::configure)
            // .configure(products_routes::configure)
            // .configure(students_routes::configure)
            // .configure(teachers_routes::configure)
            // .configure(webhooks_routes::configure),
    );
}