use clap::Arg;
use dotenv;
use std::error::Error;

/// A type alias for handling errors in this program.
pub type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn get_version(key: &str) -> String {
    // Get the version from the .env file.
    dotenv::var(key)
        // If the version is not found, use a default version.
        .unwrap_or("UNKNOWNVERSION".to_string())
}

pub fn get_author() -> String {
    dotenv::var("AUTHOR").unwrap_or("Andy Weaver".to_string())
}

pub fn flag_arg<'a>(
    name: &'a str,
    short: &'a str,
    help: &'a str,
    takes_value: bool,
) -> Arg<'a, 'a> {
    Arg::with_name(name)
        .short(short)
        .help(help)
        .takes_value(takes_value)
}

pub fn text_arg<'a>(name: &'a str, help: &'a str) -> Arg<'a, 'a> {
    Arg::with_name(name)
        .value_name("TEXT")
        .help(help)
        .required(true)
        .min_values(1)
}
