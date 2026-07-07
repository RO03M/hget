use std::{path::{Path, PathBuf}, str::FromStr};

use crate::{http_request::HttpRequest, parser, variable::Variable};

#[derive(Debug, Clone)]
pub struct HttpFile {
    pub variables: Vec<Variable>,
    pub requests: Vec<HttpRequest>
}

impl FromStr for HttpFile {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(parser::parse(s))
    }
}

impl std::fmt::Display for HttpFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let variables = self.variables.iter().map(|variable| variable.to_string()).collect::<Vec<String>>().join("\n");
        let requests = self.requests.iter().map(|request| request.to_string()).collect::<Vec<String>>().join("\n");
        
        write!(f, "{}\n{}", variables, requests)
    }
}

impl HttpFile {
    pub fn from_file(path: PathBuf) -> Result<Self, String> {
        let content = std::fs::read_to_string(&path).map_err(|e| e.to_string())?;

        content.parse()
    }

    pub fn with_name(&self, name: &str) -> Option<&HttpRequest> {
        return self.requests.iter().find(|request| request.name == name);
    }

    pub fn first(&self) -> Option<&HttpRequest> {
        return self.requests.first();
    }

    pub fn save(&self, path: &Path) -> Result<(), String> {
        std::fs::create_dir_all(path.parent().unwrap_or(Path::new(""))).map_err(|e| e.to_string())?;
        std::fs::write(path, self.to_string()).map_err(|e| e.to_string())?;

        return Ok(())
    }
}
