use crate::message::Message;

pub mod file;
#[cfg(feature = "discord")]
pub mod discord;

pub trait MessageDestination {
    fn send(&self, message: &Message) -> Result<(), Box<dyn std::error::Error>>;
}