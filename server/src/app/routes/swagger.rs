use actix_web::web;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::app::domains::{
    auth::models::*,
    // corporate::{controller as corporate_controller, models::*},
};

#[derive(OpenApi)]
#[openapi(
    info(
        title = "CrematoryShop API",
        version = "0.0.1",
        description = r#"API для керування CrematoryShop.

Ролі системи:
- user — звичайний користувач.
- employee — звичайний робітник.
- admin — адміністратор.
"#,
    ),
    paths(),
    components(
        schemas(
            UserRole
        )
    ),
    tags(
        (name = "Auth", description = "Ендпоінти аутентифікації"),
    ),
    modifiers(&SecurityAddon)
)]
struct ApiDoc;

struct SecurityAddon;

impl utoipa::Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "bearer_auth",
                utoipa::openapi::security::SecurityScheme::Http(
                    utoipa::openapi::security::HttpBuilder::new()
                        .scheme(utoipa::openapi::security::HttpAuthScheme::Bearer)
                        .bearer_format("JWT")
                        .build(),
                ),
            );
        }
    }
}

pub(super) fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", ApiDoc::openapi()),
    );
}
