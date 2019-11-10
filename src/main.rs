use std::error::Error;
use std::path::PathBuf;

mod settings;

fn main() {
    let settings = settings::Settings::load();

    if !settings.source_dir.is_dir() {
        panic!("Source dir exists and is not a file, exiting.");
    }

    create_dir(&settings.source_dir);
    create_dir(&settings.destination_dir);

    let files = get_files(&settings.source_dir);

    for file in &files {
        // TODO: Replace tree_magic
        let mime_type = tree_magic::from_filepath(&file);
        for pattern in &settings.sort_patterns {
            if pattern.mime_type.contains(&mime_type) {
                let destination_dir = &settings.destination_dir.join(&pattern.destination_subdir);

                let destination_file = &destination_dir.join(&file.file_name().unwrap());

                create_dir(&destination_dir);
                match std::fs::rename(&file, &destination_file) {
                    Ok(_o) => println!(
                        "Successfully moved {} to {}",
                        &file.display(),
                        &destination_dir.display()
                    ),
                    Err(e) => panic!("Error {}", e),
                }

                break;
            }
        }
    }
}

fn create_dir(path: &PathBuf) {
    if !path.exists() {
        match std::fs::create_dir_all(&path) {
            Ok(()) => println!("{} dir created successfully!", &path.display()),
            Err(e) => panic!("Error {}", e.description()),
        }
    } else {
        if path.is_dir() {
            // Already exists
        } else {
            panic!(
                "{} already exists and is not a directory! Remove them manually or change path.",
                &path.display()
            );
        }
    }
}

fn get_files(path: &PathBuf) -> Vec<PathBuf> {
    let mut files: Vec<PathBuf> = Vec::new();

    for entry in std::fs::read_dir(path).unwrap() {
        let entry = entry.unwrap();
        if entry.path().is_file() {
            files.push(entry.path().to_path_buf())
        }
    }

    files
}
