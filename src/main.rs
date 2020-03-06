#[macro_use]
extern crate clap;

use clap::{App, AppSettings, Arg};

mod settings;
mod utils;

fn main() {
    // TODO: Add more arguments
    let matches = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .setting(AppSettings::ArgRequiredElseHelp)
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
        settings::Settings::rewrite_config();
    }

    if matches.is_present("sort") {
        sort();
    }
}

fn sort() {
    println!("Starting...");

    let settings = settings::Settings::load();

    if !settings.source.is_dir() {
        panic!("Source path exists but is not a directory! Shutting down...");
    }

    utils::create_dirs(vec![&settings.source, &settings.destination]);

    let files = utils::get_files(&settings.source);

    for file in &files {
        // TODO: Fallback to mime-type detection if file doesn't have extension

        let file_extension = &file.extension();

        if let None = file_extension {
            println!(
                "Failed to get extension for file '{}', skipping it...",
                &file.display()
            );
            continue;
        }

        let file_extension_str = &file_extension
            .unwrap()
            .to_os_string()
            .into_string()
            .unwrap();

        for pattern in &settings.sort_patterns {
            if pattern.extensions.contains(&file_extension_str) {
                let destination_dir = settings.destination.join(&pattern.destination);

                let destination_file = &destination_dir.join(&file.file_name().unwrap());

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
