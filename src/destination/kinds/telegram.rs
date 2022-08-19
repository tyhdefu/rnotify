use std::error::Error;
use std::fmt::Debug;
use chrono::{Local, SecondsFormat, TimeZone};
use serde::{Serialize, Deserialize};
use super::MessageDestination;
use crate::{http_util, Message};
use crate::destination::message_condition_config::MessageNotifyConditionConfigEntry;
use crate::message::formatted_detail::{FormattedMessageComponent, FormattedString, Style};
use crate::message::MessageDetail;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct TelegramDestination {
    bot_token: String,
    chat_id: String,
    #[serde(default = "Vec::new")]
    notify: Vec<MessageNotifyConditionConfigEntry<bool>>,
}

#[derive(Serialize, Debug)]
struct TelegramMessage {
    chat_id: String,
    text: String,
    disable_notification: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    parse_mode: Option<ParseMode>,
}

#[derive(Serialize, Debug)]
enum ParseMode {
    HTML,
}

impl TelegramMessage {
    pub fn new(chat_id: String, message: String, notify: bool, parse_mode: Option<ParseMode>) -> Self {
        Self {
            chat_id,
            text: message,
            disable_notification: !notify,
            parse_mode,
        }
    }
}

impl TelegramDestination {
    fn to_tg_message(&self, message: &Message) -> TelegramMessage {
        let html_formatting = message.get_message_detail().has_formatting();

        let mut content = String::new();
        content.push_str(&format!("{:?}", message.get_level()));
        if let Some(title) = message.get_title() {
            if html_formatting {
                content.push_str(&format!(": <b>{}</b>", title));
            } else {
                content.push_str(&format!(": {}", title));
            }
        }
        content.push('\n');

        match message.get_message_detail() {
            MessageDetail::Raw(raw) => content.push_str(raw),
            MessageDetail::Formatted(formatted) => {
                for component in formatted.components() {
                    match component {
                        FormattedMessageComponent::Section(title, section_content) => {
                            content.push_str(&format!("<b><u>{}</u></b>\n", title));
                            for part in section_content {
                                content.push_str(&to_tg_format(part))
                            }
                        }
                        FormattedMessageComponent::Text(parts) => {
                            for part in parts {
                                content.push_str(&to_tg_format(part))
                            }
                        }
                    }
                }
            }
        }


        content.push('\n');
        content.push_str("-----\n");
        let timestamp = Local::timestamp_millis(&Local, message.get_unix_timestamp_millis());
        let timestamp_string = timestamp.to_rfc3339_opts(SecondsFormat::Millis, true);
        if html_formatting {
            content.push_str(&format!("<pre>{}</pre>", timestamp_string));
        } else {
            content.push_str(&timestamp_string);
        }

        content.push('\n');
        content.push_str(&format!("@ {}", message.get_author()));
        let parse_mode = if html_formatting { Some(ParseMode::HTML) } else { None };
        let notify = self.notify.iter()
            .filter(|n| n.matches(&message))
            .map(|n| n.get_notify())
            .any(|b| *b);
        TelegramMessage::new(self.chat_id.clone(), content, notify, parse_mode)
    }
}

impl MessageDestination for TelegramDestination {
    fn send(&self, message: &Message) -> Result<(), Box<dyn Error>> {
        // TODO: Add component and pretty up.
        let message = self.to_tg_message(message);

        let url = format!("https://api.telegram.org/bot{}/sendMessage", self.bot_token);

        http_util::post_as_json_to(&url, &message)
    }
}

fn to_tg_format(formatted_string: &FormattedString) -> String {
    let mut result = String::from(formatted_string.get_string());
    for style in formatted_string.get_styles() {
        result = apply_style(&result, style);
    }
    result
}

fn apply_style(s: &str, style: &Style) -> String {
    match style {
        Style::Bold => format!("*{}*", s),
        Style::Italics => format!("_{}_", s),
        Style::Monospace => format!("`{}`", s),
    }
}