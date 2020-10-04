/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

#[macro_use]
extern crate clap;
extern crate chrono;

use std::path::PathBuf;

mod settings;
mod utils;

fn main() {
    let matches = utils::get_arg_matches();

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
                    .add_source(source_dir)
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

    if settings.sources.len() == 0 {
        panic!("Config file not initialized, you should initialize them! Run `filesorter help init` for help.")
    }

    for source in &settings.sources {
        if !source.exists() {
            panic!("Source dir '{}' doesn't exists!", source.display());
        }
        if !source.is_dir() {
            panic!(
                "Source dir '{}' exists but is not a directory, exiting...",
                source.display()
            );
        }
        if !settings.destination.is_dir() {
            panic!(
                "Destination dir '{}' doesn't exists!",
                settings.destination.display()
            );
        }
        if !settings.destination.is_dir() {
            panic!(
                "Destination dir '{}' exists but is not a directory, exiting...",
                settings.destination.display()
            );
        }

        utils::create_dirs(vec![&source, &settings.destination]);

        let files = utils::get_files(&source);
        for file in &files {
            // Ignore files which starts from dot
            if let Some(filename) = &file.file_name() {
                if &filename.to_str().unwrap_or(".")[..1] == "." {
                    println!("Ignoring file {}", &file.display());
                    continue;
                }

                let file_extension = &file.extension();

                if let None = file_extension {
                    'outer: for pattern in &settings.sort_patterns {
                        for mime_type in &pattern.mime_types {
                            if tree_magic::match_filepath(mime_type, &file) {
                                let destination_dir = utils::get_destination_dir(
                                    &settings,
                                    &file,
                                    &pattern.destination,
                                );
                                let destination_file =
                                    &destination_dir.join(&file.file_name().unwrap());

                                utils::move_file(&file, &destination_dir, &destination_file);

                                break 'outer;
                            }
                        }
                    }

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
                        let destination_dir =
                            utils::get_destination_dir(&settings, &file, &pattern.destination);
                        let destination_file = &destination_dir.join(&file.file_name().unwrap());

                        utils::move_file(&file, &destination_dir, &destination_file);

                        break;
                    }
                }
            } else {
                println!("Failed to proceed file {}", file.display());
                continue;
            }
        }
    }

    println!("Done!")
}
