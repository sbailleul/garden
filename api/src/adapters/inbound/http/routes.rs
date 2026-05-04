use actix_web::web;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::adapters::inbound::http::handlers::{
    get_companions, get_group, get_varieties_by_vegetable, get_variety, get_vegetable, list_groups,
    list_varieties, list_vegetables, list_vegetables_by_group, post_plan,
};
use crate::adapters::inbound::http::openapi::ApiDoc;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(list_groups)
            .service(get_group)
            .service(list_vegetables_by_group)
            .service(list_varieties)
            .service(get_variety)
            .service(get_companions)
            .service(list_vegetables)
            .service(get_vegetable)
            .service(get_varieties_by_vegetable)
            .service(post_plan),
    )
    .service(SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", ApiDoc::openapi()));
}
