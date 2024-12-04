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

fn parse(input: String) -> anyhow::Result<Grid> {
    Ok(Grid {
        data: input
            .lines()
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect(),
    })
}

fn process(data: Grid) -> u32 {
    let mut goal = vec!['M', 'A', 'S'];
    goal.sort();
    let mut match_count = 0;

    for x in 0..data.max_x() {
        for y in 0..data.max_y() {
            if x > 0 && y > 0 && data.get(Coord { x, y }) == 'A' {
                let top_left = Coord { x: x - 1, y: y - 1 };
                let line1 = data.get_line(&top_left, 1, 1, 3);
                let bottom_left = Coord { x: x - 1, y: y + 1 };
                let line2 = data.get_line(&bottom_left, 1, -1, 3);

                if let (Some(mut chrs1), Some(mut chrs2)) = (line1, line2) {
                    chrs1.sort();
                    chrs2.sort();
                    if chrs1 == goal && chrs2 == goal {
                        match_count += 1;
                    }
                }
            }
        }
    }

    match_count
}

#[derive(Clone, Copy, Debug)]
struct Coord {
    x: usize,
    y: usize,
}
struct Grid {
    data: Vec<Vec<char>>,
}
impl Grid {
    fn get(&self, coord: Coord) -> char {
        self.data[coord.y][coord.x]
    }
    fn is_valid_coord(&self, coord: &Coord) -> bool {
        coord.x < self.max_x() && coord.y < self.max_y()
    }
    fn get_line(&self, start: &Coord, x_inc: i32, y_inc: i32, length: usize) -> Option<Vec<char>> {
        if !self.is_valid_coord(start) {
            return None;
        }
        let mut out = vec![self.get(*start)];
        let mut cur = *start;
        for _ in 1..length {
            let new_x = cur.x as i32 + x_inc;
            let new_y = cur.y as i32 + y_inc;
            if new_x < 0 || new_y < 0 {
                return None;
            }
            cur = Coord {
                x: new_x as usize,
                y: new_y as usize,
            };
            if !self.is_valid_coord(&cur) {
                return None;
            }
            out.push(self.get(cur));
        }

        Some(out)
    }
    fn max_x(&self) -> usize {
        self.data[0].len()
    }
    fn max_y(&self) -> usize {
        self.data.len()
    }
}
