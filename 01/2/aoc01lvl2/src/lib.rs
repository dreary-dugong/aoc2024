use std::collections::HashMap;
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
    Ok(input
        .lines()
        .fold((Vec::new(), Vec::new()), |mut lists, line| {
            let mut iter = line.split_whitespace().map(|n| n.parse::<u32>().unwrap());
            lists.0.push(iter.next().unwrap());
            lists.1.push(iter.next().unwrap());
            lists
        }))
}

fn process(data: (Vec<u32>, Vec<u32>)) -> u32 {
    let (list1, list2) = data;
    let counts = list2.into_iter().fold(HashMap::new(), |mut map, n| {
        map.entry(n).and_modify(|v| *v += 1).or_insert(1);
        map
    });

    list1
        .into_iter()
        .map(|n| counts.get(&n).unwrap_or(&0) * n)
        .sum()
}
