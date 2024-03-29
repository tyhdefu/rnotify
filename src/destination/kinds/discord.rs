use std::error::Error;
use chrono::{SecondsFormat, TimeZone, Utc};
use serde::{Serialize, Deserialize};
use crate::util::http_util;
use crate::destination::message_condition::MessageNotifyConditionConfigEntry;
use crate::destination::{MessageDestination, SerializableDestination};
use crate::message::formatted_detail::{FormattedMessageComponent, FormattedString, Style};
use crate::message::{Level, Message, MessageDetail};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DiscordDestination {
    url: String,
    username: Option<String>,
    #[serde(default = "Vec::new")]
    notify: Vec<MessageNotifyConditionConfigEntry<String>>,
}

impl DiscordDestination {
    pub fn new(url: String) -> Self {
        Self {
            url,
            username: None,
            notify: vec![]
        }
    }

    fn to_discord_message(&self, message: &Message) -> discord_webhook::models::Message {
        let mut discord_msg = discord_webhook::models::Message::new();

        let notify_receivers: Vec<String> = self.notify.iter().filter(|n| n.matches(message))
            .map(|n| n.get_notify())
            .map(|s| s.to_owned())
            .collect();

        if !notify_receivers.is_empty() {
            let content = notify_receivers.join(" ");
            discord_msg.content(&content);
        }

        discord_msg.embed(|embed| {
            embed.title(message.get_title().as_deref().unwrap_or("Rnotify Notification"));

            let timestamp = Utc::timestamp_millis(&Utc, message.get_unix_timestamp_millis());
            let footer_str = format!("{} @ {}\n{} v{}",
                                     timestamp.to_rfc3339_opts(SecondsFormat::Millis, true),
                                     message.get_author(),
                                     env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
            embed.footer(&footer_str, None);
            let color = get_color_from_level(message.get_level());
            embed.color(&format!("{}", color));

            if let Some(component) = message.get_component() {
                embed.author(&format!("[{}]", component), None, None);
            }

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
        Style::Monospace => {
            if s.is_empty() || s.contains('\n') {
                format!("```\n{}\n```", s);
            }
            format!("`{}`", s)
        },
        Style::Code { lang} => format!("```{}\n{}```", lang, s),
    }
}

impl MessageDestination for DiscordDestination {
    fn send(&self, message: &Message) -> Result<(), Box<dyn Error>> {
        let discord_msg = self.to_discord_message(message);
        //let payload = serde_json::to_string(&discord_msg)?;
        http_util::post_as_json_to(&self.url, &discord_msg)
    }
}

#[typetag::serde(name = "Discord")]
impl SerializableDestination for DiscordDestination {
    fn as_message_destination(&self) -> &dyn MessageDestination {
        self
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

#[cfg(test)]
mod tests {
    use std::fs;
    use crate::destination::kinds::discord::DiscordDestination;

    #[test]
    fn test_deserialize() {
        let s = fs::read_to_string("test/discord_example.toml").expect("Should be able to read file");
        let dest: DiscordDestination = toml::from_str(&s).expect("Should deserialize");

        let expected = DiscordDestination::new("https://discord.com/api/webhooks/11111111111111/2aaaaaaaaaaaaaaaaa".to_string());

        assert_eq!(dest, expected);
    }
}