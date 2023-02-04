use std::fmt::{Display, Formatter};

/// The hostname used when the hostname
/// cannot be retrieved.
const UNKNOWN_HOSTNAME: &str = "?";

/// The author or creator of the message.
///
/// The base author will contain the hostname of the OS that sent to message.
/// This helps identify which machine the message came from. If this is not available then '?' will
/// be used.
///
/// A good author should allow the user to quickly and easily identify where the message
/// originates from.
/// To make a good author, think, could you quickly and easily stop the source of the messages just
/// based on the author?
///
/// Filtering should not be performed based on an Author, rather you should filter instead
/// based on [`Level`] and [`Component`]
///
/// [`Level`]: crate::message::Level
/// [`Component`]: crate::message::component::Component
#[derive(Debug, Clone, PartialEq)]
pub struct Author {
    parts: Vec<String>,
}

impl Author {
    /// Parses an Author from the given string.
    ///
    /// ```rust
    /// // A good Author for a message originating from the db_checker program
    /// // which is invoked by user's crontab.
    /// use rnotifylib::message::author::Author;
    ///
    /// let author = Author::parse("user/cron/db_checker".to_owned());
    /// ```
    pub fn parse(parts: String) -> Author {
        let mut new = Self::base();
        new.extend(parts);
        new
    }

    /// Provides the base Author, of either the hostname.
    /// If the hostname cannot be found, uses [`base_incognito`](Self::base_incognito)
    pub fn base() -> Author {
        let base = match hostname::get() {
            Ok(hostname) => {
                hostname.to_string_lossy().to_string()
            }
            Err(_) => {
                return Self::base_incognito();
            }
        };

        Author {
            parts: vec![base],
        }
    }

    /// Creates an Author with an unknown hostname.
    /// You should use [`base`](Self::base) whenever possible, but if you do not want to expose
    /// the hostname of the sender, you can use this method.
    pub fn base_incognito() -> Author {
        Author {
            parts: vec![UNKNOWN_HOSTNAME.to_string()]
        }
    }

    /// Parses an author, using an unknown hostname
    /// You should use [`parse`](Self::parse) whenever possible, but if you do not want to expose
    /// the hostname of the sender, you can use this method.
    pub fn parse_incognito(s: String) -> Author {
        let mut base = Self::base_incognito();
        base.extend(s);
        base
    }

    /// Adds more information to the author of this
    /// ```rust
    /// use rnotifylib::message::author::Author;
    ///
    /// // Normally you should use Author::base(), but we want predictable output for this test.
    /// let mut author = Author::base_incognito();
    /// author.extend("user".to_owned());
    /// author.extend("cron".to_owned());
    /// author.extend("db_checker".to_owned());
    ///
    /// assert_eq!(author.to_string(), "?/user/cron/db_checker");
    pub fn extend(&mut self, parts: String) {
        let vec: Vec<String> = parts.split("/").into_iter()
            .filter(|s| !s.is_empty())
            .map(|s| s.to_owned())
            .collect();

        self.parts.extend(vec);
    }
}

impl Display for Author {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.parts.join("/"))
    }
}