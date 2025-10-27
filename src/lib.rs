use anyhow::{Context, Result};
use lettre::message::header::ContentType;
use lettre::message::{Attachment, MultiPart, SinglePart};
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use std::path::Path;
use std::{env, fs};

const APP_SHORT_NAME: &str = "c_c";
const GMAIL_USERNAME: &str = "GMAIL_USER";
const GMAIL_APPLICATION_PASSWORD: &str = "GMAIL_APPLICATION_PASSWORD";
const EMAIL_RECIPIENT: &str = "EMAIL_RECIPIENT";

const EMAIL_MESSAGE: &str = r#"
The requested book is attached to this email.

Delivered by c_c | crab_courier
"#;

pub fn run(env_variables: Secrets, path_to_ebook: &str) -> Result<()> {
    let email = build_email(&env_variables, path_to_ebook).context("Failed to build the email")?;
    let mailer = get_gmail_mailer(&env_variables);

    send_email(mailer, email).context("Failed to send the email")?;

    Ok(())
}

fn build_email(env_variables: &Secrets, path_to_ebook: &str) -> Result<Message> {
    let path_to_ebook = Path::new(path_to_ebook);
    let filename = path_to_ebook
        .file_name()
        .context("Provided path has no valid file name")?
        .to_str()
        .context("The filename contains invalid UTF-8 characters")?
        .to_string();

    Message::builder()
        .from(
            env_variables
                .username
                .parse()
                .context("Failed to parse username.")?,
        )
        .to(env_variables
            .email_recipient
            .parse()
            .context("Failed to parse email recipient")?)
        .subject(format!("{APP_SHORT_NAME} | {filename}"))
        .multipart(
            MultiPart::mixed()
                .singlepart(get_text_part())
                .singlepart(get_attachment(path_to_ebook, filename)),
        )
        .context("Failed to build email message")
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

fn get_gmail_mailer(env_variables: &Secrets) -> SmtpTransport {
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

pub struct Secrets {
    username: String,
    app_password: String,
    email_recipient: String,
}

impl Secrets {
    pub fn new() -> Self {
        Secrets {
            username: env::var(GMAIL_USERNAME)
                .expect("GMAIL_USERNAME must be set in the environment."),
            app_password: env::var(GMAIL_APPLICATION_PASSWORD)
                .expect("GMAIL_APPLICATION_PASSWORD must be set in the environment."),
            email_recipient: env::var(EMAIL_RECIPIENT)
                .expect("EMAIL_RECIPIENT must be set in the environment."),
        }
    }
}

impl Default for Secrets {
    fn default() -> Self {
        Secrets::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use std::path::PathBuf;
    use tempfile::{TempDir, tempdir};

    #[test]
    fn test_get_text_part_content() {
        let text_part = get_text_part();
        let body_str = String::from_utf8(text_part.raw_body().to_vec()).unwrap();

        assert!(body_str.contains("The requested book is attached to this email."));
        assert!(body_str.contains("Delivered by c_c | crab_courier"));
    }

    #[test]
    fn test_get_attachment() {
        let fake_ebook_name = "fake_ebook.epub";
        let file_content = "This is a pretty useless book.";
        // Need to get temp_dir so it, and created file, exists in the scope until end of the test
        let (_temp_dir, file_path) = create_temp_file(fake_ebook_name, file_content);

        let attachment = get_attachment(&file_path, fake_ebook_name.to_string().clone());
        let attachment_body = String::from_utf8(attachment.raw_body().to_vec()).unwrap();

        assert_eq!(attachment_body.trim(), file_content);
    }

    fn create_temp_file(filename: &str, content: &str) -> (TempDir, PathBuf) {
        let dir = tempdir().expect("Failed to create temporary directory");
        let file_path = dir.path().join(filename);
        let mut file = File::create(&file_path).expect("Failed to create temporary file");
        writeln!(file, "{}", content).expect("Failed to write to temporary file");
        (dir, file_path)
    }
}
