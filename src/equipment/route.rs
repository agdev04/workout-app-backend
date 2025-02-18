use actix_web::web;
use crate::equipment::handler::*;

pub fn equipment_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/equipment")
            .route("", web::post().to(create_equipment))
            .route("", web::get().to(get_equipment_list))
            .route("/{id}", web::get().to(get_equipment))
            .route("/{id}", web::patch().to(update_equipment))
            .route("/{id}", web::delete().to(delete_equipment))
    );
}