use actix_web::{web, HttpResponse, Result};
use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use serde::Deserialize;
use std::env;

#[derive(Deserialize)]
pub struct TestEmailRequest {
    to: String,
}

pub async fn test_smtp(req: web::Json<TestEmailRequest>) -> Result<HttpResponse> {
    let smtp_host = env::var("SMTP_HOST").expect("SMTP_HOST not set");
    let smtp_port = env::var("SMTP_PORT")
        .expect("SMTP_PORT not set")
        .parse::<u16>()
        .unwrap();
    let smtp_username = env::var("SMTP_USERNAME").expect("SMTP_USERNAME not set");
    let smtp_password = env::var("SMTP_PASSWORD").expect("SMTP_PASSWORD not set");
    let smtp_sender = env::var("SMTP_SENDER").expect("SMTP_SENDER not set");

    let email = Message::builder()
        .from(smtp_sender.parse().unwrap())
        .to(req.to.parse().unwrap())
        .subject("Test Email from Workout App")
        .header(ContentType::TEXT_PLAIN)
        .body(String::from(
            "This is a test email from the Workout App API.",
        ))
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
            "message": "Test email sent successfully"
        }))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(serde_json::json!({
            "status": "error",
            "message": format!("Failed to send test email: {}", e)
        }))),
    }
}
