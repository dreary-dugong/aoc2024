use std::fs;
use std::io;
use std::path::PathBuf;

extern crate clap;
use clap::Parser;

extern crate anyhow;

extern crate regex;
use regex::Regex;

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

fn parse(input: String) -> anyhow::Result<Vec<Inst>> {
    let regex = Regex::new(r"(mul\((\d{1,3}),(\d{1,3})\))|(do\(\))|(don\'t\(\))").unwrap();
    let mut result = Vec::new();
    for cap in regex.captures_iter(&input) {
        if cap.get(1).is_some() {
            let x = cap.get(2).unwrap().as_str().parse::<u32>().unwrap();
            let y = cap.get(3).unwrap().as_str().parse::<u32>().unwrap();
            result.push(Inst::Mul(x, y));
        } else if cap.get(4).is_some() {
            result.push(Inst::Do);
        } else if cap.get(5).is_some() {
            result.push(Inst::Dont);
        }
    }

    Ok(result)
}

fn process(data: Vec<Inst>) -> u32 {
    let mut do_muls = true;
    let mut result = 0;
    for inst in data.into_iter() {
        match inst {
            Inst::Do => do_muls = true,
            Inst::Dont => do_muls = false,
            Inst::Mul(x, y) => {
                if do_muls {
                    result += x * y
                }
            }
        }
    }

    result
}

#[derive(Debug)]
enum Inst {
    Mul(u32, u32),
    Do,
    Dont,
}
