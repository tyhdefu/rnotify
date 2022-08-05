use std::error::Error;
use std::fs::File;
use std::path::PathBuf;
use serde::{Serialize, Deserialize};
use std::fmt::Write;
use std::io::Write as IoWrite;
use crate::destinations::MessageDestination;
use crate::message::Message;

#[derive(Serialize, Deserialize)]
pub struct FileDestination {
    path: PathBuf,
}

impl MessageDestination for FileDestination {
    fn send(&self, message: &Message) -> Result<(), Box<dyn Error>> {
        let s = self.format_message(message);
        let mut file = File::options()
            .create(true)
            .append(true)
            .open(&self.path)?;

        file.write(s.as_bytes())?;
        Ok(())
    }
}

impl FileDestination {
    pub fn new(path: PathBuf) -> Self {
        Self {
            path
        }
    }

    // TODO: Allow custom format.
    fn format_message(&self, message: &Message) -> String {
        let mut s = String::new();
        write!(s, "{:?} - {:?}:", message.get_timestamp(), message.get_level()).unwrap();
        if message.get_title().is_some() {
            write!(s, " {} - ", message.get_title().as_ref().unwrap()).unwrap();
        }
        write!(s, "'{}'", message.get_message_detail()).unwrap();
        if message.get_author().is_some() {
            write!(s, " @ {}", message.get_author().as_ref().unwrap()).unwrap();
        }
        s
    }
}