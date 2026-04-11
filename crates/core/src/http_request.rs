#[derive(Debug, Clone)]
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
