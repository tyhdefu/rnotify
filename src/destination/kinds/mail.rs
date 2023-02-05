use std::error::Error;
use lettre::message::{Mailbox, SinglePart};
use lettre::{SmtpTransport, Transport};
use lettre::transport::smtp::authentication;
use serde::{Serialize, Deserialize};
use crate::destination::{MessageDestination, SerializableDestination};
use crate::message::{Message, MessageDetail};
use crate::message::formatted_detail::{FormattedMessageComponent, FormattedMessageDetail, FormattedString, Style};

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
            SinglePart::html(formatted_to_html(formatted))
        }
    }
}

fn formatted_to_html(formatted: &FormattedMessageDetail) -> String {
    let mut html = String::with_capacity(100);
    for component in formatted.components() {
        match component {
            FormattedMessageComponent::Section(section, formatted_string) => {
                html.push_str(&format!("<div><h2>{}</h2><p>{}</p></div>",
                                       escape_html(section),
                                       parse_formatted(formatted_string)
                ));
            }
            FormattedMessageComponent::Text(formatted_string) => {
                html.push_str(&format!("<p>{}</p>", parse_formatted(formatted_string)))
            }
        }
    }
    html
}

fn escape_html(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('>', "&gt;")
        .replace('<', "&lt;")
}

fn parse_formatted(formatted: &Vec<FormattedString>) -> String {
    let mut html = String::new();
    for part in formatted {
        for style in part.get_styles() {
            let start_tag = match style {
                Style::Bold => "<b>",
                Style::Italics => "<i>",
                Style::Monospace => "<code>",
                Style::Code { lang: _ } => "<code>",
            };
            html.push_str(start_tag);
        }
        html.push_str(&escape_html(part.get_string()));
        for style in part.get_styles() {
            let end_tag = match style {
                Style::Bold => "</b>",
                Style::Italics => "</i>",
                Style::Monospace => "</code>",
                Style::Code { lang: _ } => "</code>",
            };
            html.push_str(end_tag);
        }
    }
    html
}

#[cfg(test)]
mod test {
    use crate::message::detail_builder::{FormattedStringAppendable, MessageDetailBuilder};
    use crate::message::MessageDetail::Formatted;
    use super::*;

    #[test]
    fn test_html_conversion() {
        let mut builder = MessageDetailBuilder::new();
        builder.section("hello world", |body| {
            body.append_plain("Dear fellow inhabitants. it has come to my attention that ");
            body.append_styled("you have not been doing your job.", Style::Bold);
        });
        builder.text_block(|block| {
            block.append_plain("That is all.");
        });
        let message = builder.build();
        if let Formatted(formatted_detail) = message {
            let html = formatted_to_html(&formatted_detail);
            assert_eq!(html, "<div><h2>hello world</h2><p>Dear fellow inhabitants. it has come to my attention that <b>you have not been doing your job.</b></p></div><p>That is all.</p>")
        }
        else {
            panic!("oops");
        }
    }
}