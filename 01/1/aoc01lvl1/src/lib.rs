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

fn parse(input: String) -> anyhow::Result<(Vec<u32>, Vec<u32>)> {
    let mut list1 = Vec::new();
    let mut list2 = Vec::new();
    for line in input.lines() {
        let mut iter = line.split_whitespace();
        list1.push(iter.next().unwrap().parse::<u32>().unwrap());
        list2.push(iter.next().unwrap().parse::<u32>().unwrap());
    }
    Ok((list1, list2))
}

fn process(data: (Vec<u32>, Vec<u32>)) -> u32 {
    let (mut list1, mut list2) = data;
    list1.sort_unstable();
    list2.sort_unstable();
    let mut total = 0;
    for (n1, n2) in list1.into_iter().zip(list2.into_iter()) {
        total += n1.abs_diff(n2);
    }

    total
}
