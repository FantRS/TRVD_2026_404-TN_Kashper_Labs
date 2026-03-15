use actix_web::web;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::app::domains::{
    auth::{controller as auth_controller, models::*},
    catalog::controller as catalog_controller,
    orders::controller as orders_controller,
    payments::controller as payments_controller,
    reports::controller as reports_controller,
    schedule::controller as schedule_controller,
    users::controller as users_controller,
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
    paths(
        auth_controller::register,
        auth_controller::login,
        auth_controller::logout,
        auth_controller::logout_all,
        auth_controller::me,
        catalog_controller::get_services,
        catalog_controller::get_service,
        catalog_controller::create_service,
        catalog_controller::update_service,
        catalog_controller::get_products,
        catalog_controller::get_product,
        catalog_controller::create_product,
        catalog_controller::update_product,
        catalog_controller::get_service_categories,
        catalog_controller::create_service_category,
        catalog_controller::update_service_category,
        catalog_controller::get_product_categories,
        catalog_controller::create_product_category,
        catalog_controller::update_product_category,
        orders_controller::get_cart,
        orders_controller::add_service_to_cart,
        orders_controller::add_product_to_cart,
        orders_controller::checkout,
        orders_controller::get_order,
        orders_controller::change_order_status,
        schedule_controller::get_available_slots,
        schedule_controller::create_appointment,
        schedule_controller::assign_employee,
        schedule_controller::get_employee_day_plan,
        payments_controller::create_payment,
        users_controller::get_users,
        users_controller::update_user_role,
        users_controller::block_user,
        reports_controller::get_orders_report,
        reports_controller::get_payments_report
    ),
    components(
        schemas(
            UserRole
        )
    ),
    tags(
        (name = "Auth", description = "Ендпоінти аутентифікації"),
        (name = "Catalog", description = "Публічний каталог і адмінський CRUD"),
        (name = "Orders", description = "Кошик, checkout і статуси замовлень"),
        (name = "Schedule", description = "Слоти та бронювання"),
        (name = "Payments", description = "Оплата замовлень через внутрішній гаманець"),
        (name = "Users", description = "Адміністрування користувачів"),
        (name = "Reports", description = "Адмінська аналітика"),
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
