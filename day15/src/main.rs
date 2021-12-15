use anyhow::{anyhow, Result};
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> Result<()> {
    let input = read_input("input.txt")?;
    dbg!(lowest_risk(&input))?;
    let big_input = expand_grid(&input);
    dbg!(lowest_risk(&big_input))?;
    Ok(())
}

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
struct Point {
    coords: (usize, usize),
    distance: u32,
}

impl Ord for Point {
    fn cmp(&self, other: &Self) -> Ordering {
        other.distance.cmp(&self.distance)
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn lowest_risk(grid: &Vec<Vec<u32>>) -> Result<u32> {
    let mut unvisited = BinaryHeap::new();
    unvisited.push(Point {
        coords: (0, 0),
        distance: 0,
    });
    let mut previous = HashMap::<(usize, usize), (usize, usize)>::new();
    let mut distance = enumerate_grid(&grid)
        .map(|c| (c, u32::MAX))
        .collect::<HashMap<(usize, usize), u32>>();
    distance.insert((0, 0), 0);
    let target = (grid.len() - 1, grid[0].len() - 1);

    while let Some(Point { coords, .. }) = unvisited.pop() {
        if coords == target {
            break;
        }

        for n @ (i, j) in neighbours(grid, coords) {
            let dist = distance.get(&coords).ok_or(anyhow!(
                "could not find distance for unvisited point {:?}",
                coords
            ))?;
            let n_dist = *distance
                .get(&n)
                .ok_or(anyhow!("could not find distance for neighbour {:?}", n))?;
            let new_dist = dist + grid[i][j];
            if new_dist < n_dist {
                distance.insert(n, new_dist);
                unvisited.push(Point {
                    coords: n,
                    distance: new_dist,
                });
                previous.insert(n, coords);
            }
        }
    }

    let mut path = vec![];
    let mut p = Some(&target);
    while let Some(pp) = p {
        path.push(pp);
        p = previous.get(pp);
    }
    path.reverse();

    let cost = path.iter().map(|(i, j)| grid[*i][*j]).sum::<u32>();
    Ok(cost - grid[0][0])
}

fn expand_grid(grid: &Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    let expanded_rows = grid.iter().map(expand_row).collect::<Vec<Vec<u32>>>();
    let mut result = expanded_rows.clone();
    for n in 1..5 {
        result.extend(expanded_rows.iter().map(|r| inc_digit_row(r, n)))
    }
    result
}

fn expand_row(row: &Vec<u32>) -> Vec<u32> {
    let mut result = row.clone();
    for n in 1..5 {
        result.extend(row.iter().map(|d| inc_digit(*d, n)))
    }
    result
}

fn inc_digit_row(row: &Vec<u32>, n: u32) -> Vec<u32> {
    row.iter().map(|d| inc_digit(*d, n)).collect::<Vec<u32>>()
}

fn inc_digit(d: u32, n: u32) -> u32 {
    let mut s = d + n;
    if s > 9 {
        s -= 9;
    }
    s
}

fn enumerate_grid<T>(grid: &Vec<Vec<T>>) -> impl Iterator<Item = (usize, usize)> + '_ {
    (0..grid.len())
        .flat_map(|i| (0..grid[i].len()).map(move |j| (i, j)))
        .map(|(i, j)| (i, j))
}

fn read_input<P: AsRef<std::path::Path>>(path: P) -> Result<Vec<Vec<u32>>> {
    let file = File::open(path)?;
    BufReader::new(file).lines().map(|l| digits(l?)).collect()
}

fn digits(s: String) -> Result<Vec<u32>> {
    s.chars()
        .map(|c| c.to_digit(10))
        .collect::<Option<Vec<u32>>>()
        .ok_or(anyhow!("invalid string"))
}

fn neighbours(
    grid: &Vec<Vec<u32>>,
    p: (usize, usize),
) -> impl Iterator<Item = (usize, usize)> + '_ {
    let mut result = vec![];
    let (i, j) = p;

    if i > 0 {
        result.push((i - 1, j));
    };

    if i < grid.len() - 1 {
        result.push((i + 1, j))
    };

    if j > 0 {
        result.push((i, j - 1));
    };

    if j < grid[i].len() - 1 {
        result.push((i, j + 1));
    };

    result.into_iter()
}

#[cfg(test)]
mod tests {
    use crate::{expand_grid, lowest_risk};
    use anyhow::Result;

    #[test]
    fn test_lowest_risk_path() -> Result<()> {
        let grid = vec![
            vec![1, 1, 6, 3, 7, 5, 1, 7, 4, 2],
            vec![1, 3, 8, 1, 3, 7, 3, 6, 7, 2],
            vec![2, 1, 3, 6, 5, 1, 1, 3, 2, 8],
            vec![3, 6, 9, 4, 9, 3, 1, 5, 6, 9],
            vec![7, 4, 6, 3, 4, 1, 7, 1, 1, 1],
            vec![1, 3, 1, 9, 1, 2, 8, 1, 3, 7],
            vec![1, 3, 5, 9, 9, 1, 2, 4, 2, 1],
            vec![3, 1, 2, 5, 4, 2, 1, 6, 3, 9],
            vec![1, 2, 9, 3, 1, 3, 8, 5, 2, 1],
            vec![2, 3, 1, 1, 9, 4, 4, 5, 8, 1],
        ];
        assert_eq!(lowest_risk(&grid)?, 40);

        let big_grid = expand_grid(&grid);
        assert_eq!(lowest_risk(&big_grid)?, 315);

        Ok(())
    }
}
