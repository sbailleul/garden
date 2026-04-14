use actix_web::web;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::adapters::inbound::http::handlers::{
    get_companions, get_variety, get_vegetable, list_varieties, list_vegetables, post_plan,
};
use crate::adapters::inbound::http::openapi::ApiDoc;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(list_vegetables)
            .service(get_vegetable)
            .service(get_companions)
            .service(list_varieties)
            .service(get_variety)
            .service(post_plan),
    )
    .service(SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", ApiDoc::openapi()));
}
