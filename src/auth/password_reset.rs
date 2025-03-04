use actix_web::{web, HttpResponse, Result};
use chrono::{Duration, Utc};
use diesel::prelude::*;
use jsonwebtoken::{encode, EncodingKey, Header};
use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use serde::{Deserialize, Serialize};
use std::env;

use crate::db::establish_connection;
use crate::schema::users;
use crate::users::model::User;

#[derive(Deserialize)]
pub struct ForgotPasswordRequest {
    email: String,
}

#[derive(Deserialize)]
pub struct ResetPasswordWithTokenRequest {
    token: String,
    new_password: String,
}

#[derive(Serialize, Deserialize)]
struct PasswordResetClaims {
    sub: String,
    email: String,
    exp: usize,
}

pub async fn forgot_password(req: web::Json<ForgotPasswordRequest>) -> Result<HttpResponse> {
    let mut connection = establish_connection();
    
    // Check if user exists
    let user = match users::table
        .filter(users::email.eq(&req.email))
        .first::<User>(&mut connection)
    {
        Ok(user) => user,
        Err(_) => return Ok(HttpResponse::Ok().json(serde_json::json!({
            "status": "success",
            "message": "If the email exists, a password reset link will be sent"
        }))),
    };

    // Generate reset token
    let expiration = Utc::now()
        .checked_add_signed(Duration::days(1))
        .expect("valid timestamp")
        .timestamp();

    let claims = PasswordResetClaims {
        sub: "password_reset".to_string(),
        email: user.email.clone(),
        exp: expiration as usize,
    };

    let key = env::var("AUTH_TOKEN").unwrap_or_else(|_| "secret_token".to_string());
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(key.as_bytes()),
    )
    .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    // Send reset email
    let smtp_host = env::var("SMTP_HOST").expect("SMTP_HOST not set");
    let smtp_port = env::var("SMTP_PORT")
        .expect("SMTP_PORT not set")
        .parse::<u16>()
        .unwrap();
    let smtp_username = env::var("SMTP_USERNAME").expect("SMTP_USERNAME not set");
    let smtp_password = env::var("SMTP_PASSWORD").expect("SMTP_PASSWORD not set");
    let smtp_sender = env::var("SMTP_SENDER").expect("SMTP_SENDER not set");
    let frontend_url = env::var("FRONTEND_URL").unwrap_or_else(|_| "http://localhost:3000".to_string());

    let reset_link = format!("{}/reset-password?token={}", frontend_url, token);
    let email_body = format!(
        "Click the following link to reset your password: {}\n\nThis link will expire in 24 hours.",
        reset_link
    );

    let email = Message::builder()
        .from(smtp_sender.parse().unwrap())
        .to(user.email.parse().unwrap())
        .subject("Password Reset Request")
        .header(ContentType::TEXT_PLAIN)
        .body(email_body)
        .unwrap();

    let creds = Credentials::new(smtp_username, smtp_password);
    let mailer = SmtpTransport::relay(&smtp_host)
        .unwrap()
        .port(smtp_port)
        .credentials(creds)
        .build();

    match mailer.send(&email) {
        Ok(_) => Ok(HttpResponse::Ok().json(serde_json::json!({
            "status": "success",
            "message": "If the email exists, a password reset link will be sent"
        }))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(serde_json::json!({
            "status": "error",
            "message": format!("Failed to send reset email: {}", e)
        })))
    }
}

pub async fn reset_password_with_token(
    req: web::Json<ResetPasswordWithTokenRequest>
) -> Result<HttpResponse> {
    let key = env::var("AUTH_TOKEN").unwrap_or_else(|_| "secret_token".to_string());
    let token_data = match jsonwebtoken::decode::<PasswordResetClaims>(
        &req.token,
        &jsonwebtoken::DecodingKey::from_secret(key.as_bytes()),
        &jsonwebtoken::Validation::default(),
    ) {
        Ok(token) => token,
        Err(_) => return Ok(HttpResponse::BadRequest().json(serde_json::json!({
            "status": "error",
            "message": "Invalid or expired reset token"
        }))),
    };

    let mut connection = establish_connection();
    let hashed_password = super::hash_password(req.new_password.clone())
        .map_err(|_| actix_web::error::ErrorInternalServerError("Failed to hash password"))?;

    let result = diesel::update(users::table)
        .filter(users::email.eq(&token_data.claims.email))
        .set(users::password.eq(hashed_password))
        .execute(&mut connection);

    match result {
        Ok(_) => Ok(HttpResponse::Ok().json(serde_json::json!({
            "status": "success",
            "message": "Password has been reset successfully"
        }))),
        Err(_) => Ok(HttpResponse::InternalServerError().json(serde_json::json!({
            "status": "error",
            "message": "Failed to update password"
        })))
    }
}