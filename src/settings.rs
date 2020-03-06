use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

use directories::ProjectDirs;
use serde_derive::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SortPattern {
    pub extensions: Vec<String>,
    pub destination: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct Settings {
    pub source: PathBuf,
    pub destination: PathBuf,
    pub use_date_pattern: bool,
    pub date_pattern: String,
    pub sort_patterns: Vec<SortPattern>,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            source: PathBuf::from("/home/elxreno/Downloads/"),
            destination: PathBuf::from("/home/elxreno/Downloads/Sorted"),
            use_date_pattern: false,
            date_pattern: String::from("%Y-%m-%d"), // 2020-01-01
            sort_patterns: vec![
                // Archives
                SortPattern {
                    extensions: vec![
                        String::from("zip"),
                        String::from("tar"),
                        String::from("tgz"),
                        String::from("gz"),
                        String::from("xz"),
                        String::from("rar"),
                        String::from("7z"),
                    ],
                    destination: String::from("archives"),
                },
                // Audio
                SortPattern {
                    extensions: vec![
                        String::from("mp3"),
                        String::from("wav"),
                        String::from("flac"),
                        String::from("opus"),
                        String::from("ogg"),
                    ],
                    destination: String::from("audio"),
                },
                // Binary
                SortPattern {
                    extensions: vec![String::from("exe"), String::from("bin")],
                    destination: String::from("binary"),
                },
                // Images
                SortPattern {
                    extensions: vec![
                        String::from("png"),
                        String::from("jpg"),
                        String::from("jpeg"),
                        String::from("gif"),
                        String::from("tif"),
                    ],
                    destination: String::from("images"),
                },
                // Documents
                SortPattern {
                    extensions: vec![
                        String::from("txt"),
                        String::from("odt"),
                        String::from("epub"),
                        String::from("djvu"),
                        String::from("pdf"),
                    ],
                    destination: String::from("docs"),
                },
                // Packages
                SortPattern {
                    extensions: vec![String::from("rpm"), String::from("spec")],
                    destination: String::from("rpm-packages"),
                },
                SortPattern {
                    extensions: vec![String::from("deb")],
                    destination: String::from("debian-packages"),
                },
                SortPattern {
                    extensions: vec![String::from("apk"), String::from("apkx")],
                    destination: String::from("apks"),
                },
                // Other
                SortPattern {
                    extensions: vec![String::from("torrent")],
                    destination: String::from("torrents"),
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
                    new_path.push("settings.json.invalid");
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

    pub fn rewrite_config() {
        let settings_file = Settings::get_settings_path();
        let settings_file_old = format!("{}.old", &settings_file.display());
        if Path::new(&settings_file).exists() {
            match std::fs::rename(&settings_file, &settings_file_old) {
                Ok(_o) => {
                    println!("Moved old settings file to {}", &settings_file_old);
                }
                Err(e) => panic!("Error {}", e),
            }
        } else {
            println!("Config file doesn't exists, just creating new...")
        }

        Settings::load();
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

    pub fn get_settings_path() -> PathBuf {
        let proj_dirs = ProjectDirs::from("com", "elxreno", "filesorter")
            .expect("System's $HOME directory path not found!");

        proj_dirs
            .config_dir()
            .join("settings")
            .with_extension("json")
    }
}
