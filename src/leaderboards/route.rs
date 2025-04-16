use super::handler;
use actix_web::web;

pub fn leaderboards_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/leaderboards")
            .route("", web::get().to(handler::get_leaderboard))
            .route("/user/{user_id}", web::get().to(handler::get_user_ranking)),
    );
}
