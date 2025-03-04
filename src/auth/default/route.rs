use crate::auth::smtp::test_smtp;
use crate::auth::password_reset::{forgot_password, reset_password_with_token};
use actix_web::web;

use super::handler::login;

pub fn auth_default_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .route("/login", web::post().to(login))
            .route("/smtp-test", web::post().to(test_smtp))
            .route("/forgot-password", web::post().to(forgot_password))
            .route("/reset-password", web::post().to(reset_password_with_token)),
    );
}
