use std::time::{SystemTime, UNIX_EPOCH};
use crate::message::component::Component;
use crate::message::{Level, Message, MessageDetail};
use crate::message::author::Author;
use crate::message::detail_builder::MessageDetailBuilder;

pub struct MessageBuilder {
    level: Level,
    title: Option<String>,
    detail: MessageDetail,
    component: Option<Component>,
    author: Author,
    unix_timestamp_millis: Option<i64>,
}

impl MessageBuilder {
    /// Creates a new [MessageBuilder] with the following defaults:
    /// - Level -> [Level]'s default
    /// - Author -> The hostname of the OS or '?' if that cannot be retrieved
    pub fn new() -> Self {
        let author = Author::base();

        Self {
            level: Default::default(),
            title: None,
            detail: Default::default(),
            component: None,
            author,
            unix_timestamp_millis: None,
        }
    }

    /// Add a message body to the message builder
    /// If not set it will be blank.
    pub fn body<F>(&mut self, apply: F) -> &mut Self
        where F: FnOnce(&mut MessageDetailBuilder) {
        let mut builder = MessageDetailBuilder::new();
        apply(&mut builder);
        self.detail = builder.build();
        self
    }

    /// Set the message [Level]
    pub fn level(&mut self, level: Level) -> &mut Self {
        self.level = level;
        self
    }

    /// Sets the title of the message,
    /// If not set, will be blank.
    pub fn title<S: ToString>(&mut self, title: S) -> &mut Self {
        self.title = Some(title.to_string());
        self
    }

    /// Sets the [Component] of the message
    /// By default the component will be blank
    pub fn component(&mut self, component: Component) -> &mut Self {
        self.component = Some(component);
        self
    }

    /// Adds [Author] data to the message.
    /// By default this is just the hostname of OS running it ('?' if this cannot be retrieved)
    /// The argument parts is appended onto this hostname data (with a '/')
    pub fn author<S: ToString>(&mut self, parts: S) -> &mut Self {
        self.author.extend(parts.to_string());
        self
    }

    /// Sets the timestamp of the message.
    /// This is set by default to the time which the [MessageBuilder] was created.
    ///
    /// However, if you are using [`build_clone`](Self::build_clone) you should be using this
    /// if you don't want your messages to all have the same timestamp.
    pub fn timestamp(&mut self, unix_timestamp_millis: i64) -> &mut Self {
        self.unix_timestamp_millis = Some(unix_timestamp_millis);
        self
    }

    /// Builds the message, consuming the [MessageBuilder]
    /// If [`timestamp`](Self::timestamp) has been set, it will be used,
    /// otherwise the current time will be retrieved and used.
    pub fn build(self) -> Message {

        Message {
            level: self.level,
            title: self.title,
            message_detail: self.detail,
            component: self.component,
            author: self.author,
            unix_timestamp_millis: self.unix_timestamp_millis.unwrap_or_else(Self::get_unix_time_millis),
        }
    }

    /// Builds a new message, without consuming the [MessageBuilder]
    /// If [`timestamp`](Self::timestamp) has been set, it will be used,
    /// otherwise the current time will be retrieved and used.
    pub fn build_clone(&self) -> Message {
        Message {
            level: self.level.clone(),
            title: self.title.clone(),
            message_detail: self.detail.clone(),
            component: self.component.clone(),
            author: self.author.clone(),
            unix_timestamp_millis: self.unix_timestamp_millis.unwrap_or_else(Self::get_unix_time_millis)
        }
    }

    fn get_unix_time_millis() -> i64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Expected now to be after unix epoch")
            .as_millis() as i64
    }
}