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

pub fn run(cfg: Config) -> anyhow::Result<i64> {
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

fn parse(input: String) -> anyhow::Result<Vec<Equation>> {
    Ok(input
        .lines()
        .map(|line| {
            let mut iter = line.split(":");
            let test_value = iter.next().unwrap().parse::<i64>().unwrap();
            let coefficients = iter
                .next()
                .unwrap()
                .split_whitespace()
                .map(|val| val.parse::<i64>().unwrap())
                .collect();
            Equation {
                test_value,
                coefficients,
            }
        })
        .collect())
}

fn process(data: Vec<Equation>) -> i64 {
    let mut output = 0;
    for equation in data {
        if get_solution_count(
            &equation.coefficients[1..],
            equation.test_value,
            equation.coefficients[0],
        ) > 0
        {
            output += equation.test_value;
        }
    }
    output
}

fn get_solution_count(coefficients: &[i64], test_value: i64, running_total: i64) -> i64 {
    // base case: no more coefficients ot operate on
    if coefficients.is_empty() {
        // we made the test value, so this way was a valid solution
        if running_total == test_value {
            return 1;
        } else {
            // we didn't make it :(
            return 0;
        }
    }
    // calculate what the running total would be in case of a concat operation
    let concated = running_total * 10i64.pow(i64::ilog10(coefficients[0]) + 1) + coefficients[0];

    // recursive case: spin up a new stack frame for each operation
    get_solution_count(
        &coefficients[1..],
        test_value,
        running_total * coefficients[0],
    ) + get_solution_count(
        &coefficients[1..],
        test_value,
        running_total + coefficients[0],
    ) + get_solution_count(&coefficients[1..], test_value, concated)
}

#[derive(Debug, Clone)]
struct Equation {
    test_value: i64,
    coefficients: Vec<i64>,
}
