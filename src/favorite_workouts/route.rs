use actix_web::web;

use super::handler::{add_favorite_workout, get_user_favorite_workouts, remove_favorite_workout};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/favorite-workouts")
            .service(add_favorite_workout)
            .service(remove_favorite_workout)
            .service(get_user_favorite_workouts),
    );
}