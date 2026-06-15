use crate::{header::Header, http_request::HttpRequest, query_param::QueryParam};

#[derive(PartialEq, Debug)]
enum State {
    Method,
    Headers,
    Body,
}

fn uncomment(line: impl Into<String>) -> String {
    let line = line.into();

    return line
        .strip_prefix("##")
        .or_else(|| line.strip_prefix("#"))
        .unwrap_or(&line)
        .to_string();
}

fn is_commented(line: impl Into<String>) -> bool {
    let line = line.into();

    return line.starts_with("#") || line.starts_with("##");
}

fn is_query_param_line(line: impl Into<String>) -> bool {
    let line = line.into();
    let uncommented = uncomment(line);

    return uncommented.starts_with("?") || uncommented.starts_with("&")
}

pub(crate) fn resolve_params(raw: impl Into<String>, comment: String) -> Vec<QueryParam> {
    let mut params: Vec<QueryParam> = Vec::new();
    let raw = raw.into();
    let parts: Vec<&str> = raw.split("&").collect();

    let is_commented = is_commented(raw.clone());
    
    for part in parts {
        let part = uncomment(part);
        let (key, value) = part
            .strip_prefix("?")
            .unwrap_or(&part)
            .split_once("=")
            .unwrap_or(("", ""));

        if key == "" {
            continue;
        }


        params.push(QueryParam::new(key, value, !is_commented, comment.clone()));
    }

    return params;
}

pub(crate) fn split_params_from_url(url: impl Into<String>) -> (String, Vec<QueryParam>) {
    let url = url.into();
    let mut parts = url.splitn(2, "?");

    let url = parts.next().unwrap();
    let params = parts.next().unwrap_or("");
    let params = resolve_params(params, "".into());

    return (url.into(), params);
}

pub fn parse(input: &str) -> Vec<HttpRequest> {
    let lines = input.trim().lines();

    let mut requests: Vec<HttpRequest> = vec![];
    let mut current_request: Option<HttpRequest> = None;
    let mut state = State::Method;
    let mut body_lines: Vec<String> = vec![];

    let mut comments = String::new();

    for line in lines {
        let line = line.trim();
        
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.is_empty() {
            if state == State::Headers {
                state = State::Body;
            }

            comments = String::new();

            continue;
        }

        if line.starts_with("//") {
            if comments.len() != 0 {
                comments.push('\n');
            }
            comments.push_str(line.strip_prefix("//").unwrap_or("").trim());

            continue;
        }

        // Request with a name
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
                params: vec![],
                headers: vec![],
                body: None,
            });
            // Resets for the next possible request
            state = State::Method;

            continue;
        }

        if state == State::Method {
            let method = parts[0].to_string();
            let raw_url = parts.get(1).unwrap_or(&"").to_string();
            let (url, params) = split_params_from_url(raw_url);

            if let Some(req) = current_request.as_mut() {
                req.method = method;
                req.url = url;
                req.params = params;
            } else {
                current_request = Some(HttpRequest {
                    name: String::new(),
                    method: method,
                    url: url,
                    headers: vec![],
                    params: params,
                    body: None,
                });
            }

            comments = String::new();

            state = State::Headers;
        } else if state == State::Headers {
            if let Some(req) = current_request.as_mut() {
                if is_query_param_line(line) {
                    let params = resolve_params(line, comments);
                    comments = String::new();

                    req.params.extend(params);
                } else {
                    let header: Vec<&str> = line.splitn(2, ":").collect();
                    let key = header.get(0).unwrap_or(&"").trim();
                    let value = header.get(1).unwrap_or(&"").trim();
                    req.headers.push(Header::new(key, value, true, comments));
                    comments = String::new();
                }
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
    fn multiline_query() {
        let raw = r#"POST https://httpbin.org/post?filter=true&leticia=linda
//Sorting desc now
&sort=DESC
#?filter=false
// This is the name of the person
// And of course, this is a comment
&name=John
#?commented=true
#&rock=and-stone
Content-Type: application/json;charset=utf8
Authorization: Bearer mytoken123
Accept: application/json
X-Content: {{var}}

{
    "name": "John Doe",
    "email": "john@example.com"
}"#;

        let http_requests = parse(raw);

        assert_eq!(
            http_requests.get(0).unwrap().params,
            vec![
                QueryParam::new("filter", "true", true, ""),
                QueryParam::new("leticia", "linda", true, ""),
                QueryParam::new("sort", "DESC", true, "Sorting desc now"),
                QueryParam::new("filter", "false", false, ""),
                QueryParam::new("name", "John", true, "This is the name of the person\nAnd of course, this is a comment"),
                QueryParam::new("commented", "true", false, ""),
                QueryParam::new("rock", "and-stone", false, ""),
            ]
        );

        assert_eq!(
            http_requests.get(0).unwrap().headers,
            vec![
                Header::new("Content-Type", "application/json;charset=utf8", true, ""),
                Header::new("Authorization", "Bearer mytoken123", true, ""),
                Header::new("Accept", "application/json", true, ""),
                Header::new("X-Content", "{{var}}", true, ""),
            ]
        );

        assert_eq!(
            http_requests.get(0).unwrap().body.clone().unwrap_or("".to_string()),
            "{\n\"name\": \"John Doe\",\n\"email\": \"john@example.com\"\n}"
        );
    }

    #[test]
    fn headers_description() {
        let raw = r#"POST https://httpbin.org/post
// This is json
Content-Type: application/json;charset=utf8
// This is the token
// we should hide it
Authorization: Bearer mytoken123
// Custom header
X-Custom: {{CUSTOM_KEY}}
"#;
        let http_requests = parse(raw);

        assert_eq!(
            http_requests.get(0).unwrap().headers,
            vec![
                Header::new("Content-Type", "application/json;charset=utf8", true, "This is json"),
                Header::new("Authorization", "Bearer mytoken123", true, "This is the token\nwe should hide it"),
                Header::new("X-Custom", "{{CUSTOM_KEY}}", true, "Custom header"),
            ]
        );
    }

    #[test]
    fn parse_url() {
        let res = split_params_from_url("http://localhost/users?name=John&surname=Devo");
        assert_eq!(
            res,
            (
                "http://localhost/users".into(),
                vec![
                    QueryParam::new("name", "John", true, ""),
                    QueryParam::new("surname", "Devo", true, ""),
                ]
            )
        );

        let res = split_params_from_url("http://localhost/users?name=John");
        assert_eq!(
            res,
            (
                "http://localhost/users".into(),
                vec![QueryParam::new("name", "John", true, ""),]
            )
        );

        let res = split_params_from_url("http://localhost/users");
        assert_eq!(res, ("http://localhost/users".into(), vec![]));
    }

    #[test]
    fn handle_invalid_params() {
        let res = resolve_params("Content-Type: application/json", "".into());
        assert_eq!(res, vec![]);
        let res = resolve_params("{
            \"name\": \"John Doe\",
            \"email\": \"john@example.com\"
        }", "".into());
        assert_eq!(res, vec![]);
    }

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
            Header::new("Authorization", "Bearer token123", true, "")
        );
        assert_eq!(
            res[0].headers[1],
            Header::new("Accept", "application/json", true, "")
        );
    }

    #[test]
    fn test_body_parsed() {
        let input = "POST https://example.com/users\nContent-Type: application/json\n\n{\"name\": \"John\"}";
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
