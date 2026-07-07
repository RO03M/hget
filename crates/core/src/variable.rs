use std::{collections::HashMap, fmt::Display, str::FromStr};

use serde::{Deserialize, Serialize};

use crate::helpers;

// A variable in .http files are defined like this: @varname=value_of_var

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Variable {
    pub key: String,
    pub value: String,
    pub comments: String,
    pub is_active: bool,
}

pub fn inject_variables(input: &str, variables: &HashMap<String, String>) -> String {
    let mut output = input.to_owned();

    for (key, value) in variables {
        let placeholder = format!("{{{{{}}}}}", key);

        output = output.replace(&placeholder, &value);
    }
    
    return output;
}

pub fn variables_to_map(variables: &Vec<Variable>) -> HashMap<String, String> {
    
}

impl Display for Variable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut line = format!("@{}={}", self.key, self.value);

        if !self.is_active {
            line.insert_str(0, "# ");
        }
        
        write!(
            f,
            "{}",
            line
        )
    }
}

impl FromStr for Variable {
    type Err = String;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let is_active = helpers::is_line_active(s);
        let uncommented = helpers::uncomment(s);
        
        if !uncommented.starts_with("@") {
            return Err("string doesn't start with @".into());
        }

        let (key, value) = uncommented.trim_start_matches("@")
            .split_once("=")
            .ok_or("invalid variable definition")?;

        return Ok(Variable {
            comments: "".into(),
            is_active: is_active,
            key: key.trim().into(),
            value: value.trim().into()
        });
    }
}

impl Variable {
    pub fn new(name: impl Into<String>, value: impl Into<String>, is_active: bool, comments: impl Into<String>) -> Self {
        Self {
            key: name.into().trim().into(),
            value: value.into().trim().into(),
            is_active,
            comments: comments.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn parsing() {
        assert_eq!("# @name=teste".parse::<Variable>().unwrap(), Variable {
            comments: "".into(),
            is_active: false,
            key: "name".into(),
            value: "teste".into()
        });
        
        assert_eq!("# @name =teste".parse::<Variable>().unwrap(), Variable {
            comments: "".into(),
            is_active: false,
            key: "name".into(),
            value: "teste".into()
        });
        
        assert_eq!("#@name=  teste".parse::<Variable>().unwrap(), Variable {
            comments: "".into(),
            is_active: false,
            key: "name".into(),
            value: "teste".into()
        });
        
        assert_eq!("@name =  teste".parse::<Variable>().unwrap(), Variable {
            comments: "".into(),
            is_active: true,
            key: "name".into(),
            value: "teste".into()
        });
    }

    #[test]
    fn parsing_trims_key_and_value() {
        let var: Variable = "@  foo   =   bar baz   ".parse().unwrap();

        assert_eq!(var.key, "foo");
        assert_eq!(var.value, "bar baz");
    }

    #[test]
    fn parsing_value_may_contain_equals() {
        let var: Variable = "@token=abc=123==xyz".parse().unwrap();

        assert_eq!(var.key, "token");
        assert_eq!(var.value, "abc=123==xyz");
    }

    #[test]
    fn parsing_rejects_missing_at() {
        assert!("name=value".parse::<Variable>().is_err());
    }

    #[test]
    fn new_trims_inputs() {
        let var = Variable::new("  key  ", "  value  ", true, "");

        assert_eq!(var.key, "key");
        assert_eq!(var.value, "value");
    }

    #[test]
    fn to_string_active() {
        let var = Variable::new("host", "localhost", true, "");

        assert_eq!(var.to_string(), "@host=localhost");
    }

    #[test]
    fn to_string_inactive() {
        let var = Variable::new("host", "localhost", false, "");

        assert_eq!(var.to_string(), "# @host=localhost");
    }

    #[test]
    fn round_trip_active() {
        let original = Variable::new("host", "localhost", true, "");

        println!("{original}");
        let parsed: Variable = original.to_string().parse().unwrap();

        assert_eq!(parsed, original);
    }

    #[test]
    fn round_trip_inactive() {
        let original = Variable::new("host", "localhost", false, "");

        let parsed: Variable = original.to_string().parse().unwrap();

        assert_eq!(parsed, original);
    }
}