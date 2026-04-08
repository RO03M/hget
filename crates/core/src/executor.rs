// Executes parsed HTTP requests via reqwest.

use crate::parser::HttpRequest;

#[derive(Debug)]
pub struct HttpResponse {
    pub status: u16,
    pub headers: Vec<(String, String)>,
    pub body: String,
}

pub async fn execute(_request: &HttpRequest) -> anyhow::Result<HttpResponse> {
    todo!("execute request and return response")
}
