use anyhow::{anyhow, Result};
use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> Result<()> {
    let input = read_input("input.txt")?;
    dbg!(solve1(&input));
    dbg!(solve2(&input));
    Ok(())
}

fn solve1(heights: &Vec<Vec<u32>>) -> u32 {
    low_points(&heights).map(|(_, _, n)| *n + 1).sum()
}

fn solve2(heights: &Vec<Vec<u32>>) -> usize {
    let mut lps = low_points(&heights)
        .map(|p| basin(&heights, p).count())
        .collect::<Vec<usize>>();
    lps.sort();
    lps.reverse();
    lps.iter().take(3).product()
}

fn basin<'a>(
    heights: &'a Vec<Vec<u32>>,
    point: (usize, usize, &'a u32),
) -> impl Iterator<Item = &'a u32> {
    let mut result = HashSet::new();
    grow_basin(&mut result, heights, point);
    result.into_iter().map(|(_, _, h)| h)
}

fn grow_basin<'a>(
    basin: &mut HashSet<(usize, usize, &'a u32)>,
    heights: &'a Vec<Vec<u32>>,
    point: (usize, usize, &'a u32),
) {
    basin.insert(point);
    for p @ (_, _, &h) in neighbours(heights, point.0, point.1) {
        if h != 9 && basin.insert(p) {
            grow_basin(basin, heights, p)
        }
    }
}

fn low_points(heights: &Vec<Vec<u32>>) -> impl Iterator<Item = (usize, usize, &u32)> + '_ {
    enumerate_grid(heights).filter(|(i, j, _)| is_low_point(heights, *i, *j))
}

fn enumerate_grid<T>(grid: &Vec<Vec<T>>) -> impl Iterator<Item = (usize, usize, &T)> + '_ {
    (0..grid.len())
        .flat_map(|i| (0..grid[i].len()).map(move |j| (i, j)))
        .map(|(i, j)| (i, j, &grid[i][j]))
}

fn is_low_point(heights: &Vec<Vec<u32>>, i: usize, j: usize) -> bool {
    neighbours(heights, i, j).all(|(_, _, n)| n > &heights[i][j])
}

fn neighbours(
    heights: &Vec<Vec<u32>>,
    i: usize,
    j: usize,
) -> impl Iterator<Item = (usize, usize, &u32)> + '_ {
    let mut result = vec![];

    if i > 0 {
        result.push((i - 1, j, &heights[i - 1][j]));
    };

    if i < heights.len() - 1 {
        result.push((i + 1, j, &heights[i + 1][j]))
    };

    if j > 0 {
        result.push((i, j - 1, &heights[i][j - 1]));
    };

    if j < heights[i].len() - 1 {
        result.push((i, j + 1, &heights[i][j + 1]));
    };

    result.into_iter()
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

#[cfg(test)]
mod tests {
    use crate::{solve1, solve2};

    #[test]
    fn test_solution() {
        let heights = vec![
            vec![2, 1, 9, 9, 9, 4, 3, 2, 1, 0],
            vec![3, 9, 8, 7, 8, 9, 4, 9, 2, 1],
            vec![9, 8, 5, 6, 7, 8, 9, 8, 9, 2],
            vec![8, 7, 6, 7, 8, 9, 6, 7, 8, 9],
            vec![9, 8, 9, 9, 9, 6, 5, 6, 7, 8],
        ];
        assert_eq!(solve1(&heights), 15);
        assert_eq!(solve2(&heights), 1134);
    }
}
