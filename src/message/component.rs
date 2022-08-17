use std::fmt::{Display, Formatter};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Component {
    parts: Vec<String>,
}

impl Component {
    pub fn is_child_of(&self, c: &Component) -> bool {
        // If the component has more parts than us, then we aren't a child (more specific than/same as it)
        if self.parts.len() > c.parts.len() {
            return false;
        }
        let l = c.parts.len();
        self.parts[..l] == c.parts[..l]
    }
}

impl From<&str> for Component {
    fn from(s: &str) -> Self {
        Self {
            parts: s.split("/").map(|s| s.to_owned()).filter(|s| !s.is_empty()).collect()
        }
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
        let parent: Component = "root/sub".into();
        let parent2: Component = "root/sub/".into();
        let parent3: Component = "root".into();

        assert!(child.is_child_of(&parent), "parent");
        assert!(child.is_child_of(&parent2), "parent2 - trailing /");
        assert!(child.is_child_of(&parent3), "parent3");
    }
}