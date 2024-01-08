use askama::Template;

use super::PossibleConfiguredMail;

pub trait MailHtmlContent: askama::Template {
    fn to_html(&self) -> Result<String, askama::Error> {
        self.render()
    }

    fn configured_mail(&self) -> PossibleConfiguredMail;
}

#[derive(Template, Debug, Default)]
#[template(path = "welcome.html", ext = "html", escape = "none")]
pub struct WelcomeMailContent {
    pub name: String,
    pub app_uri: String,
    pub token: String,
    pub mail_support_sender: String,
    pub mail_gdpr_sender: String,
}

impl MailHtmlContent for WelcomeMailContent {
    fn configured_mail(&self) -> PossibleConfiguredMail {
        PossibleConfiguredMail::WelcomeMail
    }
}
