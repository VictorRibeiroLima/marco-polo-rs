use handlebars::Handlebars;

use super::MailEngine;

impl From<handlebars::TemplateError> for super::EngineError {
    fn from(error: handlebars::TemplateError) -> Self {
        return Self {
            message: error.to_string(),
        };
    }
}

impl From<handlebars::RenderError> for super::EngineError {
    fn from(error: handlebars::RenderError) -> Self {
        return Self {
            message: error.to_string(),
        };
    }
}

pub struct HandleBarsEngine<'a> {
    handlebars: Handlebars<'a>,
}

impl HandleBarsEngine<'_> {
    pub fn new() -> Self {
        println!("Creating HandleBarsEngine...");
        let mut handlebars = Handlebars::new();
        handlebars
            .register_template_file("forgot-password", "./templates/forgot-password.hbs")
            .unwrap();

        return Self { handlebars };
    }
}

impl MailEngine for HandleBarsEngine<'_> {
    fn render(
        &self,
        template_name: impl Into<String>,
        params: Option<impl serde::Serialize>,
    ) -> Result<String, super::EngineError> {
        #[derive(serde::Serialize)]
        struct Empty;

        let template = match params {
            Some(params) => self
                .handlebars
                .render(template_name.into().as_str(), &params)?,
            None => self
                .handlebars
                .render(template_name.into().as_str(), &Empty)?,
        };

        return Ok(template);
    }
}

#[cfg(test)]
mod test {
    use crate::mail::engine::MailEngine;

    #[derive(serde::Serialize)]
    struct ForgotParams {
        name: String,
        url: String,
        token: String,
    }

    #[test]
    fn test_forgot_password_rendering() {
        let engine = super::HandleBarsEngine::new();

        let params = ForgotParams {
            name: "John".into(),
            url: "http://localhost:8080".into(),
            token: "test".into(),
        };

        let template = engine
            .render("forgot-password", Some(params))
            .expect("Failed to render template");

        assert_eq!(
            template,
            "<html>\n\n".to_owned()+
            "<head>\n  "+
            "<title>Forgot Password</title>\n"+
            "</head>\n\n"+
            "<body>\n  "+
            "<h1>Forgot Password</h1>\n  "+
            "<p>Hi John,</p>\n  "+
            "<p>Click <a href=\"http://localhost:8080/reset-password/?token=test\">here</a> to reset your password.</p>\n</body>\n\n</html>"
        );
    }
}
