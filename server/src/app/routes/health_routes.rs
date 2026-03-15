use actix_web::{HttpResponse, web};
use serde_json::json;

use crate::app::{AppData, RequestResult};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.route("/health", web::get().to(health))
        .route("/ready", web::get().to(ready));
}

#[tracing::instrument(name = "health", skip_all, fields(request_id = %uuid::Uuid::new_v4()))]
async fn health() -> HttpResponse {
    HttpResponse::Ok().json(json!({ "status": "ok" }))
}

#[tracing::instrument(name = "ready", skip_all, fields(request_id = %uuid::Uuid::new_v4()))]
async fn ready(app_data: web::Data<AppData>) -> RequestResult<HttpResponse> {
    sqlx::query_scalar::<_, i32>("SELECT 1")
        .fetch_one(&app_data.db_pool)
        .await?;
    let _ = app_data.redis.get("health:probe").await?;

    Ok(HttpResponse::Ok().json(json!({ "status": "ready" })))
}
