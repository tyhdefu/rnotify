use std::error::Error;
use crate::message::Message;
use crate::send_error::SendError;

#[derive(Debug)]
pub struct SendErrorOwned {
    err: Box<dyn Error>,
    destination_id: String,
    message: Message,
}

impl SendErrorOwned {
    pub fn create(err: Box<dyn Error>, destination_id: String, message: Message) -> Self {
        Self {
            err,
            destination_id,
            message,
        }
    }
}

impl SendError for SendErrorOwned {
    fn get_error(&self) -> &Box<dyn Error> {
        &self.err
    }

    fn get_failed_destination_id(&self) -> &str {
        &self.destination_id
    }

    fn get_failed_message(&self) -> &Message {
        &self.message
    }
}