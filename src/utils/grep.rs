use clap::{App, Arg};
use regex::Regex;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

fn test_grep() {
    let args = App::new("grep-lite")
        .version("0.1")
        .about("searches for patterns")
        .arg(
            Arg::with_name("pattern")
                .help("The pattern to search for")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("input")
                .help("File to search")
                .takes_value(true)
                .required(true),
        )
        .get_matches();

    let pattern = args.value_of("pattern").unwrap();
    let re = Regex::new(pattern).unwrap();

    let input = args.value_of("input").unwrap_or("-");

    if input == "-" {
        let stdin = io::stdin();
        let reader = stdin.lock();

        process_lines(reader, re);
    } else {
        let f = File::open(input).unwrap();
        let reader = BufReader::new(f);
        process_lines(reader, re);
    }
}

fn from_file() {
    let contents: String =
        fs::read_to_string("input.txt").expect("Should have been able to read the file");

    let parts: Vec<u64> = contents
        .trim_end()
        .split(" ")
        .map(|x| x.parse::<u64>().unwrap())
        .collect();

    let c: u64 = parts.iter().sum();
    fs::write("output.txt", c.to_string()).expect("Unable to write file");
}

fn from_stdin() {
    let mut contents = String::new();
    let stdin = io::stdin();
    let res: Result<usize, io::Error> = stdin.read_line(&mut contents);

    let parts: Vec<i64> = contents
        .trim_end()
        .split(" ")
        .map(|x| x.parse::<i64>().unwrap())
        .collect();

    let c: i64 = parts.iter().sum();
    // io::stdout().write_all(c.to_be_bytes().as_slice()).unwrap();
    println!("{}", c);
}

fn process_lines<T: BufRead + Sized>(reader: T, re: Regex) {
    for line_ in reader.lines() {
        let line = line_.unwrap();
        match re.find(&line) {
            Some(_) => println!("{}", line),
            None => (),
        }
    }
}
