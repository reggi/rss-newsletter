use lettre::address::AddressError;
use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::transport::smtp::Error as SmtpError;
use lettre::{message::Mailbox, Message, SmtpTransport, Transport};
use std::fmt;

#[derive(Debug)]
pub enum EmailSendError {
    Smtp(SmtpError),
    Address(AddressError),
}

impl fmt::Display for EmailSendError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            EmailSendError::Smtp(ref err) => write!(f, "SMTP error: {}", err),
            EmailSendError::Address(ref err) => write!(f, "Address error: {}", err),
        }
    }
}

impl From<SmtpError> for EmailSendError {
    fn from(err: SmtpError) -> Self {
        EmailSendError::Smtp(err)
    }
}

impl From<AddressError> for EmailSendError {
    fn from(err: AddressError) -> Self {
        EmailSendError::Address(err)
    }
}

impl std::error::Error for EmailSendError {}
pub struct EmailConfig {
    pub body: String,
    pub smtp_password: String,
    pub smtp_host: String,
    pub smtp_email: String,
    pub smtp_port: u16,
    pub subject: String,
    pub to: String,
}

impl fmt::Display for EmailConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "EmailConfig {{ smtp_email: {}, smtp_host: {}, smtp_port: {}, to: {}, subject: {}, body: {} }}",
            self.smtp_email, self.smtp_host, self.smtp_port, self.to, self.subject, self.body
        )
    }
}

pub fn send_email(config: EmailConfig) -> Result<(), EmailSendError> {
    let email = Message::builder()
        .from(
            config
                .smtp_email
                .parse::<Mailbox>()
                .map_err(EmailSendError::from)?,
        )
        .reply_to(
            config
                .smtp_email
                .parse::<Mailbox>()
                .map_err(EmailSendError::from)?,
        )
        .to(config.to.parse::<Mailbox>().map_err(EmailSendError::from)?)
        .subject(config.subject)
        .header(ContentType::TEXT_HTML)
        .body(config.body)
        .unwrap();

    let creds = Credentials::new(config.smtp_email, config.smtp_password);

    // Open a remote connection to gmail
    let mailer = SmtpTransport::relay(&config.smtp_host)
        .unwrap()
        .credentials(creds)
        .port(config.smtp_port)
        .build();

    // Send the email
    match mailer.send(&email) {
        Ok(_) => {
            println!("Email sent successfully!");
            Ok(())
        }
        Err(e) => {
            println!("Could not send email: {:?}", e);
            Err(EmailSendError::Smtp(e))
        }
    }
}
