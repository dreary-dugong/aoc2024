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
    'lvl_loop: for lvl in data {
        let transition = match lvl[0].cmp(&lvl[1]) {
            std::cmp::Ordering::Less => "less",
            std::cmp::Ordering::Greater => "Greater",
            std::cmp::Ordering::Equal => continue,
        };

        for pair in lvl[0..lvl.len()].windows(2) {
            match pair[0].cmp(&pair[1]) {
                std::cmp::Ordering::Less => {
                    if transition == "Greater" {
                        continue 'lvl_loop;
                    }
                }
                std::cmp::Ordering::Greater => {
                    if transition == "less" {
                        continue 'lvl_loop;
                    }
                }
                std::cmp::Ordering::Equal => continue 'lvl_loop,
            }

            if pair[0].abs_diff(pair[1]) > 3 {
                continue 'lvl_loop;
            }
        }

        safe_count += 1;
    }

    safe_count
}
