use std::error::Error;
use std::path::PathBuf;

pub struct SortPattern<'a> {
    pub mime_type: &'a Vec<String>,
    pub destination_subdir: &'a str,
}

pub struct Settings<'a> {
    pub source_dir: &'a PathBuf,
    pub destination_dir: &'a PathBuf,
    pub sort_patterns: &'a Vec<SortPattern<'a>>,
}

fn main() {
    let torrents_pattern = SortPattern {
        mime_type: &vec!["application/x-bittorrent".to_string()],
        destination_subdir: "torrents",
    };

    let images_pattern = SortPattern {
        mime_type: &vec!["image/png".to_string()],
        destination_subdir: "images",
    };

    let settings = Settings {
        source_dir: &PathBuf::from("/home/elxreno/testing/source"),
        destination_dir: &PathBuf::from("/home/elxreno/testing/destination"),
        sort_patterns: &vec![torrents_pattern, images_pattern],
    };

    if !settings.source_dir.is_dir() {
        panic!("Source dir exists and is not a file, exiting.");
    }

    create_dir(settings.source_dir);
    create_dir(settings.destination_dir);

    let files = get_files(settings.source_dir);

    for sort_pattern in settings.sort_patterns {
        let destination_dir = settings
            .destination_dir
            .join(sort_pattern.destination_subdir);

        create_dir(&destination_dir);

        for file in &files {
            let mime_type = tree_magic::from_filepath(file);
            if sort_pattern.mime_type.contains(&mime_type) {
                match std::fs::copy(file, &destination_dir.join(file.file_name().unwrap())) {
                    Ok(_o) => println!("Successfully copied {} to {}", file.display(), destination_dir.display()),
                    Err(e) => panic!("Error {}", e)
                }
            }
        }
    }
}

fn create_dir(path: &PathBuf) {
    if !path.exists() {
        match std::fs::create_dir_all(path) {
            Ok(()) => println!("{} dir created successfully!", path.display()),
            Err(e) => panic!("Error {}", e.description()),
        }
    } else {
        if path.is_dir() {
            // Already exists
        } else {
            panic!(
                "{} already exists and is not a directory! Remove them manually or change path.",
                path.display()
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
