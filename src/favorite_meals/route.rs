use crate::favorite_meals::handler::{add_favorite, remove_favorite};
use actix_web::web;

pub fn favorite_meals_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/favorite-meals")
            .service(add_favorite)
            .service(remove_favorite)
    );
}