#[macro_use]
extern crate clap;

use std::io::prelude::*;
use std::fs::File;

fn main() {
    let matches = clap_app!(app =>
        (version: "1.0")
        (author: "LightDiscord <arnaud@lightdiscord.me>")
        (about: "Format code with some semicolons")
        (@arg INPUT: +required "Sets the input file to use")
    ).get_matches();

    let file = matches.value_of("INPUT").unwrap();
    let file = handle_file(file).unwrap();
}

fn handle_file (file: &str) -> std::io::Result<()> {
    let mut file = File::open(file)?;
    let mut content = String::new();

    file.read_to_string(&mut content)?;

    let content: String = content.lines()
        .map(|line| process_line(line))
        .collect::<Vec<String>>()
        .join("\n");

    println!("{}", content);

    Ok(())
}

fn process_line (line: &str) -> String {
    let trim = line.trim_left();
    let size = line.len() - trim.len();
    let semi = vec![';'; size];

    semi.into_iter().chain(trim.chars()).collect()
}