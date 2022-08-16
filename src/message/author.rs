use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub struct Author {
    parts: Vec<String>,
}

impl Author {
    pub fn parse(s: String) -> Author {
        let mut vec: Vec<String> = s.split("/").into_iter().filter(|s| !s.is_empty()).map(|s| s.to_owned()).collect();
        match hostname::get() {
            Ok(hostname) => vec.insert(0, hostname.to_string_lossy().into()),
            Err(e) => {
                vec.insert(0, "?".to_owned());
                eprintln!("Failed to detect hostname {}", e);
            },
        }
        Self {
            parts: vec
        }
    }
}

impl Display for Author {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.parts.join("/"))
    }
}