use std::error::Error;
use std::fmt::Debug;
use serde::{Deserialize, Serialize};
use crate::destination::kinds::DestinationKind;
use crate::destination::message_condition_config::MessageCondition;
use crate::Message;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DestinationConfig {
    // Whether errors with sending notifications will be reported to this destination.
    #[serde(default)]
    routing_type: MessageRoutingBehaviour,
    #[serde(flatten)]
    dest_type: DestinationKind,
    #[serde(skip_serializing_if = "Option::is_none")]
    applies_to: Option<MessageCondition>,
}

/// Handles whether messages are routed here / if they will be routed to other destinations.
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub enum MessageRoutingBehaviour {
    /// [crate::message::Level::SelfError] messages in addition to all messages will be sent here.
    Root,
    /// Messages will be sent here if they would not be sent elsewhere (excludes [Self::Root] destinations).
    Drain,
    /// Messages will be sent here
    Additive
}

impl Default for MessageRoutingBehaviour {
    fn default() -> Self {
        MessageRoutingBehaviour::Additive
    }
}

impl DestinationConfig {
    pub fn new(routing_type: MessageRoutingBehaviour, dest_type: DestinationKind, applies_to: Option<MessageCondition>) -> Self {
        Self {
            routing_type,
            dest_type,
            applies_to,
        }
    }

    pub fn send(&self, message: &Message) -> Result<(), Box<dyn Error>> {
        self.dest_type.send_to_destination(message)
    }

    pub fn is_root(&self) -> bool {
        self.routing_type == MessageRoutingBehaviour::Root
    }

    pub fn get_routing_type(&self) -> &MessageRoutingBehaviour {
        &self.routing_type
    }

    pub fn should_receive(&self, m: &Message) -> bool {
        match &self.applies_to  {
            Some(filter) => filter.matches(m),
            None => true,
        }
    }
}


#[cfg(test)]
mod test {
    use std::sync::mpsc;
    use std::sync::mpsc::TryRecvError;
    use super::*;
    use crate::{Level, Message};
    use crate::destination::kinds::rust_receiver::RustReceiverDestination;
    use crate::message::MessageDetail;

    #[test]
    pub fn test_send_message() {
        let (send, recv) = mpsc::channel();
        let dest = DestinationConfig::new(Default::default(),
                                          DestinationKind::Test(RustReceiverDestination::create(send)),
                                          None);

        let message = Message::new(Level::Info,
                                   None, MessageDetail::Raw("hello".to_owned()),
                                   None, None, 104892);


        assert_eq!(recv.try_recv(), Err(TryRecvError::Empty), "Should be empty before we send a message");

        dest.send(&message).expect("Should not fail to send message");

        assert_eq!(recv.try_recv(), Ok(message));
    }
}