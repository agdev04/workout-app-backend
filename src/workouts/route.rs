use actix_web::web;
use crate::workouts::handler::*;

pub fn workouts_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/workouts")
            // Workout CRUD routes
            .route("", web::post().to(create_workout))
            .route("", web::get().to(get_workout_list))
            .route("/{id}", web::get().to(get_workout))
            .route("/{id}", web::patch().to(update_workout))
            .route("/{id}", web::delete().to(delete_workout))
            // Workout Exercises
            .route("/exercises", web::post().to(add_workout_exercise))
            .route("/exercises/{id}", web::delete().to(delete_workout_exercise))
    );
}