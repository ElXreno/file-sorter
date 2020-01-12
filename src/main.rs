use clap::{App, AppSettings, Arg};

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
            Arg::with_name("sort")
                .short("s")
                .long("sort")
                .help("Runs sorting"),
        )
        .get_matches();

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
        let file_extension = &file
            .extension()
            .unwrap()
            .to_os_string()
            .into_string()
            .unwrap();
        for pattern in &settings.sort_patterns {
            if pattern.extensions.contains(file_extension) {
                let destination_dir = &settings.destination.join(&pattern.destination);

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
