use actix_web::web;

use super::handler::{add_favorite_workout, get_user_favorite_workouts, remove_favorite_workout};

pub fn favorite_workouts_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/favorite-workouts")
            .route("", web::post().to(add_favorite_workout))
            .route("/{id}", web::delete().to(remove_favorite_workout))
            .route("/user/{user_id}", web::get().to(get_user_favorite_workouts))
    );
}