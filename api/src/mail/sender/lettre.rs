use lettre::{
    address::AddressError,
    error::Error,
    message::{header::ContentType, Mailbox},
    transport::smtp::authentication::Credentials,
    Message, Transport,
};

use super::MailSender;

impl From<AddressError> for super::SenderError {
    fn from(error: AddressError) -> Self {
        return Self {
            message: error.to_string(),
        };
    }
}

impl From<Error> for super::SenderError {
    fn from(error: Error) -> Self {
        return Self {
            message: error.to_string(),
        };
    }
}

impl From<lettre::transport::smtp::Error> for super::SenderError {
    fn from(error: lettre::transport::smtp::Error) -> Self {
        return Self {
            message: error.to_string(),
        };
    }
}

pub struct LettreMailer {
    credentials: Credentials,
    host: String,
    from: Mailbox,
}

impl LettreMailer {
    pub fn new() -> Self {
        println!("Creating LettreMailer...");
        let username = std::env::var("SMTP_USERNAME").unwrap();
        let password = std::env::var("SMTP_PASSWORD").unwrap();
        let host = std::env::var("SMTP_HOST").unwrap();
        let from = std::env::var("SMTP_FROM").unwrap();
        let from = from.parse().unwrap();

        Self {
            credentials: Credentials::new(username, password),
            host,
            from,
        }
    }
}

#[async_trait::async_trait]
impl MailSender for LettreMailer {
    async fn send(&self, options: super::SendEmailOptions) -> Result<(), super::SenderError> {
        let to: Mailbox = options.to.parse()?;

        let email = Message::builder()
            .from(self.from.clone())
            .to(to)
            .subject("Happy new year")
            .header(ContentType::TEXT_HTML)
            .body(options.body)?;

        //open connection to smtp server
        let mailer = lettre::SmtpTransport::relay(&self.host)?;
        let mailer = mailer.credentials(self.credentials.clone()).build();

        mailer.send(&email)?;

        return Ok(());
    }
}
