use actix_web::web;

use crate::app::domains::orders::controller;
use crate::app::middlewares::auth_middleware::AuthMiddlewareFactory;
use crate::app::middlewares::role_middleware::RoleGuardFactory;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/orders")
            .service(
                web::scope("")
                    .wrap(RoleGuardFactory::user_only())
                    .wrap(AuthMiddlewareFactory)
                    .route("", web::get().to(controller::get_orders))
                    .route("/cart", web::get().to(controller::get_cart))
                    .route(
                        "/cart/services",
                        web::post().to(controller::add_service_to_cart),
                    )
                    .route(
                        "/cart/services/{id}",
                        web::delete().to(controller::remove_service_from_cart),
                    )
                    .route(
                        "/cart/products",
                        web::post().to(controller::add_product_to_cart),
                    )
                    .route(
                        "/cart/products/{id}",
                        web::delete().to(controller::remove_product_from_cart),
                    )
                    .route("/checkout", web::post().to(controller::checkout))
                    .route("/{id}", web::get().to(controller::get_order)),
            )
            .service(
                web::scope("")
                    .wrap(RoleGuardFactory::all_employees())
                    .wrap(AuthMiddlewareFactory)
                    .route(
                        "/{id}/status",
                        web::patch().to(controller::change_order_status),
                    ),
            ),
    );
}
