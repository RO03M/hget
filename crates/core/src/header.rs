use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Header {
    pub key: String,
    pub value: String,
    pub description: String,
    pub is_active: bool,
}

impl Header {
    pub fn new(key: impl Into<String>, value: impl Into<String>, is_active: bool, description: impl Into<String>) -> Self {
        Self {
            key: key.into(),
            value: value.into(),
            is_active: is_active,
            description: description.into(),
        }
    }

    pub fn to_string(&self) -> String {
        if !self.is_active {
            return format!("#{}: {}", self.key, self.value);
        }
        
        format!("{}: {}", self.key, self.value)
    }
}
