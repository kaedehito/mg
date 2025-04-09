use crate::paths;
use std::fs;

pub fn get_ignore_files() -> Vec<String> {
    let mgignore_path = paths::find_mgignore();
    let mut ignore_path: Vec<String> = Vec::new();

    if let Some(mgignore_path) = mgignore_path {
        let mgignore = fs::read_to_string(mgignore_path).unwrap();
        let mut lines = mgignore.lines();
        while let Some(line) = lines.next() {
            if line.starts_with("#") || line.is_empty() {
                continue;
            }
            ignore_path.push(line.to_string());
        }
    }
    ignore_path
}
