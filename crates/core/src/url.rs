use crate::query_params::QueryParams;

#[derive(Debug, Clone)]
pub struct Url {
    host: Option<String>,
    fragment: Option<String>,
    port: Option<u16>,
    protocol: Option<String>,
    pathname: Option<String>,
    search: String,
    query: QueryParams,
    username: Option<String>,
    password: Option<String>
}

impl Url {
    pub fn host(&self) -> Option<String> {
        self.host.clone()
    }

    pub fn fragment(&self) -> Option<String> {
        self.fragment.clone()
    }

    pub fn port(&self) -> Option<u16> {
        self.port
    }

    pub fn href(&self) -> String {
        let mut href = String::new();

        if let Some(protocol) = self.protocol() {
            href.push_str(&protocol);
            href.push_str("://"); // Change this in the future to support more protocols
        }

        if let Some(username) = self.username() && let Some(password) = self.password() {
            href.push_str(&format!("{}:{}@", username, password));
        }

        if let Some(host) = self.host() {
            href.push_str(&host);
        }

        if let Some(port) = self.port() {
            href.push_str(&format!(":{}", port));
        }

        if let Some(pathname) = self.pathname() {
            href.push_str(&pathname);
        }

        let query_string = self.query.to_string();

        if query_string.len() > 0 {
            href.push('?');
            href.push_str(&query_string);
        }

        if let Some(fragment) = self.fragment() {
            href.push_str(&format!("#{}", fragment));
        }
        
        return href;
    }

    pub fn protocol(&self) -> Option<String> {
        self.protocol.clone()
    }

    pub fn pathname(&self) -> Option<String> {
        self.pathname.clone()
    }

    pub fn query(&self) -> QueryParams {
        self.query.clone()
    }

    pub fn search(&self) -> String {
        self.search.clone()
    }
    
    pub fn username(&self) -> Option<String> {
        self.username.clone()
    }
    
    pub fn password(&self) -> Option<String> {
        self.password.clone()
    }
    
    pub fn parse(input: &str) -> Self {
        let (url, fragment) = Self::parse_fragment(input);

        let (host, search) = Self::parse_query(&url);
        let query = QueryParams::parse(&search);

        let (protocol, host) = Self::parse_protocol(&host);
        let (host, username, password) = Self::parse_auth(&host.unwrap_or_default());

        let (host, pathname) = Self::parse_pathname(&host);
        let (host, port) = Self::parse_port(&host.unwrap_or("".to_string()));

        return Self {
            username,
            password,
            pathname,
            protocol,
            host,
            search,
            query,
            fragment,
            port,
        };
    }
    
    fn parse_fragment(input: &str) -> (String, Option<String>) {
        for (i, c) in input.char_indices() {
            if c != '#' {
                continue;
            }

            // Is this '#' the first non-whitespace character of its line?
            let line_start = input[..i].rfind('\n').map(|n| n + 1).unwrap_or(0);
            let before = &input[line_start..i];
            
            if before.trim().is_empty() {
                // disabled line
                continue;
            }

            let after = &input[i + 1..];
            let fragment_end = after.find('\n').unwrap_or(after.len());
            
            return (
                input[..i].to_owned(),
                Some(after[..fragment_end].to_owned()),
            );
        }
    
        (input.to_owned(), None)
    }

    fn parse_query(input: &str) -> (String, String) {
        match input.split_once('?') {
            Some((url, search)) => (url.to_owned(), search.to_owned()),
            None => (input.to_owned(), String::new())
        }
    }
    
    fn parse_protocol(input: &str) -> (Option<String>, Option<String>) {
        match input.split_once("://") {
            Some((protocol, url)) => (Some(protocol.to_string()), Some(url.to_string())),
            None => (Some(input.to_string()), None),
        }
    }

    fn parse_auth(input: &str) -> (String, Option<String>, Option<String>) {
        match input.split_once('@') {
            Some((auth_pair, url)) => {
                let (username, password) = auth_pair.split_once(':').unwrap_or(("", ""));

                return (url.to_owned(), Some(username.to_owned()), Some(password.to_owned()));
            }
            None => (input.to_owned(), None, None)
        }
    }

    // Must not have protocol
    fn parse_port(host: &str) -> (Option<String>, Option<u16>) {
        match host.rsplit_once(':') {
            Some((host, port)) => {
                if let Ok(port) = port.parse() {
                    return (Some(host.to_owned()), Some(port));
                }

                return (Some(host.to_owned()), None);
            }
            None => (Some(host.to_owned()), None),
        }
    }

    fn parse_pathname(url: &str) -> (Option<String>, Option<String>) {
        match url.find('/') {
            Some(index) => (
                Some(url[..index].to_string()),
                Some(url[index..].to_string()),
            ),
            None => (Some(url.to_string()), None),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn full_url() {
        let url = Url::parse("https://example.com:8080/path/to/page?q=1&lang=rust#section");
        assert_eq!(url.protocol().as_deref(), Some("https"));
        assert_eq!(url.host().as_deref(), Some("example.com"));
        assert_eq!(url.port(), Some(8080));
        assert_eq!(url.pathname().as_deref(), Some("/path/to/page"));
        assert_eq!(url.fragment().as_deref(), Some("section"));
        assert_eq!(url.query().get_value("q"), Some("1".to_string()));
        assert_eq!(url.query().get_value("lang"), Some("rust".to_string()));
        assert_eq!(
            url.href(),
            "https://example.com:8080/path/to/page?q=1&lang=rust#section"
        );
    }

    #[test]
    fn multiline_url_with_comments_and_disabled_lines() {
        let url = Url::parse(
            r#"https://john:secret@example.com:8443/api/v1/users?
    page=1
    &limit=20
    // search filters
    &search=john
    &sort=name
    
    // this filter is disabled
    #&role=admin
    
    // enabled filter
    &active=true
    
    // array values
    &tag=rust
    &tag=http
    &tag=parser
    
    // disabled array
    #&tag=disabled
    
    // empty value
    &empty=
    
    // encoded value
    &redirect=https%3A%2F%2Fexample.com%2Fcallback#section
    
    // disabled parameter
    #&debug=true
    "#,
        );
    
        assert_eq!(url.protocol().as_deref(), Some("https"));
        assert_eq!(url.host().as_deref(), Some("example.com"));
        assert_eq!(url.port(), Some(8443));
        assert_eq!(url.username().as_deref(), Some("john"));
        assert_eq!(url.password().as_deref(), Some("secret"));
        assert_eq!(url.pathname().as_deref(), Some("/api/v1/users"));
        assert_eq!(url.fragment().as_deref(), Some("section"));
    
        assert_eq!(url.query().get_value("page"), Some("1".to_string()));
        assert_eq!(url.query().get_value("limit"), Some("20".to_string()));
        assert_eq!(url.query().get_value("search"), Some("john".to_string()));
        assert_eq!(url.query().get_value("sort"), Some("name".to_string()));
        assert_eq!(url.query().get_value("active"), Some("true".to_string()));
        assert_eq!(url.query().get_value("empty"), Some("".to_string()));
        assert_eq!(
            url.query().get_value("redirect"),
            Some("https%3A%2F%2Fexample.com%2Fcallback".to_string())
        );
    
        assert_eq!(url.query().get_value("role"), None);
        assert_eq!(url.query().get_value("debug"), None);
    
        assert_eq!(
            url.query().get_values("tag"),
            vec!["rust", "http", "parser"]
        );
    
        assert_eq!(
            url.href(),
            "https://john:secret@example.com:8443/api/v1/users?page=1&limit=20&search=john&sort=name&active=true&tag=rust&tag=http&tag=parser&empty=&redirect=https%3A%2F%2Fexample.com%2Fcallback#section"
        );
    }
}
