use std::{collections::HashMap, str::FromStr};

use serde::{Deserialize, Serialize};

use crate::{helpers, variable::inject_variables};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Header {
    pub name: String,
    pub value: String,
    pub description: String,
    pub is_active: bool,
}

impl FromStr for Header {
    type Err = String;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let is_active = helpers::is_line_active(s);
        let uncommented = helpers::uncomment(s);

        let (key, value) = uncommented
            .split_once(':')
            .ok_or_else(|| "invalid header: missing ':' separator".to_string())?;
    
        Ok(Header {
            name: key.trim().into(),
            value: value.trim().into(),
            description: String::new(),
            is_active,
        })
    }
}

impl Header {
    pub fn new(key: impl Into<String>, value: impl Into<String>, is_active: bool, description: impl Into<String>) -> Self {
        Self {
            name: key.into(),
            value: value.into(),
            is_active: is_active,
            description: description.into(),
        }
    }

    pub fn build_variables(&self, variables: &HashMap<String, String>) -> Header {
        let mut output = self.clone();

        output.name = inject_variables(&self.name, variables);
        output.value = inject_variables(&self.value, variables);
        
        return output;
    }

    pub fn to_string(&self) -> String {
        if !self.is_active {
            return format!("#{}: {}", self.name, self.value);
        }
        
        format!("{}: {}", self.name, self.value)
    }
}
