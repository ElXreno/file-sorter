use std::fs;
use std::io::Write;
use std::path::PathBuf;

use directories::ProjectDirs;
use serde_derive::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SortPattern {
    pub mime_type: Vec<String>,
    pub destination_subdir: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct Settings {
    pub source_dir: PathBuf,
    pub destination_dir: PathBuf,
    pub sort_patterns: Vec<SortPattern>,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            source_dir: PathBuf::from("/home/elxreno/testing/source"),
            destination_dir: PathBuf::from("/home/elxreno/testing/destination"),
            sort_patterns: vec![
                // Archives
                SortPattern {
                    mime_type: vec![
                        String::from("application/zip"),
                        String::from("application/x-tar"),
                        String::from("application/gzip"),
                        String::from("application/x-xz"),
                        String::from("application/x-rar-compressed"),
                        String::from("application/x-7z-compressed"),
                    ],
                    destination_subdir: String::from("archives"),
                },
                // Images
                SortPattern {
                    mime_type: vec![
                        String::from("image/png"),
                        String::from("image/jpeg"),
                        String::from("image/gif"),
                    ],
                    destination_subdir: String::from("images"),
                },
                // Documents
                SortPattern {
                    mime_type: vec![
                        String::from("text/plain"),
                        String::from("application/vnd.oasis.opendocument.text"),
                        String::from("application/epub+zip"),
                        String::from("image/vnd.djvu"),
                        String::from("application/pdf"),
                    ],
                    destination_subdir: String::from("docs"),
                },
                // Packages
                SortPattern {
                    mime_type: vec![String::from("application/x-rpm")],
                    destination_subdir: String::from("rpm-packages"),
                },
                SortPattern {
                    mime_type: vec![String::from("application/x-debian-package")],
                    destination_subdir: String::from("debian-packages"),
                },
                SortPattern {
                    mime_type: vec![String::from("application/vnd.android.package-archive")],
                    destination_subdir: String::from("apks"),
                },
                // Other
                SortPattern {
                    mime_type: vec![String::from("application/x-bittorrent")],
                    destination_subdir: String::from("torrents"),
                },
            ],
        }
    }
}

impl Settings {
    pub fn load() -> Self {
        let path = Settings::get_settings_path();

        if let Ok(file) = fs::File::open(&path) {
            match serde_json::from_reader(file) {
                Ok(s) => return s,
                Err(e) => {
                    println!("Failed to parse setting file! Fallback to default. {}", e);
                    // Rename the corrupted settings file
                    let mut new_path = path.to_owned();
                    new_path.pop();
                    new_path.push("settings.invalid.json");
                    if let Err(err) = std::fs::rename(path, new_path) {
                        println!("Failed to rename settings file. {}", err);
                    }
                }
            }
        }
        // This is reached if either:
        // - The file can't be opened (presumably it doesn't exist)
        // - Or there was an error parsing the file
        let default_settings = Self::default();
        default_settings.save_to_file_warn();
        default_settings
    }

    pub fn save_to_file_warn(&self) {
        if let Err(err) = self.save_to_file() {
            panic!("Failed to save settings: {:?}", err);
        }
    }

    pub fn save_to_file(&self) -> std::io::Result<()> {
        let path = Settings::get_settings_path();
        if let Some(dir) = path.parent() {
            fs::create_dir_all(dir)?;
        }
        let mut config_file = fs::File::create(path)?;

        let s: &str = &serde_json::to_string_pretty(self).unwrap();
        config_file.write_all(s.as_bytes()).unwrap();

        Ok(())
    }

    fn get_settings_path() -> PathBuf {
        let proj_dirs = ProjectDirs::from("com", "elxreno", "filesorter")
            .expect("System's $HOME directory path not found!");

        proj_dirs
            .config_dir()
            .join("settings")
            .with_extension("json")
    }
}
