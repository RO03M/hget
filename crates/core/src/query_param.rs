use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::variable::inject_variables;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QueryParam {
    pub name: String,
    pub value: String,
    pub comments: String,
    pub is_active: bool,
}

pub trait QueryParamVec {
    fn to_tuples(&self) -> Vec<(&str, &str)>;
    fn query_string(&self) -> String;
}

impl QueryParamVec for Vec<QueryParam> {
    fn to_tuples(&self) -> Vec<(&str, &str)> {
        return self
            .iter()
            .filter_map(|qp| {
                if !qp.is_active {
                    return None;
                }
    
                return Some((qp.name.as_str(), qp.value.as_str()));
            })
            .collect();
    }

    fn query_string(&self) -> String {
        let query = self
            .iter()
            .filter_map(|query| {
                if !query.is_active {
                    return None;
                }
    
                return Some(format!("{}={}", query.name, query.value));
            })
            .collect::<Vec<_>>()
            .join("&");

        return query;
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

    pub fn build_variables(&self, variables: &HashMap<String, String>) -> QueryParam {
        let mut output = self.clone();

        output.name = inject_variables(&self.name, variables);
        output.value = inject_variables(&self.value, variables);
        
        return output;
    }
    
    pub fn to_string_with_prefix(&self, is_first: bool) -> String {
        let mut line = format!("{}={}", self.name, self.value);

        if is_first {
            line.insert(0, '?');
        } else {
            line.insert(0, '&');
        }
        
        if !self.is_active {
            line.insert(0, '#');
        }

        return line;
    }
}