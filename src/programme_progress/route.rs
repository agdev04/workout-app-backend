use actix_web::web;
use crate::programme_progress::handler::*;

pub fn programme_progress_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/programme-progress")
            .route("", web::post().to(create_progress))
            .route("/user/{user_id}/{programme_id}", web::get().to(get_user_programme_by_user_progress))
            .route("/user/{user_id}", web::get().to(get_user_programme_progress))
            .route("/{id}", web::patch().to(update_progress))
            .route("/{id}", web::delete().to(delete_progress))
    );
}