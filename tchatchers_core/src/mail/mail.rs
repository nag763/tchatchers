use std::{collections::HashMap, sync::OnceLock};

use lettre::{
    message::{header::ContentType, MultiPart, SinglePart},
    transport::smtp::authentication::Credentials,
    AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor,
};
use serde::Deserialize;

use super::template::MailHtmlContent;

static SMTP_RELAY: OnceLock<AsyncSmtpTransport<Tokio1Executor>> = OnceLock::new();
static CONFIGURED_MAILS: OnceLock<ConfiguredMailMap> = OnceLock::new();

type ConfiguredMailMap = HashMap<PossibleConfiguredMail, ConfiguredMail>;

#[derive(Debug, Deserialize, PartialEq, Eq, Hash, Clone, Copy)]
pub enum PossibleConfiguredMail {
    WelcomeMail,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ConfiguredMail {
    from: String,
    subject: String,
    body_plain: String,
    associated_mail: PossibleConfiguredMail,
}

#[derive(Debug)]
pub struct Mail<MailHtmlContent> {
    pub to: String,
    pub configured_mail: ConfiguredMail,
    pub content_html: MailHtmlContent,
}

impl<T> From<Mail<T>> for lettre::Message
where
    T: MailHtmlContent,
{
    fn from(value: Mail<T>) -> Self {
        Message::builder()
            .from(
                std::env::var(value.configured_mail.from)
                    .expect("Couldn't find sender")
                    .parse()
                    .unwrap(),
            )
            .to(value.to.parse().unwrap())
            .subject(value.configured_mail.subject)
            .multipart(
                MultiPart::alternative()
                    .singlepart(
                        SinglePart::builder()
                            .header(ContentType::TEXT_PLAIN)
                            .body(value.configured_mail.body_plain),
                    )
                    .singlepart(
                        SinglePart::builder()
                            .header(ContentType::TEXT_HTML)
                            .body(value.content_html.to_html().unwrap()),
                    ),
            )
            .unwrap()
    }
}

impl<T: MailHtmlContent> Mail<T> {
    pub fn init() {
        let _ = CONFIGURED_MAILS.get_or_init(Self::init_possible_mails);
        let _ = SMTP_RELAY.get_or_init(Self::init_relay);
    }

    fn init_relay() -> AsyncSmtpTransport<Tokio1Executor> {
        let creds = Credentials::new(
            std::env::var("MAIL_USERNAME").expect("No mail username found"),
            std::env::var("MAIL_PASSWORD").expect("No mail password found"),
        );
        let relay_endpoint = std::env::var("MAIL_RELAY").expect("No mail relay defined");
        AsyncSmtpTransport::<Tokio1Executor>::relay(&relay_endpoint)
            .unwrap()
            .credentials(creds)
            .build()
    }

    fn init_possible_mails() -> ConfiguredMailMap {
        let configured_mails: Vec<ConfiguredMail> =
            serde_yaml::from_str(include_str!("../config/mail.yml")).unwrap();
        let mut configred_mails_map: ConfiguredMailMap = HashMap::new();
        for configured_mail in configured_mails {
            configred_mails_map.insert(configured_mail.associated_mail, configured_mail);
        }
        configred_mails_map
    }

    pub fn new(to: String, mail: T) -> Option<Self> {
        let mail_map = CONFIGURED_MAILS.get_or_init(Self::init_possible_mails);
        let Some(configured_mail) = mail_map.get(&mail.configured_mail()).cloned() else {
            return None;
        };
        Some(Self {
            to,
            configured_mail,
            content_html: mail,
        })
    }

    pub async fn send(
        self,
    ) -> Result<lettre::transport::smtp::response::Response, lettre::transport::smtp::Error> {
        let relay = SMTP_RELAY.get_or_init(Self::init_relay);
        let message: lettre::Message = Message::from(self);
        relay.send(message).await
    }

    pub fn send_queued(&self) {
        todo!()
    }
}
