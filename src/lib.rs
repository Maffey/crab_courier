use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use std::{env, fs};
use lettre::message::{Attachment, MultiPart, SinglePart};

const GMAIL_USERNAME: &str = "GMAIL_USER";
const GMAIL_APPLICATION_PASSWORD: &str = "GMAIL_APPLICATION_PASSWORD";
const KINDLE_ENDPOINT: &str = "KINDLE_ENDPOINT";

pub fn run(configuration: Configuration) {
    // TODO providiing arguments
    // TODO put into multiple files
    println!("Preparing e-mail...");



    let email = build_email(&configuration);
    let mailer = get_gmail_mailer(&configuration);

    send_email(mailer, email);
}

fn build_email(configuration: &Configuration) -> Message {
    // TODO send actual attachement
    let filename = String::from("the_great_gatsby.epub");
    // TODO change expects and unwraps to ?, return Result from function
    let filebody = fs::read("data/the_great_gatsby.epub").expect("Unable to read file for attachment.");

    // TODO mime guessing based on file extension
    let content_type = ContentType::parse("application/epub+zip").unwrap();
    let attachement = Attachment::new(filename).body(filebody, content_type);
    // TODO refactor into smaller functions

    let text_message = SinglePart::builder()
        .header(ContentType::TEXT_PLAIN)
        .body(String::from("Yay!"));

    Message::builder()
        .from(configuration.username.parse().unwrap())
        .to(configuration.kindle_endpoint.parse().unwrap())
        .subject("Test email") // TODO from file title
        .multipart(MultiPart::mixed().singlepart(text_message).singlepart(attachement))
        .unwrap()
}

fn get_gmail_mailer(configuration: &Configuration) -> SmtpTransport {
    let credentials = Credentials::new(
        configuration.username.clone(),
        configuration.app_password.clone(),
    );

    SmtpTransport::relay("smtp.gmail.com")
        .unwrap()
        .credentials(credentials)
        .build()
}

fn send_email(mailer: SmtpTransport, email: Message) {
    match mailer.send(&email) {
        Ok(_) => println!("Email sent successfully!"),
        Err(e) => panic!("Could not send email: {e:?}"),
    }
}

pub struct Configuration {
    username: String,
    app_password: String,
    kindle_endpoint: String,
}

impl Configuration {
    pub fn new() -> Self {
        Configuration {
            username: env::var(GMAIL_USERNAME)
                .expect("GMAIL_USERNAME must be set in the environment."),
            app_password: env::var(GMAIL_APPLICATION_PASSWORD)
                .expect("GMAIL_APPLICATION_PASSWORD must be set in the environment."),
            kindle_endpoint: env::var(KINDLE_ENDPOINT)
                .expect("KINDLE_ENDPOINT must be set in the environment."),
        }
    }
}

impl Default for Configuration {
    fn default() -> Self {
        Configuration::new()
    }
}
