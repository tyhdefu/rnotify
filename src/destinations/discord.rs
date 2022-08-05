use std::error::Error;
use crate::destinations::MessageDestination;
use crate::message::Message;

pub struct DiscordDestination {

}

impl MessageDestination for DiscordDestination {
    fn send(&self, message: &Message) -> Result<(), Box<dyn Error>> {
        todo!()
    }
}