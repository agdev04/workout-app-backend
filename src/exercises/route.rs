use actix_web::web;
use crate::exercises::handler::*;

pub fn exercises_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/exercises")
            // Exercise CRUD routes
            .route("", web::post().to(create_exercise))
            .route("", web::get().to(get_exercise_list))
            .route("/{id}", web::get().to(get_exercise))
            .route("/{id}", web::patch().to(update_exercise))
            .route("/{id}", web::delete().to(delete_exercise))
            // Exercise Body Parts routes
            .route("/body-parts", web::post().to(add_exercise_body_part))
            .route("/{exercise_id}/body-parts/{body_part_id}", web::delete().to(remove_exercise_body_part))
            // Exercise Categories routes
            .route("/{id}/categories", web::post().to(add_exercise_category))
            .route("/{exercise_id}/categories/{category_id}", web::delete().to(remove_exercise_category))
            // Exercise Equipment routes
            .route("/{id}/equipment", web::post().to(add_exercise_equipment))
            .route("/{exercise_id}/equipment/{equipment_id}", web::delete().to(remove_exercise_equipment))
    );
}