use std::error::Error;
use std::fs::File;
use std::path::PathBuf;
use serde::{Serialize, Deserialize};
use std::fmt::{Debug, Write};
use std::fs;
use std::io::Write as IoWrite;
use chrono::{Local, SecondsFormat, TimeZone};
use crate::destination::{MessageDestination, SerializableDestination};
use crate::message::Message;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct FileDestination {
    path: PathBuf,
}

impl MessageDestination for FileDestination {
    fn send(&self, message: &Message) -> Result<(), Box<dyn Error>> {
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

#[typetag::serde(name = "File")]
impl SerializableDestination for FileDestination {
    fn as_message_destination(&self) -> &dyn MessageDestination {
        self
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
        let timestamp = Local::timestamp_millis(&Local, message.get_unix_timestamp_millis());
        write!(s, "{} - {:?}: ", timestamp.to_rfc3339_opts(SecondsFormat::Millis, true), message.get_level()).unwrap();
        if message.get_component().is_some() {
            write!(s, "[{}] ", message.get_component().as_ref().unwrap()).unwrap();
        }
        if message.get_title().is_some() {
            write!(s, "{} - ", message.get_title().as_ref().unwrap()).unwrap();
        }
        write!(s, "'{}'", inline(message.get_message_detail().raw())).unwrap();
        write!(s, " @ {}", message.get_author()).unwrap();
        s
    }
}

fn inline(s: &str) -> String {
    let vec: Vec<_> = s.lines().collect();
    vec.join("\\n")
}