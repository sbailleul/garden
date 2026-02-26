use actix_web::web;

use crate::api::handlers::{get_companions, list_vegetables, post_plan};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(list_vegetables)
            .service(get_companions)
            .service(post_plan),
    );
}
