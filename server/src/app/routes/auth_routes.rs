use actix_web::web;

use crate::app::domains::auth::controller;
use crate::app::middlewares::auth_middleware::AuthMiddlewareFactory;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .route("/register", web::post().to(controller::register))
            .route("/login", web::post().to(controller::login))
            .service(
                web::scope("")
                    .wrap(AuthMiddlewareFactory)
                    .route("/logout", web::post().to(controller::logout))
                    .route("/logout-all", web::post().to(controller::logout_all))
                    .route("/me", web::get().to(controller::me)),
            ),
    );
}
