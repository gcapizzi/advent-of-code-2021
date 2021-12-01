use anyhow::Result;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> Result<()> {
    let depths = read_depths("input.txt")?;
    println!("{}", count_increases(&depths));
    println!("{}", count_three_window_increases(&depths));
    Ok(())
}

fn count_three_window_increases(depths: &Vec<i32>) -> i32 {
    let mut previous_depth = i32::MAX;
    let mut count = 0;
    for ((d1, d2), d3) in depths
        .iter()
        .zip(depths.iter().skip(1))
        .zip(depths.iter().skip(2))
    {
        let sum = d1 + d2 + d3;
        if sum > previous_depth {
            count += 1;
        }
        previous_depth = sum;
    }
    count
}

fn count_increases(depths: &Vec<i32>) -> i32 {
    let mut previous_depth = i32::MAX;
    let mut count = 0;
    for depth in depths {
        if depth > &previous_depth {
            count += 1;
        }
        previous_depth = *depth;
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
