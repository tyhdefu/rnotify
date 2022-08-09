use std::fmt::{Debug, Display};
use chrono::TimeZone;
use serde::{Deserialize, Serialize};
use crate::destinations::file::FileDestination;
use crate::destinations::MessageDestination;

#[cfg(feature = "discord")]
use crate::destinations::discord::DiscordDestination;
use crate::Message;

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum DestinationKind {
    File(FileDestination),
    #[cfg(feature = "discord")]
    Discord(DiscordDestination),
}

impl DestinationKind {
    pub fn send_to_destination<TZ: TimeZone>(&self, message: &Message<TZ>) -> Result<(), Box<dyn std::error::Error>>
        where TZ::Offset: Display {
        match &self {
            DestinationKind::File(dest) => dest.send(message),
            #[cfg(feature = "discord")]
            DestinationKind::Discord(dest) => dest.send(message),
        }
    }
}
