use std::error::Error;
use chrono::{SecondsFormat, TimeZone, Utc};
use serde::{Serialize, Deserialize};
use crate::{http_util, Level};
use super::MessageDestination;
use crate::message::formatted_detail::{FormattedMessageComponent, FormattedString, Style};
use crate::message::{Message, MessageDetail};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DiscordDestination {
    url: String,
    username: Option<String>,
    message_content: Option<String>,
}

impl DiscordDestination {
    fn to_discord_message(&self, message: &Message) -> discord_webhook::models::Message {
        let mut discord_msg = discord_webhook::models::Message::new();
        if let Some(s) = &self.message_content {
            discord_msg.content(s);
        }
        discord_msg.embed(|embed| {
            embed.title(message.get_title().as_deref().unwrap_or("Rnotify Notification"));
            let timestamp = Utc::timestamp_millis(&Utc, message.get_unix_timestamp_millis());
            embed.timestamp(&timestamp.to_rfc3339_opts(SecondsFormat::Millis, true));
            let color = get_color_from_level(message.get_level());
            embed.color(&format!("{}", color));
            embed.author(&format!("{}", message.get_author()), None, None);

            match message.get_message_detail() {
                MessageDetail::Raw(raw) => { embed.description(raw); },
                MessageDetail::Formatted(formatted) => {
                    for component in formatted.components() {
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
    fn send(&self, message: &Message) -> Result<(), Box<dyn Error>> {
        let discord_msg = self.to_discord_message(message);
        //let payload = serde_json::to_string(&discord_msg)?;
        http_util::post_as_json_to(&self.url, &discord_msg)
    }
}

fn get_color_from_level(level: &Level) -> u32 {
    match level {
        Level::Info => 0x00F4D0,
        Level::Warn => 0xFFFF00,
        Level::Error => 0xFF0000,
        Level::SelfError => 0xB30000,
        Level::SelfInfo => 0x964B00,
    }
}