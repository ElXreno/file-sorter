#[macro_use]
extern crate clap;
extern crate chrono;

use chrono::prelude::*;
use chrono::DateTime;
use clap::{App, AppSettings, Arg, SubCommand};
use std::path::PathBuf;

mod settings;
mod utils;

fn main() {
    let matches = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .setting(AppSettings::ArgRequiredElseHelp)
        .subcommand(
            SubCommand::with_name("init")
                .about("(Re)Initialize configuration file")
                .arg(
                    Arg::with_name("source_dir")
                        .help("Source directory")
                        .index(1)
                        .required(true)
                        .value_name("SOURCE"),
                )
                .arg(
                    Arg::with_name("destination_dir")
                        .help("Destination directory")
                        .index(2)
                        .required(true)
                        .value_name("DESTINATION"),
                )
                .arg(
                    Arg::with_name("use_date_pattern")
                        .short("d")
                        .long("use-date-pattern")
                        .help("Use date pattern")
                        .takes_value(false),
                )
                .arg(
                    Arg::with_name("date_pattern")
                        .short("p")
                        .long("date-pattern")
                        .help("Date subdir pattern")
                        .takes_value(true)
                        .default_value("%Y-%m-%d"), // 2020-01-01
                ),
        )
        .subcommand(SubCommand::with_name("sort").about(
            "Sorting source directory to destination (config file should be initialized first!)",
        ))
        .get_matches();

    match matches.subcommand_name() {
        Some("init") => {
            if let Some(ref matches) = matches.subcommand_matches("init") {
                let source_dir =
                    PathBuf::from(matches.value_of("source_dir").expect("Expected source dir"));
                let destination_dir = PathBuf::from(
                    matches
                        .value_of("destination_dir")
                        .expect("Expected destination dir"),
                );

                let use_date_pattern = matches.is_present("use_date_pattern");
                let date_pattern = matches
                    .value_of("date_pattern")
                    .expect("Expected output pattern");

                println!("Source dir: {}", source_dir.display());
                println!("Destination dir: {}", destination_dir.display());
                println!("Use date pattern: {}", use_date_pattern);
                println!("Date pattern: {}", date_pattern);

                let mut settings = settings::Settings::default();

                settings
                    .source(source_dir)
                    .destination(destination_dir)
                    .use_date_pattern(use_date_pattern)
                    .date_pattern(date_pattern.to_string());

                settings.backup_old_config().save_to_file_warn();

                println!("Initialized successfully!");
            } else {
                panic!("No provided arguments!");
            }
        }
        Some("sort") => sort(),
        None => {}
        _ => {}
    }
}

fn sort() {
    println!("Starting...");

    let settings = settings::Settings::load();

    if settings.source == PathBuf::new() || settings.destination == PathBuf::new() {
        panic!("Config file not initialized, you should initialize them! Run `filesorter help init` for help.")
    }
    if !settings.source.is_dir() || !settings.destination.is_dir() {
        panic!("Source or destination path exists but is not a directory, exiting...");
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
            .unwrap()
            .to_lowercase();

        for pattern in &settings.sort_patterns {
            if pattern.extensions.contains(&file_extension_str) {
                let destination_dir = if settings.use_date_pattern {
                    let metadata = std::fs::metadata(&file);
                    let modify_date = DateTime::<Utc>::from(metadata.unwrap().modified().unwrap());
                    let date_folder = modify_date.format(&settings.date_pattern).to_string();

                    settings
                        .destination
                        .join(&date_folder)
                        .join(&pattern.destination)
                } else {
                    settings.destination.join(&pattern.destination)
                };

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
