use std::fmt::{Display, Formatter};
use serde::{Serialize, Deserialize, Deserializer, Serializer};

/// Indicates what (program/functionality) a [`Message`] is referring to.
/// This helps route messages to the relevant location, as well
/// as control their severity.
///
/// # Examples #
/// ```rust
/// use rnotifylib::message::component::Component;
///
/// let db_backup_component = Component::from("database/backup");
/// let db_uptime_component = Component::from("database/uptime");
///
/// let db_component = Component::from("database");
///
/// // Both of these are children of the db component - Hence destinations that subscribe
/// // to the database component will receive both backup and uptime messages.
/// assert!(db_backup_component.is_child_of(&db_component), "backup component should be child of db component");
/// assert!(db_uptime_component.is_child_of(&db_component), "uptime component should be child of db component");
///
/// // Additionally, the database component is a "child" of itself,
/// // Therefore messages with the "database" component will be sent to places that listen for the database component
/// assert!(db_component.is_child_of(&db_component));
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct Component {
    parts: Vec<String>,
}

impl Component {
    /// Gets whether this is a child of the given parent.
    /// ```rust
    /// use rnotifylib::message::component::Component;
    ///
    /// // Two child components
    /// let db_backup_component = Component::from("database/backup");
    /// let db_uptime_component = Component::from("database/uptime");
    ///
    /// // Parent database component
    /// let db_component = Component::from("database");
    ///
    /// // Child components of the same thing are children.
    /// assert!(db_backup_component.is_child_of(&db_component), "backup component should be child of db component");
    /// assert!(db_uptime_component.is_child_of(&db_component), "uptime component should be child of db component");
    ///
    /// // And the parent is a child of itself.
    /// assert!(db_component.is_child_of(&db_component), "Should be a child of itself");
    ///
    /// // But the parent is not a child of the child.
    /// assert!(!db_component.is_child_of(&db_backup_component), "database component should not be a child of the backup sub-component");
    ///
    /// let website_component = Component::from("website");
    /// let website_backend_component = Component::from("website/component");
    ///
    /// // Unrelated components are not children of each other
    /// assert!(!db_component.is_child_of(&website_backend_component), "db component shouldn't be a child of website backend component");
    /// assert!(!db_component.is_child_of(&website_component), "db component shouldn't be a child of the website component");
    ///
    /// assert!(!db_backup_component.is_child_of(&website_component), "db backup component shouldn't be a child of the website component");
    /// assert!(!db_backup_component.is_child_of(&website_backend_component), "db backup shouldn't be a child of the website backup component");
    /// ```
    pub fn is_child_of(&self, parent: &Component) -> bool {
        if parent.parts.len() > self.parts.len()  {
            return false;
        }
        let l = parent.parts.len();
        self.parts[..l] == parent.parts[..l]
    }
}

impl From<&str> for Component {
    fn from(s: &str) -> Self {
        Self {
            parts: s.split("/").map(|s| s.to_owned()).filter(|s| !s.is_empty()).collect()
        }
    }
}

impl<'de> Deserialize<'de> for Component {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        let s = String::deserialize(deserializer)?;
        Ok(s.as_str().into())
    }
}

impl Serialize for Component {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        serializer.serialize_str(&self.parts.join("/"))
    }
}

impl Display for Component {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.parts.join("/"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_child_of() {
        let child: Component = "root/sub/block".into();
        let parent1: Component = "root".into();
        let parent2: Component = "root/sub".into();
        let parent3: Component = "root/sub/".into();

        should_be_child(&child, &parent1, "basic");
        should_be_child(&child, &parent2, "depth 2");
        should_be_child(&child, &parent3, "parent trailing /");

        let parent4: Component = "scraperpi/services".into();
        let child2: Component = "scraperpi".into();
        assert!(!child2.is_child_of(&parent4));

        let parent4: Component = "heating".into();
        let child3: Component = "heating/test".into();
        assert!(child3.is_child_of(&parent4));
    }

    fn should_be_child(child: &Component, parent: &Component, message: &str) {
        assert!(child.is_child_of(parent), "{} should be child of: {} - {}", &child, &parent, message);
    }

    #[test]
    fn completely_different() {
        let child: Component = "aaa".into();
        let parent: Component = "heating".into();
        assert!(!child.is_child_of(&parent));
    }
}