use crate::Message;

pub mod config;
pub mod kinds;

pub trait MessageDestination {
    fn send(&self, message: &Message) -> Result<(), Box<dyn std::error::Error>>;
}
