use anyhow::Result;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> Result<()> {
    let depths = read_depths("input.txt")?;
    println!("{}", count_increases(depths.iter().copied()));
    println!("{}", count_three_window_increases(&depths));
    Ok(())
}

fn count_three_window_increases(depths: &Vec<i32>) -> i32 {
    let sums = depths
        .iter()
        .zip(depths.iter().skip(1))
        .zip(depths.iter().skip(2))
        .map(|((n1, n2), n3)| n1 + n2 + n3);
    count_increases(sums)
}

fn count_increases<I: IntoIterator<Item = i32>>(ns: I) -> i32 {
    let mut prev = i32::MAX;
    let mut count = 0;
    for n in ns {
        if n > prev {
            count += 1;
        }
        prev = n;
    }
    count
}

fn read_depths<P: AsRef<std::path::Path>>(path: P) -> Result<Vec<i32>> {
    let file = File::open(path)?;
    BufReader::new(file)
        .lines()
        .map(|l| parse_depth(l?))
        .collect()
}

fn parse_depth(depth_str: String) -> Result<i32> {
    let depth = depth_str.parse()?;
    Ok(depth)
}
