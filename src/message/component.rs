use std::fmt::{Display, Formatter};
use serde::{Serialize, Deserialize, Deserializer, Serializer};

#[derive(Debug, Clone, PartialEq)]
pub struct Component {
    parts: Vec<String>,
}

impl Component {
    pub fn is_child_of(&self, parent: &Component) -> bool {
        if parent.parts.len() > self.parts.len()  {
            return false;
        }
        let l = parent.parts.len() - 1;
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
        assert!(!child2.is_child_of(&parent4))
    }

    fn should_be_child(child: &Component, parent: &Component, message: &str) {
        assert!(child.is_child_of(parent), "{} should be child of: {} - {}", &child, &parent, message);
    }
}