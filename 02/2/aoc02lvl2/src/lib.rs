use std::cmp::Ordering;
use std::fs;
use std::io;
use std::path::PathBuf;

extern crate clap;
use clap::Parser;

extern crate anyhow;

#[derive(Parser, Debug)]
pub struct Args {
    /// path to the input file
    #[arg(short, long)]
    input: Option<PathBuf>,
}

enum InputConfig {
    File(PathBuf),
    Stdin,
}
pub struct Config {
    input: InputConfig,
}

impl Config {
    pub fn make() -> Self {
        let args = Args::parse();
        let input = if let Some(path) = args.input {
            InputConfig::File(path)
        } else {
            InputConfig::Stdin
        };

        Config { input }
    }
}

pub fn run(cfg: Config) -> anyhow::Result<u32> {
    // figure out where to get our input from and read it into a string
    let input_string = match cfg.input {
        InputConfig::File(path) => fs::read_to_string(path)?,
        InputConfig::Stdin => {
            let mut buf = String::new();
            io::stdin().read_line(&mut buf)?;
            buf
        }
    };

    let data = parse(input_string)?;
    let result = process(data);
    println!("{}", result);

    Ok(result)
}

fn parse(input: String) -> anyhow::Result<Vec<Vec<u32>>> {
    Ok(input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|n| n.parse::<u32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>())
}

fn process(data: Vec<Vec<u32>>) -> u32 {
    let mut safe_count = 0;

    for line in data {
        match get_safety(&line) {
            // if it's already safe, we're good
            Safety::Safe => safe_count += 1,

            // if it's unsafe, try removing each item and see if it becomes safe
            Safety::Unsafe => {
                for index in 0..line.len() {
                    let mut shorter_line = line.clone();
                    shorter_line.remove(index);
                    if let Safety::Safe = get_safety(&shorter_line) {
                        safe_count += 1;
                        break;
                    }
                }
            }
        }
    }

    safe_count
}

fn get_safety(line: &[u32]) -> Safety {
    let mut increasing = Vec::new();
    let mut decreasing = Vec::new();

    for (i, pair) in line[..].windows(2).enumerate() {
        // if there's a difference greater than 3, we violated a rule
        if pair[0].abs_diff(pair[1]) > 3 {
            return Safety::Unsafe;
        }
        // otherwise, keep track of who's increasing and who's decreasing
        match pair[1].cmp(&pair[0]) {
            Ordering::Greater => increasing.push(i),
            Ordering::Less => decreasing.push(i),
            // if it's equal, we violated a rule
            Ordering::Equal => return Safety::Unsafe,
        };
    }

    // if we have both increasing and decreasing, we violated a rule
    if !increasing.is_empty() && !decreasing.is_empty() {
        return Safety::Unsafe;
    }

    Safety::Safe
}

#[derive(Debug)]
enum Safety {
    Safe,
    Unsafe,
}
