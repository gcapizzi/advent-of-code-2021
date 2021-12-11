use anyhow::{anyhow, Result};
use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> Result<()> {
    let mut input = read_input("input.txt")?;

    let mut n = 0;
    for _ in 0..100 {
        n += evolve(&mut input);
    }
    dbg!(n);

    let mut i = 101;
    while evolve(&mut input) != 100 {
        i += 1;
    }
    dbg!(i);

    Ok(())
}

fn evolve(energies: &mut Vec<Vec<u32>>) -> usize {
    for i in 0..energies.len() {
        for j in 0..energies[i].len() {
            energies[i][j] += 1;
        }
    }

    let mut flashes: HashSet<(usize, usize)> = HashSet::new();
    loop {
        let mut flashed = false;

        for i in 0..energies.len() {
            for j in 0..energies[i].len() {
                if !flashes.contains(&(i, j)) && energies[i][j] > 9 {
                    flashes.insert((i, j));
                    flashed = true;
                    flash(energies, i, j);
                }
            }
        }

        if !flashed {
            break;
        }
    }

    for (i, j) in &flashes {
        energies[*i][*j] = 0;
    }

    flashes.len()
}

fn flash(energies: &mut Vec<Vec<u32>>, i: usize, j: usize) {
    if j < energies[i].len() - 1 {
        energies[i][j + 1] += 1;
    }
    if j > 0 {
        energies[i][j - 1] += 1;
    }

    if i > 0 {
        energies[i - 1][j] += 1;
        if j < energies[i].len() - 1 {
            energies[i - 1][j + 1] += 1;
        }
        if j > 0 {
            energies[i - 1][j - 1] += 1;
        }
    }

    if i < energies.len() - 1 {
        energies[i + 1][j] += 1;
        if j < energies[i].len() - 1 {
            energies[i + 1][j + 1] += 1;
        }
        if j > 0 {
            energies[i + 1][j - 1] += 1;
        }
    }
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
    use crate::evolve;

    #[test]
    fn test_evolve() {
        let mut energies = vec![
            vec![5, 4, 8, 3, 1, 4, 3, 2, 2, 3],
            vec![2, 7, 4, 5, 8, 5, 4, 7, 1, 1],
            vec![5, 2, 6, 4, 5, 5, 6, 1, 7, 3],
            vec![6, 1, 4, 1, 3, 3, 6, 1, 4, 6],
            vec![6, 3, 5, 7, 3, 8, 5, 4, 7, 8],
            vec![4, 1, 6, 7, 5, 2, 4, 6, 4, 5],
            vec![2, 1, 7, 6, 8, 4, 1, 7, 2, 1],
            vec![6, 8, 8, 2, 8, 8, 1, 1, 3, 4],
            vec![4, 8, 4, 6, 8, 4, 8, 5, 5, 4],
            vec![5, 2, 8, 3, 7, 5, 1, 5, 2, 6],
        ];

        assert_eq!(evolve(&mut energies), 0);
        assert_eq!(
            energies,
            vec![
                vec![6, 5, 9, 4, 2, 5, 4, 3, 3, 4],
                vec![3, 8, 5, 6, 9, 6, 5, 8, 2, 2],
                vec![6, 3, 7, 5, 6, 6, 7, 2, 8, 4],
                vec![7, 2, 5, 2, 4, 4, 7, 2, 5, 7],
                vec![7, 4, 6, 8, 4, 9, 6, 5, 8, 9],
                vec![5, 2, 7, 8, 6, 3, 5, 7, 5, 6],
                vec![3, 2, 8, 7, 9, 5, 2, 8, 3, 2],
                vec![7, 9, 9, 3, 9, 9, 2, 2, 4, 5],
                vec![5, 9, 5, 7, 9, 5, 9, 6, 6, 5],
                vec![6, 3, 9, 4, 8, 6, 2, 6, 3, 7],
            ],
        );
        assert_eq!(evolve(&mut energies), 35);
        assert_eq!(
            energies,
            vec![
                vec![8, 8, 0, 7, 4, 7, 6, 5, 5, 5],
                vec![5, 0, 8, 9, 0, 8, 7, 0, 5, 4],
                vec![8, 5, 9, 7, 8, 8, 9, 6, 0, 8],
                vec![8, 4, 8, 5, 7, 6, 9, 6, 0, 0],
                vec![8, 7, 0, 0, 9, 0, 8, 8, 0, 0],
                vec![6, 6, 0, 0, 0, 8, 8, 9, 8, 9],
                vec![6, 8, 0, 0, 0, 0, 5, 9, 4, 3],
                vec![0, 0, 0, 0, 0, 0, 7, 4, 5, 6],
                vec![9, 0, 0, 0, 0, 0, 0, 8, 7, 6],
                vec![8, 7, 0, 0, 0, 0, 6, 8, 4, 8],
            ],
        );

        let mut n = 35;
        for _ in 0..8 {
            n += evolve(&mut energies);
        }
        assert_eq!(n, 204);
        assert_eq!(
            energies,
            vec![
                vec![0, 4, 8, 1, 1, 1, 2, 9, 7, 6],
                vec![0, 0, 3, 1, 1, 1, 2, 0, 0, 9],
                vec![0, 0, 4, 1, 1, 1, 2, 5, 0, 4],
                vec![0, 0, 8, 1, 1, 1, 1, 4, 0, 6],
                vec![0, 0, 9, 9, 1, 1, 1, 3, 0, 6],
                vec![0, 0, 9, 3, 5, 1, 1, 2, 3, 3],
                vec![0, 4, 4, 2, 3, 6, 1, 1, 3, 0],
                vec![5, 5, 3, 2, 2, 5, 2, 3, 5, 0],
                vec![0, 5, 3, 2, 2, 5, 0, 6, 0, 0],
                vec![0, 0, 3, 2, 2, 4, 0, 0, 0, 0],
            ],
        );
    }
}
