use std::error::Error;
use std::path::PathBuf;

pub fn create_dir(path: &PathBuf) {
    if !path.exists() {
        match std::fs::create_dir_all(&path) {
            Ok(()) => println!("{} dir created successfully!", &path.display()),
            Err(e) => panic!("Error {}", e.description()),
        }
    } else if !path.is_dir() {
        panic!(
            "{} already exists and is not a directory, exiting.",
            &path.display()
        );
    }
}

pub fn get_files(path: &PathBuf) -> Vec<PathBuf> {
    let mut files: Vec<PathBuf> = Vec::new();

    for entry in std::fs::read_dir(path).unwrap() {
        let entry = entry.unwrap();
        if entry.path().is_file() {
            files.push(entry.path().to_path_buf())
        }
    }

    files
}
