use anyhow::{Context, Result};
use lettre::message::header::ContentType;
use lettre::message::{Attachment, MultiPart, SinglePart};
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use std::path::Path;
use std::fs;
use clap::Parser;

const APP_SHORT_NAME: &str = "c_c";

const EMAIL_MESSAGE: &str = r#"
The requested book is attached to this email.

Delivered by c_c | crab_courier
"#;

pub fn run(arguments: &Arguments) -> Result<()> {
    let email = build_email(&arguments).context("Failed to build the email")?;
    let mailer = get_gmail_mailer(&arguments);

    send_email(mailer, email).context("Failed to send the email")?;

    Ok(())
}

fn build_email(arguments: &Arguments) -> Result<Message> {
    let path_to_ebook = Path::new(&arguments.path_to_ebook);
    let filename = path_to_ebook
        .file_name()
        .context("Provided path has no valid file name")?
        .to_str()
        .context("The filename contains invalid UTF-8 characters")?
        .to_string();

    Message::builder()
        .from(
            arguments
                .username
                .parse()
                .context("Failed to parse username.")?,
        )
        .to(arguments
            .recipient
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

fn get_gmail_mailer(arguments: &Arguments) -> SmtpTransport {
    let credentials = Credentials::new(
        arguments.username.clone(),
        arguments.password.clone(),
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use std::path::PathBuf;
    use tempfile::{tempdir, TempDir};

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
        // Need to get temp_dir so it, and created file, exist in the scope until end of the test
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

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Arguments {
    /// The path to ebook file to send to the recipient.
    path_to_ebook: String,

    /// The Gmail username to send email from.
    /// Can also be set using the environment variable.
    #[arg(long, short, env = "GMAIL_USERNAME")]
    username: String,

    /// The Gmail application password.
    /// Can also be set using the environment variable.
    #[arg(long, short, env = "GMAIL_APPLICATION_PASSWORD")]
    password: String,

    /// The email address of the recipient.
    /// Can also be set using the environment variable.
    #[arg(long, short, env = "EMAIL_RECIPIENT")]
    recipient: String,
}

pub fn get_arguments() -> Arguments {
    Arguments::parse()
}