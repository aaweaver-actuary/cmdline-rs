use crate::common;
use clap::App;

pub fn get_matches() -> common::MyResult<clap::ArgMatches<'static>> {
    let version: &str = &common::get_version("ECHO_RS_VERSION");
    let binding = common::get_author();
    let author: &str = binding.as_str();

    let text = common::text_arg("text", "The text to print");
    let omit_newline = common::flag_arg("omit_newline", "n", "Do not print newline", false);

    Ok(App::new("echo-rs")
        .version(version)
        .author(author)
        .about("Rust version of echo")
        .arg(text)
        .arg(omit_newline)
        .get_matches())
}

pub fn print_text(text: Vec<String>, omit_newline: bool) {
    let text = text.join(" ");
    let omit_newline = if omit_newline { "" } else { "\n" };
    println!("{}{}", text, omit_newline);
}

pub fn run() {
    let matches = get_matches();

    let text = &matches
        .as_ref()
        .expect("Error extracting text content.")
        .values_of_lossy("text");

    let omit_newline = &matches
        .expect("Error with omit newline -n flag.")
        .is_present("omit_newline");

    // If the text is ok, print it.
    // Otherwise, print an error message.
    if let Some(text) = text {
        print_text(text.to_vec(), *omit_newline);
    } else {
        eprintln!("Error: No text provided.");
    }
}
