use crate::http_request::HttpRequest;

#[derive(PartialEq, Debug)]
enum State {
    Method,
    Headers,
    Body,
}

const HTTP_METHODS: [&str; 7] = ["GET", "POST", "PATCH", "PUT", "DELETE", "HEAD", "OPTIONS"];

pub fn parse(input: &str) -> Vec<HttpRequest> {
    let lines = input.lines();

    let mut requests: Vec<HttpRequest> = vec![];
    let mut current_request: Option<HttpRequest> = None;
    let mut state = State::Method;
    let mut body_lines: Vec<String> = vec![];

    for line in lines {
        let parts: Vec<&str> = line.split_whitespace().collect();

        if parts.is_empty() {
            if state == State::Headers {
                state = State::Body;
            }

            continue;
        }

        if line.starts_with("###") {
            if let Some(mut req) = current_request.take() {
                if !body_lines.is_empty() {
                    req.body = Some(body_lines.join("\n"));
                    body_lines.clear();
                }
                requests.push(req);
            }

            let name = line.trim_start_matches("###").trim().to_string();

            current_request = Some(HttpRequest {
                name,
                method: String::new(),
                url: String::new(),
                headers: vec![],
                body: None,
            });
            state = State::Method;

            continue;
        }

        let is_http_method = HTTP_METHODS.contains(&parts[0]);

        if is_http_method {
            if let Some(req) = current_request.as_mut() {
                req.method = parts[0].to_string();
                req.url = parts.get(1).unwrap_or(&"").to_string();
            } else {
                current_request = Some(HttpRequest {
                    name: String::new(),
                    method: parts[0].to_string(),
                    url: parts.get(1).unwrap_or(&"").to_string(),
                    headers: vec![],
                    body: None,
                });
            }
            state = State::Headers;
        } else if state == State::Headers {
            if let Some(req) = current_request.as_mut() {
                let header: Vec<&str> = line.splitn(2, ":").collect();
                let key = header.get(0).unwrap_or(&"").trim();
                let value = header.get(1).unwrap_or(&"").trim();
                req.headers.push((key.to_string(), value.to_string()));
            }
        } else if state == State::Body {
            body_lines.push(line.to_string());
        }
    }

    if let Some(mut req) = current_request {
        if !body_lines.is_empty() {
            req.body = Some(body_lines.join("\n"));
            body_lines.clear();
        }
        requests.push(req);
    }

    return requests;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_get() {
        let input = "GET https://example.com/users";
        let res = parse(input);
        assert_eq!(res.len(), 1);
        assert_eq!(res[0].method, "GET");
        assert_eq!(res[0].url, "https://example.com/users");
        assert_eq!(res[0].name, "");
        assert!(res[0].headers.is_empty());
        assert!(res[0].body.is_none());
    }

    #[test]
    fn test_named_request() {
        let input = "### Get users\nGET https://example.com/users";
        let res = parse(input);
        assert_eq!(res.len(), 1);
        assert_eq!(res[0].name, "Get users");
        assert_eq!(res[0].method, "GET");
    }

    #[test]
    fn test_headers_parsed() {
        let input = "GET https://example.com/users\nAuthorization: Bearer token123\nAccept: application/json";
        let res = parse(input);
        assert_eq!(res[0].headers.len(), 2);
        assert_eq!(
            res[0].headers[0],
            (
                "Authorization".to_string(),
                " Bearer token123".to_string()
            )
        );
        assert_eq!(
            res[0].headers[1],
            ("Accept".to_string(), " application/json".to_string())
        );
    }

    #[test]
    fn test_body_parsed() {
        let input = "POST https://example.com/users\nContent-Type:
  application/json\n\n{\"name\": \"John\"}";
        let res = parse(input);
        assert_eq!(res[0].method, "POST");
        assert!(res[0].body.is_some());
        assert!(res[0].body.as_ref().unwrap().contains("\"name\": \"John\""));
    }

    #[test]
    fn test_multiline_body() {
        let input = "POST https://example.com/users\n\n{\n  \"name\": \"John\",\n  \"email\": \"john@example.com\"\n}";
        let res = parse(input);
        let body = res[0].body.as_ref().unwrap();
        assert!(body.contains("\"name\": \"John\""));
        assert!(body.contains("\"email\": \"john@example.com\""));
    }

    #[test]
    fn test_multiple_requests() {
        let input = "GET https://example.com/users\n\n###\nPOST https://example.com/users";
        let res = parse(input);
        assert_eq!(res.len(), 2);
        assert_eq!(res[0].method, "GET");
        assert_eq!(res[1].method, "POST");
    }

    #[test]
    fn test_multiple_named_requests() {
        let input = "### List users\nGET https://example.com/users\n\n### Create user\nPOST https://example.com/users\nContent-Type: application/json\n\n{\"name\": \"John\"}";
        let res = parse(input);
        assert_eq!(res.len(), 2);
        assert_eq!(res[0].name, "List users");
        assert_eq!(res[0].method, "GET");
        assert_eq!(res[1].name, "Create user");
        assert_eq!(res[1].method, "POST");
        assert!(res[1].body.is_some());
    }

    #[test]
    fn test_all_methods() {
        let input = "GET https://a.com\n\n### \nPOST https://a.com\n\n### \nPATCH https://a.com\n\n### \nPUT https://a.com\n\n### \nDELETE https://a.com";
        let res = parse(input);
        assert_eq!(res.len(), 5);
        let methods: Vec<&str> = res.iter().map(|r| r.method.as_str()).collect();
        assert_eq!(methods, vec!["GET", "POST", "PATCH", "PUT", "DELETE"]);
    }

    #[test]
    fn test_empty_input() {
        let res = parse("");
        assert!(res.is_empty());
    }

    // File-based tests
    #[test]
    fn test_single_http_parse() {
        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/../../sample/single.http");
        let input = std::fs::read_to_string(path).expect("sample file not found");
        let res = parse(&input);
        assert!(!res.is_empty());
    }

    #[test]
    fn test_single_with_name_http_parse() {
        let path = concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../sample/single-with-name.http"
        );
        let input = std::fs::read_to_string(path).expect("sample file not found");
        let res = parse(&input);
        assert!(!res.is_empty());
        assert!(!res[0].name.is_empty());
    }
}
