// Executes parsed HTTP requests via reqwest.

use std::str::FromStr;

use reqwest::{Client, Method};

use crate::http_request::HttpRequest;

#[derive(Debug)]
pub struct HttpResponse {
    pub status: u16,
    pub headers: Vec<(String, String)>,
    pub body: String,
}

pub async fn execute(request: &HttpRequest) -> anyhow::Result<HttpResponse> {
    let client = Client::new();
    let method = Method::from_str(&request.method).unwrap();

    let mut builder = client.request(method, &request.url);

    for (key, value) in &request.headers {
        builder = builder.header(key, value);
    }

    if let Some(body) = &request.body {
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

    return Ok(HttpResponse { status: status, headers: headers, body: body })
}
