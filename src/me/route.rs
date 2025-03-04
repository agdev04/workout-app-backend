use actix_web::web;

use super::handler::{auto_reset_password, update_me};

pub fn me_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/me")
            .route("/auto-reset-password", web::post().to(auto_reset_password))
            .route("/update", web::patch().to(update_me)),
    );
}
