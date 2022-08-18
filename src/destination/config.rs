use std::error::Error;
use std::fmt::Debug;
use serde::{Deserialize, Serialize};
use crate::destination::kinds::DestinationKind;
use crate::destination::message_condition_config::MessageCondition;
use crate::Message;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DestinationConfig {
    // Whether errors with sending notifications will be reported to this destination.
    #[serde(default)] // Default false.
    root: bool,
    #[serde(flatten)]
    dest_type: DestinationKind,
    #[serde(skip_serializing_if = "Option::is_none")]
    applies_to: Option<MessageCondition>,
}

impl DestinationConfig {
    pub fn new(root: bool, dest_type: DestinationKind, applies_to: Option<MessageCondition>) -> Self {
        Self {
            root,
            dest_type,
            applies_to,
        }
    }

    pub fn send(&self, message: &Message) -> Result<(), Box<dyn Error>> {
        self.dest_type.send_to_destination(message)
    }

    pub fn is_root(&self) -> bool {
        self.root
    }

    pub fn should_receive(&self, m: &Message) -> bool {
        match &self.applies_to  {
            Some(filter) => filter.matches(m),
            None => true,
        }
    }
}