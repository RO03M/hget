use std::path::Path;

pub fn is_http_file(path: &Path) -> bool {
    match path.file_name().and_then(|name| name.to_str()) {
        Some(filename) => filename.ends_with(".http"),
        None => false,
    }
}
