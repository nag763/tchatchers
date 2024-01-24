use askama::Template;

use crate::locale::TranslationMap;

use super::PossibleConfiguredMail;

pub trait MailHtmlContent: askama::Template {
    fn to_html(&self) -> Result<String, askama::Error> {
        self.render()
    }

    fn configured_mail(&self) -> PossibleConfiguredMail;

    fn translate_or_default(&self, _label: &str, default: &str) -> String {
        default.to_string()
    }
}

#[derive(Template, Debug, Default)]
#[template(path = "welcome.html", ext = "html", escape = "none")]
pub struct WelcomeMailContent {
    pub name: String,
    pub app_uri: String,
    pub token: String,
    pub mail_support_sender: String,
    pub mail_gdpr_sender: String,
    pub translation_map: TranslationMap,
}

impl MailHtmlContent for WelcomeMailContent {
    fn configured_mail(&self) -> PossibleConfiguredMail {
        PossibleConfiguredMail::WelcomeMail
    }

    fn translate_or_default(&self, label: &str, default: &str) -> String {
        self.translation_map.get_or_default(label, default)
    }
}
