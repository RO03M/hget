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
    name: String,
    is_dir: bool,
    children: Vec<FSNode>,
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
}
