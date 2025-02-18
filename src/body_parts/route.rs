use actix_web::web;
use crate::body_parts::handler::*;

pub fn body_part_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/body-parts")
            .route("", web::post().to(create_body_part))
            .route("", web::get().to(get_body_parts))
            .route("/{id}", web::get().to(get_body_part))
            .route("/{id}", web::patch().to(update_body_part))
            .route("/{id}", web::delete().to(delete_body_part))
    );
}