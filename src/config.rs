use std::env;

#[derive(Debug, Clone)]
pub struct SmtpConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub sender: String,
}

pub fn get_smtp_config() -> SmtpConfig {
    dotenv::dotenv().ok();

    SmtpConfig {
        host: env::var("SMTP_HOST").unwrap_or_else(|_| "smtp.gmail.com".to_string()),
        port: env::var("SMTP_PORT").unwrap_or_else(|_| "465".to_string()).parse().unwrap_or(465),
        username: env::var("SMTP_USERNAME").unwrap_or_else(|_| "workout@vpa.com.au".to_string()),
        password: env::var("SMTP_PASSWORD").expect("SMTP_PASSWORD must be set"),
        sender: env::var("SMTP_SENDER").unwrap_or_else(|_| "workout@vpa.com.au".to_string()),
    }
}

pub fn get_auth_setup() -> String {
    // Load .env file
    dotenv::dotenv().ok();

    // Attempt to get AUTH_SETUP from environment
    let auth_setup = env::var("AUTH_SETUP").unwrap_or_else(|_| {
        println!("AUTH_SETUP not found in environment. Using default value.");
        "default".to_string()
    });

    auth_setup
}
