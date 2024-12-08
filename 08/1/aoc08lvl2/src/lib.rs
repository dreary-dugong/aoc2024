use itertools::Itertools;
use std::collections::HashMap;
use std::collections::HashSet;
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

fn parse(input: String) -> anyhow::Result<(HashMap<char, Vec<Coord>>, usize, usize)> {
    let max_x = input.lines().count();
    let max_y = input.lines().next().unwrap().chars().count();
    Ok((
        input
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .filter(|(_x, ch)| *ch != '.')
                    .map(move |(x, ch)| {
                        (
                            ch,
                            Coord {
                                x: x as i32,
                                y: y as i32,
                            },
                        )
                    })
            })
            .fold(HashMap::new(), |mut map, (ch, coord)| {
                map.entry(ch).or_default().push(coord);
                map
            }),
        max_x,
        max_y,
    ))
}

fn process(data: (HashMap<char, Vec<Coord>>, usize, usize)) -> u32 {
    let mut antinodes: HashSet<Coord> = HashSet::new();
    let (freq_map, max_x, max_y) = data;

    for (_frequency, positions) in freq_map.iter() {
        for (pos1, pos2) in positions.iter().tuple_combinations() {
            let dx = pos1.x - pos2.x;
            let dy = pos1.y - pos2.y;

            let mut cur = *pos1;
            while is_valid_coord(&cur, max_x, max_y) {
                antinodes.insert(cur);
                cur = Coord {
                    x: cur.x + dx,
                    y: cur.y + dy,
                };
            }

            let mut cur = *pos2;
            while is_valid_coord(&cur, max_x, max_y) {
                antinodes.insert(cur);
                cur = Coord {
                    x: cur.x - dx,
                    y: cur.y - dy,
                };
            }
        }
    }

    antinodes.len() as u32
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Copy)]
struct Coord {
    x: i32,
    y: i32,
}

fn is_valid_coord(coord: &Coord, max_x: usize, max_y: usize) -> bool {
    coord.x >= 0 && coord.y >= 0 && (coord.x as usize) < max_x && (coord.y as usize) < max_y
}
