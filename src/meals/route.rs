use actix_web::web;
use crate::meals::handler::*;

pub fn meals_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/meals")
            // Exercise CRUD routes
            .route("", web::post().to(create_meal))
            .route("", web::get().to(get_meal_list))
            .route("/{id}", web::get().to(get_meal))
            .route("/{id}", web::patch().to(update_meal))
            .route("/{id}", web::delete().to(delete_meal))
            // Meal Ingredients
            .route("/ingredients", web::post().to(add_meal_ingredient))
            .route("/{meal_id}/ingredients/{ingredient_id}", web::delete().to(delete_meal_ingredient))
            // Meal Instructions
            .route("/instructions", web::post().to(add_meal_instruction))
            .route("/{meal_id}/instructions/{instruction_id}", web::delete().to(delete_meal_instruction))
    );
}