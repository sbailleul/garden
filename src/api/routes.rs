use actix_web::web;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::api::handlers::{get_companions, get_vegetable, list_vegetables, post_plan};
use crate::api::openapi::ApiDoc;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(list_vegetables)
            .service(get_vegetable)
            .service(get_companions)
            .service(post_plan),
    )
    .service(SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", ApiDoc::openapi()));
}
