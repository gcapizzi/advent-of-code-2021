use anyhow::{anyhow, Result};
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> Result<()> {
    let commands = read_commands("input.txt")?;
    let position = run_commands(commands);
    println!("{}", position.horizontal * position.depth);
    Ok(())
}

#[derive(Debug)]
enum Command {
    Forward(i32),
    Up(i32),
    Down(i32),
}

struct Position {
    horizontal: i32,
    depth: i32,
}

fn run_commands<I: IntoIterator<Item = Command>>(commands: I) -> Position {
    let (horizontal, depth, _) = commands
        .into_iter()
        .fold((0, 0, 0), |(h, d, a), c| match c {
            Command::Forward(n) => (h + n, d + a * n, a),
            Command::Up(n) => (h, d, a - n),
            Command::Down(n) => (h, d, a + n),
        });
    Position { horizontal, depth }
}

fn read_commands<P: AsRef<std::path::Path>>(path: P) -> Result<Vec<Command>> {
    let file = File::open(path)?;
    BufReader::new(file)
        .lines()
        .map(|l| parse_command(l?))
        .collect()
}

fn parse_command(cmd_string: String) -> Result<Command> {
    if let Some((cmd, val)) = cmd_string.split_once(' ') {
        match cmd {
            "forward" => Ok(Command::Forward(val.parse()?)),
            "up" => Ok(Command::Up(val.parse()?)),
            "down" => Ok(Command::Down(val.parse()?)),
            _ => Err(anyhow!("invalid command: {:?}", cmd_string)),
        }
    } else {
        Err(anyhow!("invalid command: {:?}", cmd_string))
    }
}
