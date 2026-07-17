use crate::{line::parse_line, query_param::QueryParam};

#[derive(Debug, Clone, PartialEq)]
pub struct QueryParams {
    params: Vec<QueryParam>,
}

impl QueryParams {
    pub fn parse(query: &str) -> Self {
        let mut params: Vec<QueryParam> = vec![];
        let mut comments = String::new();
        
        for line in query.lines() {
            let parsed_line = parse_line(line);
            if parsed_line.is_user_comment {
                if !comments.is_empty() {
                    comments.push('\n');
                }
                comments.push_str(&parsed_line.content);
                continue;
            }
            
            for part in parsed_line.content.split("&") {
                if part.is_empty() {
                    continue;
                }

                let (name, value) = part.split_once("=").unwrap_or(("", ""));

                params.push(QueryParam {
                    name: name.to_owned(),
                    value: value.to_owned(),
                    comments: comments.clone(),
                    is_active: parsed_line.is_enabled
                });
            }
            
            comments = String::new();
        }
        
        Self { params: params }
    }

    pub fn get_any(&self, key: &str) -> Option<QueryParam> {
        return self.params.iter().find(|query| query.name == key).cloned();
    }
    
    pub fn get(&self, key: &str) -> Option<QueryParam> {
        return self.params.iter().find(|query| query.name == key && query.is_active).cloned();
    }
    
    pub fn get_value(&self, key: &str) -> Option<String> {
        if let Some(query) = self.get(key) {
            Some(query.value)
        } else {
            None
        }
    }
    
    pub fn get_value_any(&self, key: &str) -> Option<String> {
        if let Some(query) = self.get_any(key) {
            Some(query.value)
        } else {
            None
        }
    }

    pub fn get_all(&self, key: &str) -> Vec<QueryParam> {
        return self.params.iter().filter(|query| query.name == key && query.is_active).cloned().collect();
    }
    
    pub fn get_values(&self, key: &str) -> Vec<String> {
        let queries = self.get_all(key);

        return queries.iter().map(|query| query.value.clone()).collect();
    }

    pub fn to_string(&self) -> String {
        let active_queries = self.params.iter().filter(|query| query.is_active).cloned().collect::<Vec<QueryParam>>();

        if active_queries.len() == 0 {
            return String::new();
        }

        return active_queries.iter().map(|query| format!("{}={}", query.name, query.value)).collect::<Vec<_>>().join("&");
    }
}

#[cfg(test)]
mod test {
    use crate::{query_param::QueryParam, query_params::QueryParams};

    #[test]
    fn complex_query() {
        let query = r#"
test=a&foo=true
// this is bar
&bar=false
//disabled it
#&disabled=true
//this is an array of numbers
&array=1&array=2
"#;
        let params = QueryParams::parse(query);

        assert_eq!(
            params,
            QueryParams {
                params: vec![
                    QueryParam::new("test", "a", true, ""),
                    QueryParam::new("foo", "true", true, ""),
                    QueryParam::new("bar", "false", true, "this is bar"),
                    QueryParam::new("disabled", "true", false, "disabled it"),
                    QueryParam::new("array", "1", true, "this is an array of numbers"),
                    QueryParam::new("array", "2", true, "this is an array of numbers"),
                ]
            }
        )
    }
    
    #[test]
    fn single_pair() {
        let query = QueryParams::parse("foo=true\n&bar=false");
        assert_eq!(query, QueryParams { params: vec![QueryParam::new("foo", "true", true, ""), QueryParam::new("bar", "false", true, "")] })
    }
}
