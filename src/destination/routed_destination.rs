use std::error::Error;
use std::fmt::Debug;
use serde::{Deserialize, Serialize};
use crate::destination::MessageDestination;
use crate::message::Message;
use crate::message_router::RoutingInfo;

/// A [Message] that also contains [RoutingInfo].
pub trait RoutedDestination {
    /// The id provides an identifier
    /// for error reporting.
    fn get_id(&self) -> &str;

    /// The message destination that messages will
    /// can be sent to.
    fn get_destination(&self) -> &dyn MessageDestination;

    /// The routing requirements of this destination.
    fn get_routing_info(&self) -> &RoutingInfo;

    fn send(&self, message: &Message) -> Result<(), Box<dyn Error>> {
        self.get_destination().send(message)
    }

    fn is_root(&self) -> bool {
        self.get_routing_info().get_routing_behaviour() == &MessageRoutingBehaviour::Root
    }

    fn get_routing_type(&self) -> &MessageRoutingBehaviour {
        &self.get_routing_info().get_routing_behaviour()
    }

    fn should_receive(&self, m: &Message) -> bool {
        self.get_routing_info().applies_to(m)
    }
}

/// Handles whether messages are routed here / if they will be routed to other destinations.
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub enum MessageRoutingBehaviour {
    /// [`SelfError`] messages in addition to all messages will be sent here.
    ///
    /// It is recommended to have at least one Root destination, as this serves as a "log"
    /// for all the notifications.
    /// This is normally a [`FileDestination`] since that unlikely to fail.
    ///
    /// [`SelfError`]: crate::message::Level::SelfError
    /// [`FileDestination`]: crate::destination::kinds::file::FileDestination
    Root,
    /// Messages will be sent here if they would not be sent elsewhere (excluding [Self::Root] destinations).
    /// Useful if you want to route "unsorted" messages. A "lazy" destination - checks everything else first.
    Drain,
    /// The default option - Messages will be sent here under normal circumstances.
    Additive
}

impl MessageRoutingBehaviour {
    pub fn always_send_messages(&self) -> bool {
        match &self {
            MessageRoutingBehaviour::Root => true,
            MessageRoutingBehaviour::Additive => true,

            MessageRoutingBehaviour::Drain => false,
        }
    }

    pub fn always_receives_errors(&self) -> bool {
        match &self {
            MessageRoutingBehaviour::Root => true,
            MessageRoutingBehaviour::Drain => false,
            MessageRoutingBehaviour::Additive => false,
        }
    }
}

impl Default for MessageRoutingBehaviour {
    fn default() -> Self {
        MessageRoutingBehaviour::Additive
    }
}

// Implementations //

#[derive(Debug)]
pub struct RoutedDestinationBase {
    id: String,
    // Whether errors with sending notifications will be reported to this destination.
    destination: Box<dyn MessageDestination>,
    routing_info: RoutingInfo,
}

impl RoutedDestinationBase {
    pub fn new(id: String, destination: Box<dyn MessageDestination>, routing_info: RoutingInfo) -> Self {
        Self {
            id,
            destination,
            routing_info,
        }
    }

    pub fn create<M: MessageDestination + 'static>(id: String, destination: M, routing_info: RoutingInfo) -> Self {
        Self {
            id,
            destination: Box::new(destination),
            routing_info,
        }
    }
}

impl RoutedDestination for RoutedDestinationBase {
    fn get_id(&self) -> &str {
        &self.id
    }

    fn get_destination(&self) -> &dyn MessageDestination {
        &*self.destination
    }

    fn get_routing_info(&self) -> &RoutingInfo {
        &self.routing_info
    }
}


#[cfg(test)]
mod test {
    use std::sync::mpsc;
    use std::sync::mpsc::TryRecvError;
    use super::*;
    use crate::destination::kinds::rust_receiver::RustReceiverDestination;
    use crate::message::{Level, MessageDetail};
    use crate::message::author::Author;

    #[test]
    pub fn test_send_message() {
        let (send, recv) = mpsc::channel();
        let dest = RoutedDestinationBase::create("test".to_owned(), RustReceiverDestination::create(send), RoutingInfo::root());

        let message = Message::new(Level::Info,
                                   None, MessageDetail::Raw("hello".to_owned()),
                                   None, Author::parse("test".to_owned()), 104892);


        assert_eq!(recv.try_recv(), Err(TryRecvError::Empty), "Should be empty before we send a message");

        dest.send(&message).expect("Should not fail to send message");

        assert_eq!(recv.try_recv(), Ok(message));
    }
}