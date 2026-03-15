use actix_web::web;

use crate::app::domains::catalog::controller;
use crate::app::middlewares::auth_middleware::AuthMiddlewareFactory;
use crate::app::middlewares::role_middleware::RoleGuardFactory;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/catalog")
            .route("/services", web::get().to(controller::get_services))
            .route("/services/{id}", web::get().to(controller::get_service))
            .route("/products", web::get().to(controller::get_products))
            .route("/products/{id}", web::get().to(controller::get_product))
            .route(
                "/categories/services",
                web::get().to(controller::get_service_categories),
            )
            .route(
                "/categories/products",
                web::get().to(controller::get_product_categories),
            )
            .service(
                web::scope("")
                    .wrap(RoleGuardFactory::admin_only())
                    .wrap(AuthMiddlewareFactory)
                    .route("/services", web::post().to(controller::create_service))
                    .route(
                        "/services/{id}",
                        web::patch().to(controller::update_service),
                    )
                    .route("/products", web::post().to(controller::create_product))
                    .route(
                        "/products/{id}",
                        web::patch().to(controller::update_product),
                    )
                    .route(
                        "/categories/services",
                        web::post().to(controller::create_service_category),
                    )
                    .route(
                        "/categories/services/{id}",
                        web::patch().to(controller::update_service_category),
                    )
                    .route(
                        "/categories/products",
                        web::post().to(controller::create_product_category),
                    )
                    .route(
                        "/categories/products/{id}",
                        web::patch().to(controller::update_product_category),
                    ),
            ),
    );
}
