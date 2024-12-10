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

fn parse(input: String) -> anyhow::Result<TopographicMap> {
    let map_data = input
        .lines()
        .map(|line| line.chars().map(|ch| ch.to_digit(10).unwrap()).collect())
        .collect();
    Ok(TopographicMap { data: map_data })
}

fn process(data: TopographicMap) -> u32 {
    data.get_trailheads()
        .into_iter()
        .map(|trailhead| data.count_nines(&trailhead))
        .sum()
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Pos {
    x: usize,
    y: usize,
}
impl Pos {
    fn new(x: usize, y: usize) -> Self {
        Pos { x, y }
    }
}

#[derive(Debug)]
struct TopographicMap {
    data: Vec<Vec<u32>>,
}
impl TopographicMap {
    fn get_height_at(&self, pos: &Pos) -> u32 {
        self.data[pos.y][pos.x]
    }
    fn get_max_x(&self) -> usize {
        self.data[0].len()
    }
    fn get_max_y(&self) -> usize {
        self.data.len()
    }
    fn is_valid_pos(&self, x: i32, y: i32) -> bool {
        x >= 0 && x < self.get_max_x() as i32 && y >= 0 && y < self.get_max_y() as i32
    }
    fn get_neighbors(&self, pos: &Pos) -> Vec<Pos> {
        let (x, y) = (pos.x as i32, pos.y as i32);
        let diffs = vec![(1, 0), (0, 1), (-1, 0), (0, -1)];
        let mut neighbors = Vec::new();
        for diff in diffs {
            let test_x = x + diff.0;
            let test_y = y + diff.1;
            if self.is_valid_pos(test_x, test_y) {
                neighbors.push(Pos::new(test_x as usize, test_y as usize))
            }
        }

        neighbors
    }
    fn get_trailheads(&self) -> Vec<Pos> {
        self.data
            .iter()
            .enumerate()
            .flat_map(|(y, line)| {
                line.iter()
                    .enumerate()
                    .filter(|(_x, h)| **h == 0)
                    .map(move |(x, _h)| Pos::new(x, y))
            })
            .collect()
    }
    fn count_nines(&self, trail: &Pos) -> u32 {
        let cur_height = self.get_height_at(trail);
        // base case
        if cur_height == 9 {
            return 1;
        }
        // otherwise, sum of the neighbors
        self.get_neighbors(trail)
            .into_iter()
            .filter(|neighbor| self.get_height_at(neighbor) == cur_height + 1)
            .map(|neighbor| self.count_nines(&neighbor))
            .sum()
    }
}
