pub struct QueryParams {
    params: Vec<QueryParams>,
}

impl QueryParams {
    pub fn parse(query: &str) -> Self {
        let query = query
            .lines()
            .flat_map(|line| line.split('&'))
            .map(str::trim)
            .filter(|string| !string.is_empty())
            .map(|pair| {
                println!("{pair:?}");
                let (key, value) = pair.split_once('=').unwrap_or((pair, ""));
                return (key.trim().to_string(), value.trim().to_string());
            });

        Self { params: vec![] }
    }
}
