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

pub fn run(cfg: Config) -> anyhow::Result<u64> {
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

fn parse(input: String) -> anyhow::Result<Vec<Block>> {
    Ok(input
        .strip_suffix("\n")
        .unwrap()
        .chars()
        .collect::<Vec<char>>()
        .chunks(2)
        .enumerate()
        .flat_map(|(i, chs)| {
            let file = chs[0].to_digit(10).unwrap() as usize;
            let mut blocks = vec![Block::File(i as u32); file];
            if chs.len() > 1 {
                let free = chs[1].to_digit(10).unwrap() as usize;
                blocks.extend(vec![Block::Free; free]);
            }
            blocks
        })
        .collect::<Vec<Block>>())
}

fn process(data: Vec<Block>) -> u64 {
    let mut disk = data;

    let mut free_cursor = 0usize;
    let mut file_cursor = disk.len() - 1;

    // keep moving files until we meet in the middle
    while free_cursor < file_cursor {
        // move free cursor
        while let Block::File(_) = disk[free_cursor] {
            free_cursor += 1;
        }

        // move file cursor
        while let Block::Free = disk[file_cursor] {
            file_cursor -= 1;
        }

        // double check exit condition, then swap
        if free_cursor < file_cursor {
            disk.swap(free_cursor, file_cursor);
        }
    }

    calc_checksum(&disk)
}

fn calc_checksum(disk: &[Block]) -> u64 {
    disk.iter()
        .map(|block| match block {
            Block::File(n) => *n,
            Block::Free => 0,
        })
        .enumerate()
        .map(|(i, n)| i as u64 * (n as u64))
        .sum()
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum Block {
    Free,
    File(u32),
}
