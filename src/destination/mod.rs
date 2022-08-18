use crate::Message;

pub mod config;
pub mod kinds;
pub mod message_condition_config;

pub trait MessageDestination {
    fn send(&self, message: &Message) -> Result<(), Box<dyn std::error::Error>>;
}
