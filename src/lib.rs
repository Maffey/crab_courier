use lettre::message::header::ContentType;
use lettre::message::{Attachment, MultiPart, SinglePart};
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use std::path::Path;
use std::{env, fs};

const APPLICATION_NAME: &str = "c_c";
const GMAIL_USERNAME: &str = "GMAIL_USER";
const GMAIL_APPLICATION_PASSWORD: &str = "GMAIL_APPLICATION_PASSWORD";
const EMAIL_RECIPIENT: &str = "EMAIL_RECIPIENT";


pub fn run(env_variables: EnvVariables, path_to_ebook: &str) {
    println!("Sending e-mail...");
    let email = build_email(&env_variables, path_to_ebook);
    let mailer = get_gmail_mailer(&env_variables);

    send_email(mailer, email);
}

fn build_email(env_variables: &EnvVariables, path_to_ebook: &str) -> Message {
    let path_to_ebook = Path::new(path_to_ebook);
    // TODO chain of unwraps required cause either might not be a path, or not valid utf-8.
    let filename = path_to_ebook
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();

    Message::builder()
        .from(env_variables.username.parse().unwrap())
        .to(env_variables.email_recipient.parse().unwrap())
        .subject(format!("{APPLICATION_NAME} | {filename}"))
        .multipart(
            MultiPart::mixed()
                .singlepart(get_text_part())
                .singlepart(get_attachment(path_to_ebook, filename)),
        )
        .unwrap()
}

fn get_attachment(path_to_ebook: &Path, filename: String) -> SinglePart {
    // TODO change expects and unwraps to ?, return Result from function
    let filebody = fs::read(path_to_ebook).expect("Unable to read file for attachment.");

    // TODO mime guessing based on file extension
    let content_type = ContentType::parse("application/epub+zip").unwrap();

    Attachment::new(filename).body(filebody, content_type)
}

fn get_text_part() -> SinglePart {
    SinglePart::builder()
        .header(ContentType::TEXT_PLAIN)
        .body(String::from("Yay!"))
}

fn get_gmail_mailer(env_variables: &EnvVariables) -> SmtpTransport {
    let credentials = Credentials::new(
        env_variables.username.clone(),
        env_variables.app_password.clone(),
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