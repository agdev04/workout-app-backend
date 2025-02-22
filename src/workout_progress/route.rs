use actix_web::web;
use super::handler;

pub fn workout_progress_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/workout-progress")
            .route("", web::post().to(handler::record_progress))
            .route("/{id}", web::patch().to(handler::update_progress))
            .route("/user/{user_id}", web::get().to(handler::get_user_workout_progress))
            .route("/workout/{workout_id}", web::get().to(handler::get_workout_progress))
    );
}