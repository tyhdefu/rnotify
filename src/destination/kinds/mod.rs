pub mod file;
#[cfg(feature = "discord")]
pub mod discord;
#[cfg(feature = "mail")]
pub mod mail;
#[cfg(feature = "telegram")]
pub mod telegram;
#[cfg(test)]
pub mod rust_receiver;

use std::fmt::Debug;
use serde::{Deserialize, Serialize};
use crate::destination::MessageDestination;
use crate::Message;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(tag = "type", deny_unknown_fields)]
pub enum DestinationKind {
    #[cfg(test)]
    Test(rust_receiver::RustReceiverDestination),

    File(file::FileDestination),
    #[cfg(feature = "discord")]
    Discord(discord::DiscordDestination),
    #[cfg(feature = "mail")]
    Mail(mail::MailDestination),
    #[cfg(feature = "telegram")]
    Telegram(telegram::TelegramDestination),
}

impl DestinationKind {
    pub fn send_to_destination(&self, message: &Message) -> Result<(), Box<dyn std::error::Error>> {
        match &self {
            #[cfg(test)]
            DestinationKind::Test(dest) => dest.send(message),

            DestinationKind::File(dest) => dest.send(message),
            #[cfg(feature = "discord")]
            DestinationKind::Discord(dest) => dest.send(message),
            #[cfg(feature = "mail")]
            DestinationKind::Mail(dest) => dest.send(message),
            #[cfg(feature = "telegram")]
            DestinationKind::Telegram(dest) => dest.send(message),
        }
    }
}