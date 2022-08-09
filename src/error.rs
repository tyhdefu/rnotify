use core::fmt::{Debug, Display, Formatter};
use std::error::Error;

#[derive(Debug)]
pub struct MessageSendError {
    msg: String,
}

impl MessageSendError {
    pub fn new(msg: String) -> Self {
        Self {
            msg,
        }
    }
}

impl Error for MessageSendError {}

impl Display for MessageSendError {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "Error sending message: {}", self.msg)
    }
}