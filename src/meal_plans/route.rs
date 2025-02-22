use actix_web::web;

use super::handler::{
    add_meal_to_plan, create_meal_plan, delete_meal_plan, delete_meal_from_plan, get_meal_plan, get_meal_plans, update_meal_plan
};

pub fn meal_plans_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/meal-plans")
            .route("", web::get().to(get_meal_plans))
            .route("", web::post().to(create_meal_plan))
            .route("/{id}", web::get().to(get_meal_plan))
            .route("/{id}", web::patch().to(update_meal_plan))
            .route("/{id}", web::delete().to(delete_meal_plan))
            .route("/{id}/meals", web::post().to(add_meal_to_plan))
            .route("/{meal_plan_id}/meals/{meal_plan_meal_id}", web::delete().to(delete_meal_from_plan))
    );
}