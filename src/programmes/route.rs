use actix_web::web;
use crate::programmes::handler::*;

pub fn programmes_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/programmes")
            // Programme CRUD routes
            .route("", web::post().to(create_programme))
            .route("", web::get().to(get_programme_list))
            .route("/{id}", web::get().to(get_programme))
            .route("/{id}", web::patch().to(update_programme))
            .route("/{id}", web::delete().to(delete_programme))
            // Programme Week routes
            .route("/weeks", web::post().to(add_programme_week))
            .route("/{programme_id}/weeks/{week_number}", web::delete().to(delete_programme_week))
            // Programme Day routes
            .route("/days", web::post().to(add_programme_day))
            .route("/{programme_week_id}/days/{day_number}/exercises/{exercise_id}", web::delete().to(delete_programme_day))
    );
}