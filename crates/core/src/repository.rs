use std::{
    ffi::OsStr,
    fs,
    path::{Path, PathBuf},
};

use crate::http_file::HttpFile;

static LOOP_LIMIT: u16 = 2048;

#[derive(Debug, Clone)]
pub struct Repository {
    pub dot_path: PathBuf,
}

impl Repository {
    fn find_dot_path(path: &Path) -> Result<PathBuf, String> {
        let mut i: u16 = 0;
        let mut current_path = std::path::absolute(path).map_err(|e| e.to_string())?;

        loop {
            if i == LOOP_LIMIT {
                return Err("Max depth for searching .hget directory reached".into());
            }

            let dot_path = current_path.clone().join(".hget");

            if dot_path.exists() && dot_path.is_dir() {
                return Ok(dot_path);
            }

            if current_path.as_os_str() == "/" {
                return Err("Couldn't find a .hget directory. Initialize it with \"hget init\"".into());
            }

            current_path = current_path.parent().unwrap_or(Path::new("/")).into();
            i += 1;
        }
    }

    fn new(dotpath: &Path) -> Self {
        Repository {
            dot_path: dotpath.into(),
        }
    }

    pub fn open(path: &Path) -> Result<Self, String> {
        let dotpath = Repository::find_dot_path(path)?;

        Ok(Repository::new(&dotpath))
    }

    pub fn init(path: &Path) -> Result<Self, String> {
        let abspath = std::path::absolute(path).map_err(|e| e.to_string())?;
        let dotpath = abspath.clone().join(".hget");

        std::fs::create_dir_all(&dotpath).map_err(|e| e.to_string())?;
        let _ = std::fs::write(
            dotpath.clone().join("description"),
            "Repository without description",
        );
        
        let _ = std::fs::write(
            dotpath.clone().join("variables"),
            "",
        );

        Ok(Repository::new(&dotpath))
    }

    pub fn get_name(&self) -> String {
        let dir_name = self
            .dot_path
            .iter()
            .last()
            .unwrap_or(OsStr::new(""))
            .to_string_lossy()
            .into_owned();

        return dir_name.strip_prefix("/").unwrap_or(&dir_name).to_string();
    }

    pub fn get_http_file(&self, path: &Path) -> Result<HttpFile, String> {
        return HttpFile::from_file(std::path::absolute(".").unwrap().join(&path));
    }

    // acho que collection não faz sentido aqui, pq o repository é a própria collection
    pub fn create_dir(&self, path: PathBuf) -> Result<(), String> {
        let path = path.strip_prefix("/").unwrap();

        if path.is_absolute() {
            return Err("absolute paths are not allowed".into());
        }

        let path = self.dot_path.join(path);

        fs::create_dir_all(path).map_err(|e| e.to_string())?;

        return Ok(());
    }

    pub fn find_http_files(&self) -> Vec<PathBuf> {
        let mut files = Vec::new();
        let mut stack = vec![self.dot_path.clone()];

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

    pub fn save_http_file(&self, http_file: HttpFile, path: PathBuf) -> Result<(), String> {
        let path = path.strip_prefix("/").unwrap_or(&path);
        let path = self.dot_path.join(path);

        fs::create_dir_all(path.parent().unwrap_or(Path::new(""))).map_err(|e| e.to_string())?;

        http_file.save(&path)?;

        Ok(())
    }

    pub fn delete_collection(&self, name: &str) -> Result<(), ()> {
        let path = self.dot_path.join(name);
        if !path.exists() {
            return Err(());
        }

        fs::remove_dir_all(path).unwrap();

        return Ok(());
    }

    pub fn list_collections(&self) -> Result<Vec<String>, ()> {
        let mut collections: Vec<String> = vec![];
        for entry in fs::read_dir(&self.dot_path).unwrap() {
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

// #[cfg(test)]
// mod tests {
//     use tempfile::tempdir;

//     use super::*;

//     fn repo() -> (Repository, tempfile::TempDir) {
//         let dir = tempdir().unwrap();
//         let repo = Repository::new(dir.path().to_path_buf());

//         return (repo, dir);
//     }

//     fn dummy_http_request() -> HttpRequest {
//         HttpRequest {
//             name: "dummy".into(),
//             method: "GET".into(),
//             url: "https://romera.dev".into(),
//             headers: vec![],
//             body: Some("".into()),
//             ..Default::default()
//         }
//     }

//     #[test]
//     fn create_and_list_collection() {
//         let (repo, _dir) = repo();
//         repo.create_dir("auth".into()).unwrap();

//         let collections = repo.list_collections();

//         println!("{:?} {}", collections, repo.root.to_str().unwrap());
//     }

//     #[test]
//     fn create_and_find_http_files() {
//         let (repo, _dir) = repo();

//         let _ = repo.create_dir("auth/custom".into());
//         let _ = repo.create_dir("users".into());
//         let _ = repo.create_dir("features".into());

//         let http_request = dummy_http_request();

//         let _ = repo.save_http_file(&http_request, &".".into());
//         let _ = repo.save_http_file(&http_request, &"auth".into());
//         let _ = repo.save_http_file(&http_request, &"auth/custom".into());
//         let _ = repo.save_http_file(&http_request, &"users".into());
//         let _ = repo.save_http_file(&http_request, &"features".into());
//         let _ = fs::write(_dir.path().join("users/randomfile"), "");

//         let paths = repo.find_http_files();

//         assert_eq!(paths.len(), 5);
//     }
// }
