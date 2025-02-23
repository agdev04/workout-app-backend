use actix_web::web;

use super::handler::{reset_password, update_me};

pub fn me_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users/me")
            .route("/reset-password", web::post().to(reset_password))
            .route("/update", web::put().to(update_me))
    );
}