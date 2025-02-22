use crate::favorite_meals::handler::{add_favorite, remove_favorite, get_user_favorites};
use actix_web::web;

pub fn favorite_meals_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/favorite-meals")
            .route("", web::post().to(add_favorite))
            .route("/{id}", web::delete().to(remove_favorite))
            .route("/user/{user_id}", web::get().to(get_user_favorites))
    );
}