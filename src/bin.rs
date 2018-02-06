extern crate atty;
#[macro_use]
extern crate clap;
extern crate dropbox_dir;

use atty::Stream;
use clap::{App, Arg};
use dropbox_dir::Error as DBE;
use dropbox_dir::SmartPath;

fn print_path(prefix: &str, path: Result<SmartPath, DBE>) -> i32 {
    match path {
        Ok(sp) => {
            print!("{}{}", prefix, sp.local().display());
            if atty::is(Stream::Stdout) {
                println!();
            }
            0
        }
        Err(e) => {
            eprintln!("{}error: <{}>", prefix, e);
            1
        }
    }
}

pub fn main() {
    let matches = App::new(crate_name!())
        .version(crate_version!())
        .about(crate_description!())
        .arg(
            Arg::with_name("business")
                .short("b")
                .long("business")
                .help("Output path for business instead of personal account."),
        )
        .arg(
            Arg::with_name("all")
                .long("all")
                .help("Output path for personal and business accounts."),
        )
        .arg(
            Arg::with_name("path")
                .default_value("")
                .help("A path relative to Dropbox's folder."),
        )
        .get_matches();

    let mode_all = matches.is_present("all");
    let mode_business = !mode_all && matches.is_present("business");
    let added_path = matches.value_of("path").unwrap();
    let mut exit_status = 0;
    if !mode_business {
        let prefix = if mode_all { "personal: " } else { "" };
        exit_status = print_path(prefix, SmartPath::new_personal(added_path));
    }
    if mode_all || mode_business {
        let prefix = if mode_all { "business: " } else { "" };
        exit_status = print_path(prefix, SmartPath::new_business(added_path));
    }
    std::process::exit(exit_status)
}
