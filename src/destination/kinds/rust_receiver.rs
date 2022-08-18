use std::error::Error;
use std::sync::mpsc::Sender;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use crate::destination::MessageDestination;
use crate::Message;

#[derive(Debug, Clone)]
pub struct RustReceiverDestination {
    sender: Sender<Message>
}

impl RustReceiverDestination {
    pub fn create(sender: Sender<Message>) -> Self  {
        Self {
            sender,
        }
    }
}

impl MessageDestination for RustReceiverDestination {
    fn send(&self, message: &Message) -> Result<(), Box<dyn Error>> {
        self.sender.send(message.clone())?;
        Ok(())
    }
}

impl Serialize for RustReceiverDestination {
    fn serialize<S>(&self, _: S) -> Result<S::Ok, S::Error> where S: Serializer {
        panic!("Not possible - testing only.")
    }
}

impl<'de> Deserialize<'de> for RustReceiverDestination {
    fn deserialize<D>(_: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        panic!("Not possible - testing only")
    }
}