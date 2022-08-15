use std::error::Error;
use std::fmt::Debug;
use serde::{Deserialize, Serialize};
use crate::destination::kinds::DestinationKind;
use crate::Message;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DestinationConfig {
    // Whether errors with sending notifications will be reported to this destination.
    #[serde(default)] // Default false.
    root: bool,
    #[serde(flatten)]
    dest_type: DestinationKind,
}

impl DestinationConfig {
    pub fn new(root: bool, dest_type: DestinationKind) -> Self {
        Self {
            root,
            dest_type,
        }
    }

    pub fn send(&self, message: &Message) -> Result<(), Box<dyn Error>> {
        self.dest_type.send_to_destination(message)
    }

    pub fn is_root(&self) -> bool {
        self.root
    }
}