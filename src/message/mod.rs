use std::cmp::Ordering;
use std::fmt::Debug;
use message::formatted_detail::FormattedMessageDetail;
use serde::{Serialize, Deserialize};
use crate::message;
use crate::message::author::Author;
use crate::message::component::Component;

pub mod formatted_detail;
pub mod author;
pub mod component;
pub mod builder;
pub mod detail_builder;

/// A Message represents a [`MessageDestination`] independent way to send a message to a platform.
/// **However** - not every destination will support every type of formatting, or may have length / size
/// restrictions - so be aware of this. Destinations should do their best to receive the message, even if they
/// have to ignore formatting to do so.
///
/// To construct a Message, consider using [`MessageBuilder`] or [`MessageDetailBuilder`] to make
/// your life easier.
///
/// # Parts of a Message #
/// - [Level] - Severity of message
/// - Title - Optional, short snappy summary of what the message is about
/// - [MessageDetail] - Structured body text supporting formatting
/// - [Component] - Optional, indicating what the message is about, e.g a program or server.
/// - [Author] - Who created the message
/// - Timestamp - The unix timestamp in milliseconds, showing when the message was sent.
///
/// [`MessageBuilder`]: builder::MessageBuilder
/// [`MessageDetailBuilder`]: detail_builder::MessageDetailBuilder
#[derive(Debug, Clone, PartialEq)]
pub struct Message {
    level: Level,
    title: Option<String>,
    message_detail: MessageDetail,
    component: Option<Component>,
    author: Author,
    unix_timestamp_millis: i64,
}

impl Message {
    pub fn new(level: Level, title: Option<String>,
               message_detail: MessageDetail, component: Option<Component>,
               author: Author, unix_timestamp_millis: i64,
    ) -> Self {
        Self {
            level,
            title,
            message_detail,
            component,
            author,
            unix_timestamp_millis
        }
    }

    pub fn get_level(&self) -> &Level {
        &self.level
    }

    pub fn get_title(&self) -> &Option<String> {
        &self.title
    }

    pub fn get_message_detail(&self) -> &MessageDetail {
        &self.message_detail
    }

    pub fn get_unix_timestamp_millis(&self) -> i64 {
        self.unix_timestamp_millis
    }

    pub fn get_author(&self) -> &Author {
        &self.author
    }

    pub fn get_component(&self) -> &Option<Component> {
        &self.component
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum MessageDetail {
    Raw(String),
    Formatted(FormattedMessageDetail),
}

impl MessageDetail {
    pub fn raw(&self) -> &str {
        match &self {
            MessageDetail::Raw(raw) => raw,
            MessageDetail::Formatted(formatted) => formatted.raw()
        }
    }

    pub fn has_formatting(&self) -> bool {
        matches!(&self, MessageDetail::Formatted(_))
    }
}

impl Default for MessageDetail {
    fn default() -> Self {
        Self::Raw(String::new())
    }
}

/// The level / severity of the [Message]. This can be thought of as the log level.
/// This is used in conjunction to [Component] to indicate how a message should be
/// routed.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "binary", derive(clap::ArgEnum))]
pub enum Level {
    /// Indicates an informational message when everything is working properly.
    /// # Examples #
    /// - A job has completed successfully, e.g a backup
    /// - A daily status update to confirm that everything is running correctly.
    Info,
    /// Used when something unexpected occurs that could be the source of an error,
    /// but requires the user to check whether it is actually an issue.
    ///
    /// # Examples #
    /// - A job took longer than normal
    /// - A job failed that fails fairly regularly, but will presumably sort itself out soon.
    Warn,
    /// Indicates a failure has occurred.
    ///
    /// # Examples #
    /// - A program crashed / had a non-zero exit code
    /// - A monitoring program detected that another program has not completed its job
    ///     - Possibly indicating the program is not running
    ///     - Or the program is malfunctioning
    /// - A program restarted in attempt to recover itself, but is expected to recover safely
    Error,
    /// Indicates a failure in the notifications own configuration / workings.
    ///
    /// # Examples #
    /// - Sent by [`MessageRouter`] to [`Root`] level [`MessageDestination`]s when a message
    /// cannot be sent to one or more destinations (e.g. network failure, invalid tokens)
    ///
    /// [`Root`]: crate::destination::routed_destination::MessageRoutingBehaviour::Root
    /// [`MessageDestination`]: crate::destination::MessageDestination
    /// [`MessageRouter`]: crate::message_router::MessageRouter
    SelfError,
}

impl Default for Level {
    fn default() -> Self {
        Self::Info
    }
}

impl Level {
    pub(crate) fn get_priority(&self) -> u32 {
        match &self {
            Self::Info => 1,
            Self::Warn => 3,
            Self::Error => 4,
            Self::SelfError => 5,
        }
    }

    /// Gets the least severe [Level]
    pub fn min() -> Level {
        Level::Info
    }

    /// Gets the most severe [Level]
    pub fn max() -> Level {
        Level::SelfError
    }
}

impl PartialOrd<Self> for Level {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Level {
    fn cmp(&self, other: &Self) -> Ordering {
        self.get_priority().cmp(&other.get_priority())
    }
}
