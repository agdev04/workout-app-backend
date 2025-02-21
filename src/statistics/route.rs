use actix_web::web;
use crate::statistics::handler::*;

pub fn statistics_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/statistics")
            .route("", web::get().to(get_statistics))
    );
}