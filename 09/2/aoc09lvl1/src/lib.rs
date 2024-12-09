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

fn parse(input: String) -> anyhow::Result<(Vec<FreeSpace>, Vec<FileSpace>)> {
    let mut free_space = Vec::new();
    let mut files = Vec::new();

    let mut disk_index = 0;
    input
        .strip_suffix("\n")
        .unwrap()
        .chars()
        .collect::<Vec<char>>()
        .chunks(2)
        .enumerate()
        .for_each(|(id, chs)| {
            let file = chs[0].to_digit(10).unwrap() as usize;
            files.push(FileSpace {
                id,
                pos: disk_index,
                length: file,
            });
            disk_index += file;
            if chs.len() > 1 {
                let free = chs[1].to_digit(10).unwrap() as usize;
                free_space.push(FreeSpace {
                    pos: disk_index,
                    length: free,
                });
                disk_index += free;
            }
        });

    Ok((free_space, files))
}

fn process(data: (Vec<FreeSpace>, Vec<FileSpace>)) -> u64 {
    let (mut free_space, mut files) = data;

    // for each file, check for free space
    'file_loop: for file in files.iter_mut().rev() {
        for space in free_space.iter_mut() {
            if space.pos > file.pos {
                continue 'file_loop;
            }

            // if the file fits, move it
            if space.length >= file.length {
                file.pos = space.pos;
                space.length -= file.length;
                space.pos += file.length;
                continue 'file_loop;
            }
        }
    }

    calc_checksum(&files)
}

fn calc_checksum(files: &[FileSpace]) -> u64 {
    files
        .iter()
        .map(|file| (file.id as u64, file.pos as u64, file.length as u64))
        .map(|(id, pos, len)| id * (len.pow(2) + 2 * pos * len - len) / 2)
        .sum()
}

#[derive(Debug)]
struct FreeSpace {
    pos: usize,
    length: usize,
}
#[derive(Debug)]
struct FileSpace {
    id: usize,
    pos: usize,
    length: usize,
}
