use std::str::FromStr;
use reqwest::{Client, Method};
use serde::Deserialize;

use crate::executor::HttpResponse;

#[derive(Debug, Clone, Deserialize)]
pub struct HttpRequest {
    pub name: String,
    pub method: String,
    pub url: String,
    pub headers: Vec<(String, String)>,
    pub body: Option<String>,
}

impl HttpRequest {
    pub fn to_string(&self) -> String {
        let mut result = format!("{} {}\n", self.method, self.url);

        for (key, value) in &self.headers {
            result.push_str(&format!("{}: {}\n", key, value));
        }

        if let Some(body) = &self.body {
            result.push('\n');
            result.push_str(body);
        }

        result
    }

    pub async fn run(&self) -> anyhow::Result<HttpResponse> {
        let client = Client::new();
        let method = Method::from_str(&self.method).unwrap();

        let mut builder = client.request(method, &self.url);

        for (key, value) in &self.headers {
            builder = builder.header(key, value);
        }

        if let Some(body) = &self.body {
            builder = builder.body(body.clone());
        }

        let response = builder.send().await.unwrap();

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
            headers: vec![("Content-Type".to_string(), "application/json".to_string())],
            body: Some(r#"{"name":"Alice"}"#.to_string()),
        };
        assert_eq!(
            req.to_string(),
            "POST https://example.com\nContent-Type: application/json\n\n{\"name\":\"Alice\"}"
        );
    }
}
