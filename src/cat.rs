/// This is the library crate for the `cat-rs` command-line tool.
/// It provides the `cat` function that reads a file and prints its contents to the standard output.
use crate::common;
use clap::{App, Arg};
use dotenvy;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Debug, Clone)]
pub struct CatConfig {
    files: Vec<String>,
    show_all: bool,
    number_lines: bool,
    number_nonblank_lines: bool,
}

impl CatConfig {
    /// Validate the configuration and return a new instance of `CatConfig`.
    ///
    /// # Validation Rules
    /// - If `number_nonblank_lines` is set, `number_lines` is ignored.
    /// - If `show_all` is set, it is removed from the configuration.
    ///
    pub fn validate_config(&mut self) -> CatConfig {
        if self.number_nonblank_lines {
            self.number_lines = false;
        }

        if self.show_all {
            self.show_all = false;
        }

        self.clone()
    }
}

pub fn get_args() -> common::MyResult<CatConfig> {
    dotenvy::dotenv()?;
    let version: &str = &dotenvy::var("CAT_RS_VERSION")?;
    let binding = common::get_author();
    let author: &str = binding.as_str();

    let matches = App::new("cat-rs")
        .version(version)
        .author(author)
        .about("Rust version of cat")
        .arg(
            Arg::with_name("files")
                .value_name("FILES")
                .help("The files to print")
                .default_value("-")
                .min_values(1),
        )
        .arg(
            Arg::with_name("show_all")
                .short("A")
                .long("show-all")
                .help("NOT IMPLEMENTED: equivalent to -vET")
                .required(false),
        )
        .arg(
            Arg::with_name("number_lines")
                .short("n")
                .long("number")
                .help("number all output lines")
                .required(false),
        )
        .arg(
            Arg::with_name("number_nonblank_lines")
                .short("b")
                .long("number-nonblank")
                .help("number nonempty output lines, overrides -n"),
        )
        .get_matches();

    Ok(CatConfig {
        files: matches.values_of_lossy("files").unwrap(),
        show_all: matches.is_present("show_all"),
        number_lines: matches.is_present("number_lines"),
        number_nonblank_lines: matches.is_present("number_nonblank_lines"),
    })
}

fn open_file(filename: &str) -> common::MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

fn read_file(filename: &str, n_flag: bool, b_flag: bool) -> common::MyResult<()> {
    let file = open_file(filename)?;
    for (i, line) in file.lines().enumerate() {
        if b_flag {
            if i + 1 < 10 {
                print!("    {} | ", i + 1);
            } else if i + 1 < 100 {
                print!("  {} | ", i + 1);
            } else if i + 1 < 1000 {
                print!(" {} | ", i + 1);
            } else {
                print!("{} | ", i + 1);
            }
        }
        println!("{}", line?);
    }
    Ok(())
}

fn print_error(file: &str, error: &str) {
    eprintln!(
        "cat-rs failed:\n=============\n\tfile:\t{}\n\terror:\t{}\n",
        file, error
    );
}

pub fn run(config: CatConfig) -> common::MyResult<()> {
    for file in config.files {
        match open_file(&file) {
            Err(e) => print_error(&file, &e.to_string()),
            Ok(_) => read_file(&file, config.number_lines, config.number_nonblank_lines)?,
        }
    }
    Ok(())
}
