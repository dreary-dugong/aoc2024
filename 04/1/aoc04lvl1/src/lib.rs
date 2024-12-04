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

fn parse(input: String) -> anyhow::Result<Vec<Vec<char>>> {
    Ok(input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect())
}

fn process(data: Vec<Vec<char>>) -> u32 {
    const GOAL: [char; 4] = ['X', 'M', 'A', 'S'];
    let mut match_count = 0;
    let max_x = data.len() - 1;
    let max_y = data[0].len() - 1;
    for (x, row) in data.iter().enumerate() {
        for (y, ch) in row.iter().enumerate() {
            if *ch == GOAL[0] {
                for line in get_line_coords((x, y), GOAL.len(), (max_x, max_y)) {
                    if line
                        .into_iter()
                        .map(|(lx, ly)| data[lx][ly])
                        .collect::<Vec<char>>()
                        == GOAL[1..GOAL.len()]
                    {
                        match_count += 1;
                    }
                }
            }
        }
    }

    match_count
}

fn get_line_coords(
    coord: (usize, usize),
    len: usize,
    max: (usize, usize),
) -> Vec<Vec<(usize, usize)>> {
    let (x, y) = coord;
    let (x, y) = (x as i32, y as i32);
    let (max_x, max_y) = max;
    let (max_x, max_y) = (max_x as i32, max_y as i32);
    let mut lines = vec![vec!(); 8];
    for i in 1..len {
        let i = i as i32;
        lines[0].push((x + i, y));
        lines[1].push((x, y + i));
        lines[2].push((x + i, y + i));
        lines[3].push((x - i, y));
        lines[4].push((x, y - i));
        lines[5].push((x - i, y - i));
        lines[6].push((x + i, y - i));
        lines[7].push((x - i, y + i));
    }
    lines
        .into_iter()
        .filter(|line| {
            line.iter()
                .map(|(x, y)| 0 <= *x && *x <= max_x && 0 <= *y && *y <= max_y)
                .all(|v| v)
        })
        .map(|line| {
            line.iter()
                .map(|(x, y)| (*x as usize, *y as usize))
                .collect()
        })
        .collect()
}
