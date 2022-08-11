use std::error::Error;
use std::fmt::Display;
use chrono::{SecondsFormat, TimeZone};
use serde::{Serialize, Deserialize};
use crate::destinations::MessageDestination;
use crate::{curl_util, Level};
use crate::message::Message;

#[derive(Serialize, Deserialize, Debug)]
pub struct DiscordDestination {
    url: String,
    username: Option<String>,
    #[serde(default = "default_message_content")]
    message_content: String,
}

fn default_message_content() -> String {
    return String::from("@RNotify");
}

impl MessageDestination for DiscordDestination {
    fn send<TZ: TimeZone>(&self, message: &Message<TZ>) -> Result<(), Box<dyn Error>>
        where TZ::Offset: Display {
        let mut discord_msg = discord_webhook::models::Message::new();
        discord_msg.content(&self.message_content);
        discord_msg.embed(|embed| {
            embed.timestamp(&message.get_timestamp().to_rfc3339_opts(SecondsFormat::Millis, true));
            embed.description(message.get_message_detail());
            if let Some(author) = message.get_author() {
                embed.author(author, None, None);
            }
            embed.title(message.get_title().as_deref().unwrap_or("Rnotify Notification"));
            let color = get_color_from_level(message.get_level());
            embed.color(&format!("{}", color));
            return embed;
        });
        let payload = serde_json::to_string(&discord_msg)?;
        curl_util::post_json_to(&self.url, &payload)
    }
}

fn get_color_from_level(level: &Level) -> u32 {
    match level {
        Level::Info => 0x00F4D0,
        Level::Warn => 0xFFFF00,
        Level::Error => 0xFF0000,
        Level::SelfError => 0xB30000,
    }
}