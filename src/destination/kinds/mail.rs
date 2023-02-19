use std::error::Error;
use lettre::message::{Mailbox, SinglePart};
use lettre::{SmtpTransport, Transport};
use lettre::transport::smtp::authentication;
use serde::{Serialize, Deserialize};
use crate::destination::{MessageDestination, SerializableDestination};
use crate::message::{Message, MessageDetail};
use crate::util::html::HtmlMessageDetail;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct MailDestination {
    from: Mailbox,
    relay: Relay,
    to: Mailbox,
    reply_to: Option<Mailbox>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Relay {
    url: String,
    #[serde(default = "default_port")]
    port: u16,
    #[serde(default = "bool::default")]
    start_tls_relay: bool,
    username: String,
    password: String,
}

fn default_port() -> u16 {
    return lettre::transport::smtp::SMTP_PORT;
}

impl MessageDestination for MailDestination {
    fn send(&self, message: &Message) -> Result<(), Box<dyn Error>> {
        println!("Message destination.");
        let mut message_builder = lettre::Message::builder()
            .from(self.from.clone())
            .to(self.to.clone());

        if let Some(reply_to) = self.reply_to.clone() {
            message_builder = message_builder.reply_to(reply_to);
        }
        if let Some(title) = message.get_title() {
            message_builder = message_builder.subject(title);
        }

        let email = message_builder.singlepart(create_body(message.get_message_detail()))?;

        let creds = authentication::Credentials::new(self.relay.username.clone(), self.relay.password.clone());
        let mailer =
            if self.relay.start_tls_relay { SmtpTransport::starttls_relay(&self.relay.url) } else { SmtpTransport::relay(&self.relay.url) }?.port(self.relay.port)
                .credentials(creds)
                .build();

        mailer.send(&email)?;
        println!("Mail successfully sent.");
        Ok(())
    }
}

#[typetag::serde(name = "Mail")]
impl SerializableDestination for MailDestination {
    fn as_message_destination(&self) -> &dyn MessageDestination {
        self
    }
}

fn create_body(detail: &MessageDetail) -> SinglePart {
    match detail {
        MessageDetail::Raw(raw) => {
            SinglePart::plain(raw.to_owned())
        }
        MessageDetail::Formatted(formatted) => {
            SinglePart::html(formatted.create_html())
        }
    }
}