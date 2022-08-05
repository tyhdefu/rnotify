use std::time::Instant;
use clap::clap_derive::ValueEnum;

#[derive(Debug)]
pub struct Message {
    level: Level,
    title: Option<String>,
    message_detail: String,
    component: Option<String>,
    author: Option<String>,
    timestamp: Instant,
}

impl Message {
    pub fn new(level: Level, title: Option<String>,
               message_detail: String, component: Option<String>,
               author: Option<String>, timestamp: Instant,
    ) -> Self {
        Self {
            level,
            title,
            message_detail,
            component,
            author,
            timestamp
        }
    }

    pub fn get_level(&self) -> &Level {
        &self.level
    }

    pub fn get_title(&self) -> &Option<String> {
        &self.title
    }

    pub fn get_message_detail(&self) -> &String {
        &self.message_detail
    }

    pub fn get_timestamp(&self) -> &Instant {
        &self.timestamp
    }

    pub fn get_author(&self) -> &Option<String> {
        &self.author
    }

    pub fn get_component(&self) -> &Option<String> {
        &self.component
    }
}

#[derive(Debug, ValueEnum, Clone)]
pub enum Level {
    Info,
    Warn,
    Error,
    SelfError,
}