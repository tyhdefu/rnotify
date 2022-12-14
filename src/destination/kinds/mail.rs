use std::error::Error;
use lettre::message::Mailbox;
use lettre::{SmtpTransport, Transport};
use lettre::transport::smtp::authentication;
use serde::{Serialize, Deserialize};
use crate::destination::kinds::MessageDestination;
use crate::Message;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct MailDestination {
    from: Mailbox,
    relay: Relay,
    to: Mailbox,
    reply_to: Option<Mailbox>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
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

        let email = message_builder.body(message.get_message_detail().raw().to_owned())?;

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