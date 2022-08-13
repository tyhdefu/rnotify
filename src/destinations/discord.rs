use std::error::Error;
use std::fmt::Display;
use chrono::{SecondsFormat, TimeZone};
use serde::{Serialize, Deserialize};
use crate::destinations::MessageDestination;
use crate::{curl_util, Level, MessageDetail};
use crate::formatted_message_detail::{FormattedMessageComponent, FormattedString, Style};
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

impl DiscordDestination {
    fn to_discord_message<TZ: TimeZone>(&self, message: &Message<TZ>) -> discord_webhook::models::Message
        where TZ::Offset: Display {
        let mut discord_msg = discord_webhook::models::Message::new();
        discord_msg.content(&self.message_content);
        discord_msg.embed(|embed| {
            embed.title(message.get_title().as_deref().unwrap_or("Rnotify Notification"));
            embed.timestamp(&message.get_timestamp().to_rfc3339_opts(SecondsFormat::Millis, true));
            let color = get_color_from_level(message.get_level());
            embed.color(&format!("{}", color));
            if let Some(author) = message.get_author() {
                embed.author(author, None, None);
            }

            match message.get_message_detail() {
                MessageDetail::Raw(raw) => { embed.description(raw); },
                MessageDetail::Formatted(formatted) => {
                    for component in formatted.components() {
                        println!("Component: {:?}", component);
                        match component {
                            FormattedMessageComponent::Section(title, contents) => {
                                let string: String = contents.iter().map(|s| to_discord_format(s)).collect();
                                embed.field(title, &string, false);
                            }
                            FormattedMessageComponent::Text(text) => {
                                let string: String = text.iter().map(|s| to_discord_format(s)).collect();
                                embed.description(&string);
                            }
                        }
                    }
                }
            }

            return embed;
        });

        discord_msg
    }
}

fn to_discord_format(formatted_string: &FormattedString) -> String {
    let mut result = String::from(formatted_string.get_string());
    for style in formatted_string.get_styles() {
        result = apply_style(&result, style);
    }
    result
}

fn apply_style(s: &str, style: &Style) -> String {
    match style {
        Style::Bold => format!("**{}**", s),
        Style::Italics => format!("_{}_", s),
        Style::Monospace => format!("`{}`", s),
    }
}

impl MessageDestination for DiscordDestination {
    fn send<TZ: TimeZone>(&self, message: &Message<TZ>) -> Result<(), Box<dyn Error>>
        where TZ::Offset: Display {
        let discord_msg = self.to_discord_message(message);
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