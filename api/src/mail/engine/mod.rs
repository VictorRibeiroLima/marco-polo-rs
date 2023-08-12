#[derive(Debug)]
pub struct EngineError;

pub mod handlebars;

pub trait MailEngine {
    fn render(
        &self,
        template_name: impl Into<String>,
        params: Option<impl serde::Serialize>,
    ) -> Result<String, EngineError>;
}
