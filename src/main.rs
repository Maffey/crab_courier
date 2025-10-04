use std::env;
use dotenvy::dotenv;
use lettre::{Message, SmtpTransport};

const GMAIL_USERNAME: &str = "GMAIL_USER";
const GMAIL_APPLICATION_PASSWORD: &str = "GMAIL_APPLICATION_PASSWORD";



struct Configuration {
    username: String,
    app_password: String,
}

impl Configuration {

    fn new() -> Self {
        Configuration {
            username: env::var(GMAIL_USERNAME).expect("GMAIL_USERNAME must be set in .env file"),
            app_password: env::var(GMAIL_APPLICATION_PASSWORD).expect("GMAIL_APPLICATION_PASSWORD must be set in .env file")
        }
    }
}

fn main() {
    // TODO configuration here, rest in lib.rs

    dotenv().ok();
    let config = Configuration::new();



}