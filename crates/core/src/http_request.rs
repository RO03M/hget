use std::{str::FromStr};
use anyhow::anyhow;
use reqwest::{Client, Method};
use serde::{Deserialize, Serialize};

use crate::{executor::HttpResponse, header::Header, query_param::QueryParam};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HttpRequest {
    pub name: String,
    pub method: String,
    pub url: String,
    pub params: Vec<QueryParam>,
    pub headers: Vec<Header>,
    pub body: Option<String>,
}

impl HttpRequest {
    pub fn to_string(&self) -> String {
        let mut result = format!("{} {}\n", self.method, self.url);

        let mut is_first = true;
        for param in &self.params {
            if param.comments.len() > 0 {
                for comment in param.comments.split('\n') {
                    result.push_str(&format!("// {}\n", comment));
                }
            }

            result.push_str(&param.to_string(is_first));
            result.push('\n');
            if param.is_active {
                is_first = false;
            }
        }

        for header in &self.headers {
            result.push_str(&format!("{}: {}\n", header.key, header.value));
        }

        if let Some(body) = &self.body {
            result.push('\n');
            result.push_str(body);
        }

        result
    }

    pub async fn run(&self) -> anyhow::Result<HttpResponse> {
        if self.url.is_empty() {
            return Err(anyhow!("Invalid URL"));
        }

        let client = Client::new();
        let method = Method::from_str(&self.method)?;

        let mut builder = client.request(method, &self.url);

        for header in &self.headers {
            builder = builder.header(header.key.clone(), header.value.clone());
        }

        if let Some(body) = &self.body {
            builder = builder.body(body.clone());
        }

        let response = builder.send().await?;

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
            params: vec![
                QueryParam::new("filter", "true", true, "filtering"),
                QueryParam::new("disabled", "true", false, ""),
                QueryParam::new("sort", "DESC", true, ""),
            ],
            headers: vec![Header::new("Content-Type", "application/json", "This is the content type")],
            body: Some(r#"{"name":"Alice"}"#.to_string()),
            ..Default::default()
        };

        let expected = r#"POST https://example.com
// filtering
?filter=true
#&disabled=true
&sort=DESC
// This is the content type
Content-Type: application/json

{"name":"Alice"}
"#.trim();

        println!("{}", req.to_string());
        println!("{expected}");

        assert_eq!(
            req.to_string(),
            expected
        );
    }
}
