use std::collections::HashMap;

pub struct Url {
    host: Option<String>,
    fragment: Option<String>,
    port: Option<u16>,
    href: String,
    protocol: Option<String>,
    pathname: Option<String>,
    query: HashMap<String, String>,
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
        self.href.clone()
    }

    pub fn protocol(&self) -> Option<String> {
        self.protocol.clone()
    }

    pub fn pathname(&self) -> Option<String> {
        self.pathname.clone()
    }

    pub fn query(&self) -> HashMap<String, String> {
        self.query.clone()
    }

    pub fn parse(input: &str) -> Self {
        let href = input.to_string();
        let (url, fragment) = Self::parse_fragment(input);
        let url = url.unwrap();

        let (host, query) = url.split_once('?').unwrap_or((&url, ""));

        let (protocol, host) = Self::parse_protocol(host);
        let (host, pathname) = Self::parse_pathname(&host.unwrap_or_default());
        let (host, port) = Self::parse_port(&host.unwrap_or("".to_string()));

        let query: HashMap<String, String> = query
            .lines()
            .flat_map(|line| line.split('&'))
            .map(str::trim)
            .filter(|string| !string.is_empty())
            .map(|pair| {
                let (key, value) = pair.split_once('=').unwrap_or((pair, ""));
                return (key.trim().to_string(), value.trim().to_string());
            })
            .collect();

        return Self {
            pathname,
            protocol,
            href,
            host,
            query,
            fragment,
            port,
        };
    }

    fn parse_fragment(input: &str) -> (Option<String>, Option<String>) {
        match input.split_once('#') {
            Some((url, fragment)) => (Some(url.to_owned()), Some(fragment.to_string())),
            None => (Some(input.to_owned()), None),
        }
    }
    fn parse_protocol(input: &str) -> (Option<String>, Option<String>) {
        match input.split_once("://") {
            Some((protocol, url)) => (Some(protocol.to_string()), Some(url.to_string())),
            None => (Some(input.to_string()), None),
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
        assert_eq!(url.query().get("q"), Some(&"1".to_string()));
        assert_eq!(url.query().get("lang"), Some(&"rust".to_string()));
        assert_eq!(
            url.href(),
            "https://example.com:8080/path/to/page?q=1&lang=rust#section"
        );
    }
}
