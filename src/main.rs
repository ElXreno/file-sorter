use clap::{App, AppSettings, Arg};
use std::path::Path;

mod app;
mod settings;
mod utils;

fn main() {
    // TODO: Add more arguments
    let matches = App::new(app::NAME)
        .setting(AppSettings::ArgRequiredElseHelp)
        .version(app::VERSION)
        .author(app::AUTHOR)
        .about(app::ABOUT)
        .arg(
            Arg::with_name("rewrite-config")
                .long("rewrite-config")
                .help("Rewrite default config"),
        )
        .arg(
            Arg::with_name("sort")
                .short("s")
                .long("sort")
                .help("Runs sorting"),
        )
        .get_matches();

    if matches.is_present("rewrite-config") {
        let settings_file = settings::Settings::get_settings_path();
        let settings_file_dest = format!("{}.old", &settings_file.display());
        if Path::new(&settings_file).exists() {
            match std::fs::rename(&settings_file, &settings_file_dest) {
                Ok(_o) => {
                    settings::Settings::load();
                    println!("Done!");
                }
                Err(e) => panic!("Error {}", e),
            }
        } else {
            println!("Config file doesn't exists, nothing to do.")
        }
        return;
    }

    if !matches.is_present("sort") {
        return;
    }

    println!("Sorting is running...");

    let settings = settings::Settings::load();

    if !settings.source.is_dir() {
        panic!("Source path exists and is not a directory, exiting.");
    }

    utils::create_dir(&settings.source);
    utils::create_dir(&settings.destination);

    let files = utils::get_files(&settings.source);

    for file in &files {
        // TODO: Fallback to mime-type detection if file doesn't have extension
        // TODO: Fix work with files without name | Example: .directory .file
        let file_extension = &file
            .extension()
            .unwrap()
            .to_os_string()
            .into_string()
            .unwrap();
        for pattern in &settings.sort_patterns {
            if pattern.extensions.contains(file_extension) {
                let destination_dir = settings.destination.join(&pattern.destination);

                let destination_file = destination_dir.join(&file.file_name().unwrap());

                utils::create_dir(&destination_dir);
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

    println!("Done!")
}
