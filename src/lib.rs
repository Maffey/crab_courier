use lettre::message::header::ContentType;
use lettre::message::{Attachment, MultiPart, SinglePart};
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use std::path::Path;
use std::{env, fs};
use anyhow::{Context, Result};

const APP_SHORT_NAME: &str = "c_c";
const GMAIL_USERNAME: &str = "GMAIL_USER";
const GMAIL_APPLICATION_PASSWORD: &str = "GMAIL_APPLICATION_PASSWORD";
const EMAIL_RECIPIENT: &str = "EMAIL_RECIPIENT";

const EMAIL_MESSAGE: &str = r#"
The requested book is attached to this email.

Delivered by c_c
"#;

pub fn run(env_variables: EnvVariables, path_to_ebook: &str) -> Result<()> {
    let email = build_email(&env_variables, path_to_ebook).context("Failed to build the email")?;
    let mailer = get_gmail_mailer(&env_variables);

    send_email(mailer, email).context("Failed to send the email")?;

    Ok(())
}

fn build_email(env_variables: &EnvVariables, path_to_ebook: &str) -> Result<Message> {
    let path_to_ebook = Path::new(path_to_ebook);
    let filename = path_to_ebook
        .file_name().context("Provided path has no valid file name")?
        .to_str().context("The filename contains invalid UTF-8 characters")?
        .to_string();

    Message::builder()
        .from(env_variables.username
            .parse()
            .context("Failed to parse username.")?)
        .to(env_variables.email_recipient.parse().context("Failed to parse email recipient")?)
        .subject(format!("{APP_SHORT_NAME} | {filename}"))
        .multipart(
            MultiPart::mixed()
                .singlepart(get_text_part())
                .singlepart(get_attachment(path_to_ebook, filename)),
        ).context("Failed to build email message")
}

fn get_attachment(path_to_ebook: &Path, filename: String) -> SinglePart {
    let filebody = fs::read(path_to_ebook).expect("Unable to read file for attachment");

    let guessed_mime = mime_guess::from_path(path_to_ebook)
        .first()
        .unwrap()
        .to_string();
    let content_type = ContentType::parse(&guessed_mime).unwrap();

    Attachment::new(filename).body(filebody, content_type)
}

fn get_text_part() -> SinglePart {
    SinglePart::builder()
        .header(ContentType::TEXT_PLAIN)
        .body(String::from(EMAIL_MESSAGE))
}

fn get_gmail_mailer(env_variables: &EnvVariables) -> SmtpTransport {
    let credentials = Credentials::new(
        env_variables.username.clone(),
        env_variables.app_password.clone(),
    );

    SmtpTransport::relay("smtp.gmail.com")
        .expect("Failed to establish fail connection with email server")
        .credentials(credentials)
        .build()
}

fn send_email(mailer: SmtpTransport, email: Message) -> Result<()> {
    mailer.send(&email).context("Failed to send an email")?;
    Ok(())
}

pub struct EnvVariables {
    username: String,
    app_password: String,
    email_recipient: String,
}

impl EnvVariables {
    pub fn new() -> Self {
        EnvVariables {
            username: env::var(GMAIL_USERNAME)
                .expect("GMAIL_USERNAME must be set in the environment."),
            app_password: env::var(GMAIL_APPLICATION_PASSWORD)
                .expect("GMAIL_APPLICATION_PASSWORD must be set in the environment."),
            email_recipient: env::var(EMAIL_RECIPIENT)
                .expect("EMAIL_RECIPIENT must be set in the environment."),
        }
    }
}

impl Default for EnvVariables {
    fn default() -> Self {
        EnvVariables::new()
    }
}
