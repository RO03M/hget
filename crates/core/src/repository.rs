use std::{ffi::OsStr, fs, path::{Path, PathBuf}};

use crate::{http_request::HttpRequest, parser::parse};

pub struct Repository {
    pub root: PathBuf,
}

impl Repository {
    pub fn new(root: PathBuf) -> Self {
        return Self { root };
    }

    pub fn get_name(&self) -> String {
        let dir_name = self.root.iter().last().unwrap_or(OsStr::new("")).to_string_lossy().into_owned();

        return dir_name.strip_prefix("/").unwrap_or(&dir_name).to_string();
    }

    pub fn get_http_file(&self, path: &Path) -> Result<(HttpRequest, String), String> {
        let file = std::fs::read_to_string(self.root.join(path)).map_err(|e| e.to_string())?;

        let http_request = parse(&file).get(0).expect("Failed to parse http request").clone();

        return Ok((http_request, file));
    }

    // acho que collection não faz sentido aqui, pq o repository é a própria collection
    pub fn create_collection(&self, name: &str) -> Result<(), ()> {
        let _ = fs::create_dir_all(self.root.join(name));

        return Ok(());
    }

    pub fn find_http_files(&self) -> Vec<PathBuf> {
        let mut files = Vec::new();
        let mut stack = vec![self.root.clone()];
        
        while let Some(dir) = stack.pop() {
            let entries = fs::read_dir(dir).unwrap();
            
            for entry in entries {
                let entry = match entry {
                    Ok(e) => e,
                    Err(_) => continue,
                };
    
                let path = entry.path();
    
                if path.is_dir() {
                    stack.push(path);
                } else if crate::helpers::is_http_file(&path) {
                    files.push(path);
                }
            }
        }

        return files;
    }

    pub fn create_folder(&self, path: PathBuf) -> anyhow::Result<()> {
        let _ = fs::create_dir_all(self.root.join(path));

        Ok(())
    }

    pub fn save_http_file(&self, http_request: &HttpRequest, path: &PathBuf) -> anyhow::Result<()> {
        let path = path.strip_prefix("/").unwrap_or(&path);
        let path = self.root.join(path);
        println!("{} {}", path.to_str().unwrap(), self.root.to_str().unwrap());
        fs::create_dir_all(path.parent().unwrap_or(Path::new("")))?;


        let _ = fs::write(path, http_request.to_string())?;

        Ok(())
    }

    pub fn delete_collection(&self, name: &str) -> Result<(), ()> {
        let path = self.root.join(name);
        if !path.exists() {
            return Err(());
        }

        fs::remove_dir_all(path).unwrap();

        return Ok(());
    }

    pub fn list_collections(&self) -> Result<Vec<String>, ()> {
        let mut collections: Vec<String> = vec![];
        for entry in fs::read_dir(&self.root).unwrap() {
            let entry = entry.unwrap();

            if entry.file_type().unwrap().is_dir() {
                if let Some(name) = entry.file_name().to_str() {
                    collections.push(name.to_string());
                }
            }
        }

        return Ok(collections);
    }
}

#[cfg(test)]
mod tests {
    use tempfile::tempdir;

    use super::*;

    fn repo() -> (Repository, tempfile::TempDir) {
        let dir = tempdir().unwrap();
        let repo = Repository::new(dir.path().to_path_buf());

        return (repo, dir);
    }

    fn dummy_http_request() -> HttpRequest {
        HttpRequest {
            name: "dummy".into(),
            method: "GET".into(),
            url: "https://romera.dev".into(),
            headers: vec![],
            body: Some("".into()),
        }
    }

    #[test]
    fn create_and_list_collection() {
        let (repo, _dir) = repo();
        repo.create_collection("auth").unwrap();

        let collections = repo.list_collections();

        println!("{:?} {}", collections, repo.root.to_str().unwrap());
    }

    #[test]
    fn create_and_find_http_files() {
        let (repo, _dir) = repo();

        let _ = repo.create_folder("auth/custom".into());
        let _ = repo.create_folder("users".into());
        let _ = repo.create_folder("features".into());

        let http_request = dummy_http_request();

        let _ = repo.save_http_file(&http_request, &".".into());
        let _ = repo.save_http_file(&http_request, &"auth".into());
        let _ = repo.save_http_file(&http_request, &"auth/custom".into());
        let _ = repo.save_http_file(&http_request, &"users".into());
        let _ = repo.save_http_file(&http_request, &"features".into());
        let _ = fs::write(_dir.path().join("users/randomfile"), "");

        let paths = repo.find_http_files();

        assert_eq!(paths.len(), 5);
    }
}
