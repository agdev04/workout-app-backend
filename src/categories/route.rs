use actix_web::web;
use crate::categories::handler::*;

pub fn category_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/categories")
            .route("", web::post().to(create_category))
            .route("", web::get().to(get_categories))
            .route("/{id}", web::get().to(get_category))
            .route("/{id}", web::patch().to(update_category))
            .route("/{id}", web::delete().to(delete_category))
    );
}