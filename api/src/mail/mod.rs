use std::fmt::Display;

use self::{engine::MailEngine, sender::MailSender};

pub mod engine;
pub mod sender;

#[derive(Debug)]
pub struct MailError {
    pub message: String,
}

impl Display for MailError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#?}", self.message)
    }
}

impl From<engine::EngineError> for MailError {
    fn from(error: engine::EngineError) -> Self {
        Self {
            message: error.message,
        }
    }
}

impl From<sender::SenderError> for MailError {
    fn from(error: sender::SenderError) -> Self {
        Self {
            message: error.message,
        }
    }
}

pub struct Mailer<E: MailEngine, S: MailSender> {
    pub engine: E,
    pub sender: S,
}

impl<E: MailEngine, S: MailSender> Mailer<E, S> {
    pub fn new(engine: E, sender: S) -> Self {
        Self { engine, sender }
    }

    pub async fn send(
        &self,
        to: String,
        subject: String,
        template_name: impl Into<String>,
        params: Option<impl serde::Serialize>,
    ) -> Result<(), MailError> {
        let body = self.engine.render(template_name, params)?;
        let options = sender::SendEmailOptions::new(to, subject, body);

        self.sender.send(options).await?;

        Ok(())
    }
}

impl Default
    for Mailer<engine::handlebars::HandleBarsEngine<'static>, sender::lettre::LettreMailer>
{
    /// Create a new instance of the default mailer.
    /// This will use the Handlebars engine and the Lettre sender.
    fn default() -> Self {
        let engine = engine::handlebars::HandleBarsEngine::new("./api/templates");
        let sender = sender::lettre::LettreMailer::new();

        Self::new(engine, sender)
    }
}
