use anyhow::{anyhow, Result};
use std::collections::HashSet;
use std::fmt;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> Result<()> {
    let input = read_input("input.txt")?;
    let mut sheet = parse_sheet(&input)?;

    sheet.apply_next_fold()?;
    dbg!(sheet.dots.len());

    sheet.apply_all_remaining_folds();
    println!("{}", sheet);

    Ok(())
}

#[derive(Debug, Hash, Eq, PartialEq)]
struct Dot {
    x: i32,
    y: i32,
}

impl Dot {
    pub fn fold(&self, fold: &Fold) -> Dot {
        match fold {
            Fold::X(x) => Dot {
                x: if self.x > *x {
                    x - (self.x - x)
                } else {
                    self.x
                },
                y: self.y,
            },
            Fold::Y(y) => Dot {
                x: self.x,
                y: if self.y > *y {
                    y - (self.y - y)
                } else {
                    self.y
                },
            },
        }
    }
}

#[derive(Debug)]
enum Fold {
    X(i32),
    Y(i32),
}

#[derive(Debug)]
struct Sheet {
    dots: HashSet<Dot>,
    folds: Vec<Fold>,
}

impl Sheet {
    pub fn apply_next_fold(&mut self) -> Result<()> {
        if self.folds.is_empty() {
            return Err(anyhow!("no folds to apply"));
        }

        let fold = self.folds.remove(0);
        self.apply_fold(&fold);
        Ok(())
    }

    pub fn apply_all_remaining_folds(&mut self) {
        while self.apply_next_fold().is_ok() {}
    }

    fn apply_fold(&mut self, fold: &Fold) {
        self.dots = self.dots.iter().map(|d| d.fold(fold)).collect();
    }
}

impl fmt::Display for Sheet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let ymax = self.dots.iter().map(|d| d.y).max().unwrap_or(0);
        let xmax = self.dots.iter().map(|d| d.x).max().unwrap_or(0);

        for y in 0..ymax + 1 {
            for x in 0..xmax + 1 {
                if self.dots.contains(&Dot { x, y }) {
                    write!(f, "#")?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f, "")?;
        }

        Ok(())
    }
}

fn parse_sheet(lines: &Vec<String>) -> Result<Sheet> {
    let mut chunks = lines.splitn(2, |l| l.is_empty());
    let dots = chunks
        .next()
        .unwrap()
        .iter()
        .map(parse_dot)
        .collect::<Result<HashSet<Dot>>>()?;
    let folds = chunks
        .next()
        .unwrap()
        .iter()
        .map(parse_fold)
        .collect::<Result<Vec<Fold>>>()?;
    Ok(Sheet { dots, folds })
}

fn parse_dot(s: &String) -> Result<Dot> {
    let (x, y) = s
        .split_once(',')
        .ok_or(anyhow!("invalid dot string: {:?}", s))?;
    Ok(Dot {
        x: x.parse()?,
        y: y.parse()?,
    })
}

fn parse_fold(s: &String) -> Result<Fold> {
    let fs = s
        .split_whitespace()
        .last()
        .ok_or(anyhow!("invalid fold string: {:?}", s))?;
    let (a, n) = fs
        .split_once('=')
        .ok_or(anyhow!("invalid fold string: {:?}", s))?;
    match a {
        "x" => Ok(Fold::X(n.parse()?)),
        "y" => Ok(Fold::Y(n.parse()?)),
        _ => Err(anyhow!("invalid fold string: {:?}", s)),
    }
}

fn read_input<P: AsRef<std::path::Path>>(path: P) -> Result<Vec<String>> {
    let file = File::open(path)?;
    BufReader::new(file).lines().map(|l| Ok(l?)).collect()
}

#[cfg(test)]
mod tests {
    use crate::{Dot, Fold, Sheet};
    use anyhow::Result;

    #[test]
    fn test_dot_fold() {
        assert_eq!(Dot { x: 9, y: 10 }.fold(&Fold::Y(7)), Dot { x: 9, y: 4 });
    }

    #[test]
    fn test_folding() -> Result<()> {
        let mut sheet = Sheet {
            dots: vec![
                Dot { x: 6, y: 10 },
                Dot { x: 0, y: 14 },
                Dot { x: 9, y: 10 },
                Dot { x: 0, y: 3 },
                Dot { x: 10, y: 4 },
                Dot { x: 4, y: 11 },
                Dot { x: 6, y: 0 },
                Dot { x: 6, y: 12 },
                Dot { x: 4, y: 1 },
                Dot { x: 0, y: 13 },
                Dot { x: 10, y: 12 },
                Dot { x: 3, y: 4 },
                Dot { x: 3, y: 0 },
                Dot { x: 8, y: 4 },
                Dot { x: 1, y: 10 },
                Dot { x: 2, y: 14 },
                Dot { x: 8, y: 10 },
                Dot { x: 9, y: 0 },
            ]
            .into_iter()
            .collect(),
            folds: vec![Fold::Y(7), Fold::X(5)],
        };

        sheet.apply_next_fold()?;
        assert_eq!(sheet.dots.len(), 17);

        sheet.apply_all_remaining_folds();
        println!("{}", sheet);

        Ok(())
    }
}
