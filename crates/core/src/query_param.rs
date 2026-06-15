use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QueryParam {
    pub name: String,
    pub value: String,
    pub comments: String,
    pub is_active: bool,
}

impl Display for QueryParam {  
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}={} ({})",
            self.name,
            self.value,
            self.is_active
        )
    }
}

impl QueryParam {
    pub fn new(name: impl Into<String>, value: impl Into<String>, is_active: bool, comments: impl Into<String>) -> Self {
        Self {
            name: name.into().trim().into(),
            value: value.into().trim().into(),
            is_active,
            comments: comments.into(),
        }
    }

    pub fn to_string(&self, is_first: bool) -> String {
        let prefix = if is_first { "?" } else { "&" };
        let comment_hash = if self.is_active { "" } else { "#" };

        return format!("{comment_hash}{prefix}{}={}", self.name, self.value)
    }
}