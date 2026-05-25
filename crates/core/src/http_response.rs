use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct HttpResponse {
    pub status: u16,
    pub headers: Vec<(String, String)>,
    pub body: String,
}
