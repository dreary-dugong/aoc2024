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

fn parse(input: String) -> anyhow::Result<(Vec<Rule>, Vec<Vec<u32>>)> {
    let mut initer = input.split("\n\n");
    let rule_str = initer.next().unwrap();
    let rules = rule_str
        .lines()
        .map(|r| r.split("|"))
        .map(|mut iter| {
            let before = iter.next().unwrap().parse::<u32>().unwrap();
            let after = iter.next().unwrap().parse::<u32>().unwrap();
            Rule { before, after }
        })
        .collect::<Vec<Rule>>();
    let updates = initer
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            line.split(",")
                .map(|n| n.parse::<u32>().unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();

    Ok((rules, updates))
}

fn process(data: (Vec<Rule>, Vec<Vec<u32>>)) -> u32 {
    let (rules, updates) = data;

    // construct graph of page rules
    let mut pages = HashMap::new();
    for rule in rules {
        let before = pages.entry(rule.before).or_insert(Page::new(rule.before));
        before.comes_before.push(rule.after);

        let after = pages.entry(rule.after).or_insert(Page::new(rule.after));
        after.comes_after.push(rule.before);
    }

    // check updates one at a time
    let mut mid_sum = 0;
    'update_loop: for mut update in updates {
        let mid = update[update.len() / 2];

        let mut came_after: Vec<u32> = Vec::new(); // all pages that come after the current page in the current update
        while let Some(cur) = update.pop() {
            for successor in came_after.iter() {
                // search for contradictions
                if let Some(page) = pages.get(successor) {
                    if page.comes_before.contains(&cur) {
                        continue 'update_loop;
                    }
                }
            }
            came_after.push(cur);
        }

        mid_sum += mid;
    }

    mid_sum
}

struct Rule {
    before: u32,
    after: u32,
}
struct Page {
    id: u32,
    comes_before: Vec<u32>,
    comes_after: Vec<u32>,
}
impl Page {
    fn new(id: u32) -> Self {
        Page {
            id,
            comes_after: Default::default(),
            comes_before: Default::default(),
        }
    }
}
