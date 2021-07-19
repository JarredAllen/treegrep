use std::{
    fs::File,
    io::{stdin, BufRead, BufReader},
};

use clap::{App, Arg};
use regex::Regex;
use treegrep::filter_by_regex;

fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about("Search through tree-formatted output (like from `cargo tree`) to find nodes matching a pattern and their ancestors")
        .arg(
            Arg::with_name("regex")
                .help("Regular expression to search for inside the tree")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("file")
                .help("A file containing the tree (if not specified, will read from stdin)")
                .index(2),
        )
        .get_matches();
    let regex_str = matches.value_of("regex").unwrap();
    let regex = Regex::new(regex_str)?;
    let data = matches
        .value_of("file")
        .map_or_else(read_stdin, read_file)?;
    let data = filter_by_regex(data, regex);
    for line in data {
        println!("{}", line.trim_end());
    }
    Ok(())
}

fn read_stdin() -> Result<Vec<String>, Box<dyn std::error::Error + 'static>> {
    let mut data = Vec::new();
    let stdin = stdin();
    loop {
        let mut buffer = String::new();
        match stdin.read_line(&mut buffer)? {
            0 => return Ok(data),
            _ => {
                data.push(buffer);
            }
        }
    }
}

fn read_file(filename: &str) -> Result<Vec<String>, Box<dyn std::error::Error + 'static>> {
    let file = BufReader::new(File::open(filename)?);
    Ok(file.lines().collect::<Result<Vec<_>, _>>()?)
}
