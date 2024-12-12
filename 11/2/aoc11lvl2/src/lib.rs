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

pub fn run(cfg: Config) -> anyhow::Result<usize> {
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

fn parse(input: String) -> anyhow::Result<Stones> {
    Ok(Stones {
        stones: input
            .split_whitespace()
            .map(|s| s.parse::<u64>().unwrap())
            .collect(),
    })
}

fn process(mut stones: Stones) -> usize {
    stones.count_after_blinks(75) as usize
}

#[derive(Debug, Clone)]
struct Stones {
    stones: Vec<u64>,
}
impl Stones {
    fn count_after_blinks(&mut self, count: u32) -> u64 {
        fn rec_blink(stone: u64, steps: u32, cache: &mut HashMap<(u64, u32), u64>) -> u64 {
            // check memo
            if let Some(val) = cache.get(&(stone, steps)) {
                return *val;
            }

            // otherwise, figure out the answer recursively
            let ans;
            // base case
            if steps == 0 {
                ans = 1;
            } else if stone == 0 {
                ans = rec_blink(1, steps - 1, cache);
            } else if (stone.ilog10() + 1) % 2 == 0 {
                let num_digits = stone.ilog10() + 1;
                let left = stone / 10u64.pow(num_digits / 2);
                let right = stone - (left * 10u64.pow(num_digits / 2));
                ans = rec_blink(left, steps - 1, cache) + rec_blink(right, steps - 1, cache);
            } else {
                ans = rec_blink(stone * 2024, steps - 1, cache);
            }

            // update memo
            cache.insert((stone, steps), ans);

            ans
        }

        let mut cache: HashMap<(u64, u32), u64> = HashMap::new();

        self.stones
            .iter()
            .map(|stone| rec_blink(*stone, count, &mut cache))
            .sum()
    }
}
