use std::path::Path;

use serde::Serialize;

pub fn is_http_file(path: &Path) -> bool {
    match path.file_name().and_then(|name| name.to_str()) {
        Some(filename) => filename.ends_with(".http"),
        None => false,
    }
}

#[derive(Debug, Serialize)]
pub struct FSNode {
    pub name: String,
    pub is_dir: bool,
    pub children: Vec<FSNode>,
}

pub fn list_http_tree(root: &Path) -> Vec<FSNode> {
    build_dir(root)
}

fn build_dir(path: &Path) -> Vec<FSNode> {
    let mut nodes = Vec::new();

    let entries = match std::fs::read_dir(path) {
        Ok(e) => e,
        Err(_) => return nodes,
    };

    for entry in entries {
        let entry = match entry {
            Ok(e) => e,
            Err(_) => continue,
        };

        let path = entry.path();
        let name = entry.file_name().to_string_lossy().to_string();

        if path.is_dir() {
            let children = build_dir(&path);

            nodes.push(FSNode {
                name,
                is_dir: true,
                children,
            });
        } else if crate::helpers::is_http_file(&path) {
            nodes.push(FSNode {
                name,
                is_dir: false,
                children: vec![],
            });
        }
    }

    nodes
}

pub fn slugify(s: &str) -> String {
    s.chars()
        .map(|c| match c {
            'a'..='z' | '0'..='9' => c,
            'A'..='Z' => c.to_ascii_lowercase(),
            ' ' | '-' | '_' => '-',
            'á' | 'à' | 'â' | 'ã' | 'ä' => 'a',
            'é' | 'è' | 'ê' | 'ë' => 'e',
            'í' | 'ì' | 'î' | 'ï' => 'i',
            'ó' | 'ò' | 'ô' | 'õ' | 'ö' => 'o',
            'ú' | 'ù' | 'û' | 'ü' => 'u',
            'ç' => 'c',
            'ñ' => 'n',
            _ => '\0',
        })
        .collect::<String>()
        .split('-')
        .filter(|seg| !seg.is_empty())
        .collect::<Vec<_>>()
        .join("-")
        .trim_matches('-')
        .to_string()
}

pub fn is_line_commented(s: &str) -> bool {
    return s.starts_with("//");
}

pub fn uncomment(line: &str) -> &str {
    return line
        .trim()
        .trim_start_matches("#")
        .trim_start_matches("//")
        .trim_start_matches("/")
        .trim();
}

pub fn is_line_active(line: &str) -> bool {
    return !line.starts_with("#");
}

pub fn strip_slashes(line: &str) -> &str {
    return line
        .trim()
        .trim_start_matches("//")
        .trim_start_matches("/")
        .trim();
}

#[cfg(test)]
mod tests {
    use tempfile::tempdir;

    use super::*;

    #[test]
    fn test_http_tree() {
        let dir = tempdir().unwrap();

        std::fs::create_dir_all(dir.path().join("folder/a")).expect("Failed to create folder/a");

        std::fs::write(dir.path().join("file.http"), "").unwrap();
        std::fs::write(dir.path().join("folder/file.http"), "").unwrap();
        std::fs::write(dir.path().join("folder/a/file.http"), "").unwrap();
        std::fs::write(dir.path().join("folder/a/file"), "").unwrap();
        std::fs::write(dir.path().join("file"), "").unwrap();

        let nodes = list_http_tree(dir.path());

        println!("{:?}", nodes);
    }

    #[test]
    fn test_uncomment() {
        assert_eq!(uncomment("# hello "), "hello");
        assert_eq!(uncomment("#hello"), "hello");
        assert_eq!(uncomment("###hello"), "hello");
        assert_eq!(uncomment("### hello"), "hello");
        assert_eq!(uncomment("###  hello"), "hello");
    }
}
