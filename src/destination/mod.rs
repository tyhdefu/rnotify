use std::fmt::Debug;
use crate::message::Message;

pub mod routed_destination;
pub mod kinds;
pub mod message_condition_config;

pub trait MessageDestination: Debug {
    fn send(&self, message: &Message) -> Result<(), Box<dyn std::error::Error>>;
}

#[typetag::serde(tag = "type")]
pub trait SerializableDestination: MessageDestination {
    fn as_message_destination(&self) -> &dyn MessageDestination;
}