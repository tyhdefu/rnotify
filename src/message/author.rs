use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, PartialEq)]
pub struct Author {
    parts: Vec<String>,
}

impl Author {
    pub fn parse(parts: String) -> Author {
        let mut new = Self::base();
        new.extend(parts);
        new
    }

    pub fn base() -> Author {
        let base = match hostname::get() {
            Ok(hostname) => {
                hostname.to_string_lossy().to_string()
            }
            Err(_) => {
                "?".to_owned()
            }
        };

        Author {
            parts: vec![base],
        }
    }

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