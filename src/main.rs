extern crate clap;
use std::process;

use clap::{App, Arg, ArgMatches};

fn check_apps(supported_apps: &[String; 3], matches: &ArgMatches) -> (String, String) {

    let source_app = match matches.value_of("from") {
        Some(v) => {
            if supported_apps.contains(&v.to_string()) {
                v
            } else {
                print!(
                    "Wrong --from application supplied ({}). Supported applications are: ",
                    v
                );

                println!("{}", supported_apps.join(", "));
                process::exit(1);
            }
        }
        None => {
            print!("No --from application supplied. This is technically impossible.");
            process::exit(1);
        }
    };

    let destination_app = match matches.value_of("to") {
        Some(v) => {
            if v == source_app {
                println!(
                    "Error: --to application is equal to --from ({}). Skipping",
                    source_app
                );
                process::exit(1);
            } else if supported_apps.contains(&v.to_string()) {
                v
            } else {
                print!(
                    "Wrong --to application supplied ({}). Supported applications are: ",
                    v
                );
                println!("{}", supported_apps.join(", "));
                process::exit(1);
            }
        }
        None => {
            print!("No --from application supplied. This is technically impossible.");
            process::exit(1);
        }
    };

    (source_app.to_string(), destination_app.to_string())
}

fn main() {

    // Parse arguments
    let matches = App::new("UDIM Textures Renamer")
        .version("1.0")
        .author("Valerio V. <10340139+vvzen@users.noreply.github.com>")
        .about("Copies and renames textures from/to mari, mudbox and zbrush adhering to their different UDIM naming standards.")
        .arg(
            Arg::with_name("dir")
                .long("-dir")
                .value_name("DIR")
                .help("Target directory where to look for textures")
                .takes_value(true)
                .required(true)
        )
        .arg(
            Arg::with_name("from")
                .long("-from")
                .value_name("FROM")
                .help("Sets the source application")
                .takes_value(true)
                .required(true)
                // .index(1),
        )
        .arg(
            Arg::with_name("to")
                .long("-to")
                .value_name("TO")
                .help("Sets the destination application")
                .takes_value(true)
                .required(true),
        )
        .get_matches();

    let target_dir = matches.value_of("dir").unwrap();

    let supported_apps: [String; 3] = [
        "mari".to_string(),
        "mudbox".to_string(),
        "zbrush".to_string(),
    ];

    let (source_app, destination_app) = check_apps(&supported_apps, &matches);

    println!(
        "dir: {} from: {} to: {}",
        target_dir, source_app, destination_app
    );
}
