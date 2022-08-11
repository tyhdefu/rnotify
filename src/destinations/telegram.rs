use std::error::Error;
use std::fmt::Display;
use chrono::{SecondsFormat, TimeZone};
use serde::{Serialize, Deserialize};
use crate::destinations::MessageDestination;
use crate::{curl_util, Message};

#[derive(Serialize, Deserialize, Debug)]
pub struct TelegramDestination {
    bot_token: String,
    chat_id: String,
}

#[derive(Serialize)]
struct TelegramMessage {
    chat_id: String,
    text: String,
    disable_notification: bool,
}

impl TelegramMessage {
    pub fn new(chat_id: String, message: String) -> Self {
        Self {
            chat_id,
            text: message,
            disable_notification: false,
        }
    }
}

impl MessageDestination for TelegramDestination {
    fn send<TZ: TimeZone>(&self, message: &Message<TZ>) -> Result<(), Box<dyn Error>> where TZ::Offset: Display {
        // TODO: Add component and pretty up.
        let mut content = String::new();
        content.push_str(&format!("{:?}", message.get_level()));
        if let Some(title) = message.get_title() {
            content.push_str(&format!(": {}\n", title));
        }
        else {
            content.push_str(&format!("\n"));
        }
        content.push_str(message.get_message_detail());
        content.push('\n');
        content.push('\n');
        content.push_str(&message.get_timestamp().to_rfc3339_opts(SecondsFormat::Millis, true));
        if let Some(author) = message.get_author() {
            content.push('\n');
            content.push_str(&format!("@ {}", author));
        }
        let message = TelegramMessage::new(self.chat_id.clone(), content);

        let url = format!("https://api.telegram.org/bot{}/sendMessage", self.bot_token);

        let payload = serde_json::to_string(&message)?;

        curl_util::post_json_to(&url, &payload)
    }
}