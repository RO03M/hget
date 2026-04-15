use std::{fs, path::PathBuf};

pub struct Repository {
    pub root: PathBuf,
}

impl Repository {
    pub fn new(root: PathBuf) -> Self {
        return Self {
            root
        };
    }

    pub fn create_collection(&self, name: &str) -> Result<(), ()> {
        let _ = fs::create_dir_all(self.root.join(name));

        return Ok(());
    }

    pub fn delete_collection(&self, name: &str) -> Result<(), ()> {
        let path = self.root.join(name);
        if !path.exists() {
            return Err(())
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

    #[test]
    fn create_and_list_collection() {
        let (repo, _dir) = repo();
        repo.create_collection("auth").unwrap();

        let collections = repo.list_collections();

        println!("{:?} {}", collections, repo.root.to_str().unwrap());
    }
}