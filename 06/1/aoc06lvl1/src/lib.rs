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

fn parse(input: String) -> anyhow::Result<(Board, Guard)> {
    let mut guard_pos = None;
    let mut guard_direction = None;
    let grid = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, chr)| match chr {
                    '.' => Space::Empty,
                    '#' => Space::Obstacle,
                    'v' | '<' | '>' | '^' => {
                        guard_pos = Some(Pos {
                            x: x as i32,
                            y: y as i32,
                        });
                        guard_direction = match chr {
                            '^' => Some(Direction::North),
                            '>' => Some(Direction::East),
                            'v' => Some(Direction::South),
                            '<' => Some(Direction::South),
                            _ => unreachable!(),
                        };
                        Space::Empty
                    }
                    _ => Space::Empty,
                })
                .collect::<Vec<Space>>()
        })
        .collect::<Vec<Vec<Space>>>();

    Ok((
        Board { grid },
        Guard {
            pos: guard_pos.unwrap(),
            direction: guard_direction.unwrap(),
        },
    ))
}

fn process(data: (Board, Guard)) -> u32 {
    let (board, mut guard) = data;

    // keep track of positions we've touched
    let mut seen = HashSet::new();
    seen.insert(guard.pos);

    // execute until the next move is off the board
    while board.is_in_bounds(&guard.get_facing_pos()) {
        // if we're facing an obstacle, turn right
        while let Some(Space::Obstacle) = board.get_space(&guard.get_facing_pos()) {
            guard.turn_right();
        }
        // otherwise, move forward
        guard.move_forward();
        seen.insert(guard.pos);
    }

    seen.len() as u32
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Pos {
    x: i32,
    y: i32,
}
#[derive(Clone, Copy, Debug)]
enum Space {
    Empty,
    Obstacle,
}
struct Board {
    grid: Vec<Vec<Space>>,
}
impl Board {
    fn get_space(&self, pos: &Pos) -> Option<Space> {
        if !self.is_in_bounds(pos) {
            None
        } else {
            Some(self.grid[pos.y as usize][pos.x as usize])
        }
    }
    fn is_in_bounds(&self, pos: &Pos) -> bool {
        pos.x >= 0
            && pos.x < self.grid[0].len() as i32
            && pos.y >= 0
            && pos.y < self.grid.len() as i32
    }
}
#[derive(Clone, Copy, Debug)]
enum Direction {
    North,
    East,
    South,
    West,
}
#[derive(Clone, Copy, Debug)]
struct Guard {
    pos: Pos,
    direction: Direction,
}
impl Guard {
    fn get_facing_pos(&self) -> Pos {
        match self.direction {
            Direction::North => Pos {
                y: self.pos.y - 1,
                ..self.pos
            },
            Direction::East => Pos {
                x: self.pos.x + 1,
                ..self.pos
            },
            Direction::South => Pos {
                y: self.pos.y + 1,
                ..self.pos
            },
            Direction::West => Pos {
                x: self.pos.x - 1,
                ..self.pos
            },
        }
    }
    fn turn_right(&mut self) {
        self.direction = match self.direction {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        };
    }
    fn move_forward(&mut self) {
        self.pos = self.get_facing_pos();
    }
}
