use std::fmt::Display;
use chrono::TimeZone;
use crate::message::Message;

pub mod file;
#[cfg(feature = "discord")]
pub mod discord;

pub trait MessageDestination {
    fn send<TZ: TimeZone>(&self, message: &Message<TZ>) -> Result<(), Box<dyn std::error::Error>>
        where TZ::Offset: Display;
}