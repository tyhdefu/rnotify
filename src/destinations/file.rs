use std::error::Error;
use std::fs::File;
use std::path::PathBuf;
use serde::{Serialize, Deserialize};
use std::fmt::{Debug, Display, Write};
use std::fs;
use std::io::Write as IoWrite;
use chrono::{SecondsFormat, TimeZone};
use crate::destinations::MessageDestination;
use crate::message::Message;

#[derive(Serialize, Deserialize, Debug)]
pub struct FileDestination {
    path: PathBuf,
}

impl MessageDestination for FileDestination {
    fn send<TZ: TimeZone>(&self, message: &Message<TZ>) -> Result<(), Box<dyn Error>>
        where TZ::Offset: Display {
        if let Some(parent) = self.path.parent() {
            if !parent.exists() {
                fs::create_dir_all(parent)?;
            }
        }
        let s = self.format_message(message);
        let mut file = File::options()
            .create(true)
            .append(true)
            .open(&self.path)?;

        writeln!(&mut file, "{}", s)?;
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
    fn format_message<TZ: TimeZone>(&self, message: &Message<TZ>) -> String
        where TZ::Offset: Display {
        let mut s = String::new();
        write!(s, "{} - {:?}: ", message.get_timestamp().to_rfc3339_opts(SecondsFormat::Millis, true), message.get_level()).unwrap();
        if message.get_component().is_some() {
            write!(s, "[{}] ", message.get_component().as_ref().unwrap()).unwrap();
        }
        if message.get_title().is_some() {
            write!(s, "{} - ", message.get_title().as_ref().unwrap()).unwrap();
        }
        write!(s, "'{}'", message.get_message_detail()).unwrap();
        if message.get_author().is_some() {
            write!(s, " @ {}", message.get_author().as_ref().unwrap()).unwrap();
        }
        s
    }
}