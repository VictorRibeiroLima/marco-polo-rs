pub mod lettre;

#[derive(Debug)]
pub struct SenderError {
    pub message: String,
}

pub struct SendEmailOptions {
    pub to: String,
    pub subject: String,
    pub body: String,
}

impl SendEmailOptions {
    pub fn new(to: String, subject: String, body: String) -> Self {
        Self { to, subject, body }
    }
}

#[async_trait::async_trait]
pub trait MailSender {
    async fn send(&self, options: SendEmailOptions) -> Result<(), SenderError>;
}
