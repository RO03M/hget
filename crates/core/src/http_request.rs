use reqwest::{Client, Method};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, str::FromStr};

use crate::{
    executor::HttpResponse,
    header::Header,
    query_param::{QueryParam, QueryParamVec},
    variable::{Variable, inject_variables, resolve_variables},
};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HttpRequest {
    pub name: String,
    pub method: String,
    pub url: String,
    pub queries: Vec<QueryParam>,
    pub headers: Vec<Header>,
    pub body: Option<String>,
}

impl HttpRequest {
    fn format_comments(&self, comments: impl Into<String>) -> String {
        let comments = comments.into();
        if comments.len() == 0 {
            return String::new();
        }

        let mut result = String::new();
        for comment in comments.split('\n') {
            result.push_str(&format!("// {}\n", comment));
        }
        result
    }

    pub fn active_headers(&self) -> Vec<Header> {
        return self
            .headers
            .iter()
            .filter(|header| header.is_active)
            .cloned()
            .collect();
    }

    pub fn normalize_url_queries(&mut self) {
        // let (url, query) = self.url.split('?').collect();
    }

    pub fn to_string(&self) -> String {
        let mut result = format!("{} {}\n", self.method, self.url);

        let mut is_first = true;
        for query in self.queries.clone() {
            result.push_str(&self.format_comments(query.comments.clone()));

            result.push_str(&query.to_string_with_prefix(is_first));
            result.push('\n');

            if query.is_active {
                is_first = false;
            }
        }

        for header in &self.headers {
            result.push_str(&self.format_comments(header.description.clone()));
            result.push_str(&header.to_string());
            result.push('\n');
        }

        if let Some(body) = &self.body {
            result.push('\n');
            result.push_str(body);
        }

        result
    }

    pub fn full_url(&self) -> String {
        let formatted_queries = self.queries.query_string();

        return format!("{}?{}", self.url, formatted_queries);
    }

    pub fn injected(&self, variables: HashMap<String, String>) -> HttpRequest {
        let resolved_variables = resolve_variables(variables);
        let mut request = self.clone();

        request.url = inject_variables(&self.url, &resolved_variables);
        request.queries = self
            .queries
            .iter()
            .map(|param| param.build_variables(&resolved_variables))
            .collect();
        request.headers = self
            .headers
            .iter()
            .map(|header| header.build_variables(&resolved_variables))
            .collect();
        request.body = if let Some(body) = self.body.clone() {
            Some(inject_variables(&body, &resolved_variables))
        } else {
            None
        };

        return request;
    }

    pub async fn run(&self, variables: HashMap<String, String>) -> Result<HttpResponse, String> {
        let resolved_request = self.injected(variables);

        if resolved_request.url.is_empty() {
            return Err("url is empty".into());
        }

        let client = Client::new();
        let method = Method::from_str(&resolved_request.method).map_err(|e| e.to_string())?;

        let mut builder = client
            .request(method, &resolved_request.url)
            .query(&resolved_request.queries.to_tuples());

        for header in resolved_request.active_headers() {
            builder = builder.header(header.name.clone(), header.value.clone());
        }

        if let Some(body) = &resolved_request.body {
            builder = builder.body(body.clone());
        }

        let response = builder.send().await.map_err(|e| e.to_string())?;

        let status = response.status().as_u16();
        let headers: Vec<(String, String)> = response
            .headers()
            .iter()
            .map(|(key, value)| (key.to_string(), value.to_str().unwrap_or("").to_string()))
            .collect();
        let body = response.text().await.unwrap_or("".to_string());

        return Ok(HttpResponse {
            status: status,
            headers: headers,
            body: body,
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_string_with_body() {
        let req = HttpRequest {
            name: "test".to_string(),
            method: "POST".to_string(),
            url: "https://example.com".to_string(),
            queries: vec![
                QueryParam::new("filter", "true", true, "filtering"),
                QueryParam::new("disabled", "true", false, ""),
                QueryParam::new("sort", "DESC", true, ""),
            ],
            headers: vec![
                Header::new(
                    "Content-Type",
                    "application/json",
                    true,
                    "This is the content type\nWith another line",
                ),
                Header::new(
                    "Accept",
                    "application/json",
                    false,
                    "This is a disabled header",
                ),
            ],
            body: Some(r#"{"name":"Alice"}"#.to_string()),
            ..Default::default()
        };

        let expected = r#"POST https://example.com
// filtering
?filter=true
#&disabled=true
&sort=DESC
// This is the content type
// With another line
Content-Type: application/json
// This is a disabled header
#Accept: application/json

{"name":"Alice"}
"#
        .trim();

        println!("got: {}", req.to_string());
        println!("expected: {expected}");

        assert_eq!(req.to_string(), expected);
    }

    #[test]
    fn variables_injection() {
        let variables = HashMap::from([
            (
                "URL".to_string(),
                "https://example.com?key=hidden".to_string(),
            ),
            ("mode".to_string(), "{{ENV}}".to_string()),
            ("ENV".to_string(), "PROD".to_string()),
        ]);

        let req = HttpRequest {
            name: "test".to_string(),
            method: "POST".to_string(),
            url: "{{URL}}".to_string(),
            queries: vec![
                QueryParam::new("filter", "true", true, "filtering"),
                QueryParam::new("{{queryname}}", "true", false, ""),
                QueryParam::new("mode", "{{mode}}", true, ""),
            ],
            headers: vec![
                Header::new(
                    "Content-Type",
                    "application/json",
                    true,
                    "This is the content type\nWith another line",
                ),
                Header::new(
                    "Accept",
                    "application/json",
                    false,
                    "This is a disabled header",
                ),
            ],
            body: Some(r#"{"name":"Alice"}"#.to_string()),
            ..Default::default()
        };

        assert_eq!(
            req.injected(variables.clone()).queries.get(2),
            Some(&QueryParam::new("mode", "PROD", true, ""))
        );
        assert_eq!(req.injected(variables.clone()).url, "https://example.com");
    }
}
