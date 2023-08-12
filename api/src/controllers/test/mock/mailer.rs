use crate::mail::sender::{MailSender, SenderError};

pub struct MailSenderMock;

#[async_trait::async_trait]
impl MailSender for MailSenderMock {
    async fn send(
        &self,
        _option: crate::mail::sender::SendEmailOptions,
    ) -> Result<(), SenderError> {
        Ok(())
    }
}
