use anyhow::{anyhow, Result};
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> Result<()> {
    let input = read_input("input.txt")?;
    let segments = parse_segments(&input)?;

    let result = solve(&segments);
    println!("{}", result);
    Ok(())
}

#[derive(Eq, PartialEq, Debug, Hash)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Segment {
    from: Point,
    to: Point,
}

impl Segment {
    fn points(&self) -> Vec<Point> {
        let xs: Vec<i32> = if self.from.x < self.to.x {
            (self.from.x..self.to.x + 1).collect()
        } else {
            (self.to.x..self.from.x + 1).rev().collect()
        };
        let ys: Vec<i32> = if self.from.y < self.to.y {
            (self.from.y..self.to.y + 1).collect()
        } else {
            (self.to.y..self.from.y + 1).rev().collect()
        };

        if self.from.x == self.to.x {
            ys.iter()
                .map(|y| Point {
                    x: self.from.x,
                    y: *y,
                })
                .collect()
        } else if self.from.y == self.to.y {
            xs.iter()
                .map(|x| Point {
                    x: *x,
                    y: self.from.y,
                })
                .collect()
        } else if (self.from.x - self.to.x).abs() == (self.from.y - self.to.y).abs() {
            xs.iter()
                .zip(ys.iter())
                .map(|(x, y)| Point { x: *x, y: *y })
                .collect()
        } else {
            vec![]
        }
    }
}

fn solve<'a, I: IntoIterator<Item = &'a Segment>>(segments: I) -> usize {
    let mut counts = HashMap::new();
    for s in segments {
        for p in s.points() {
            *counts.entry(p).or_insert(0) += 1;
        }
    }

    counts.iter().filter(|(_, &v)| v >= 2).count()
}

fn parse_segments<'a, I: IntoIterator<Item = &'a String>>(input: I) -> Result<Vec<Segment>> {
    input.into_iter().map(|l| parse_segment(l)).collect()
}

fn parse_segment(s: &str) -> Result<Segment> {
    let (from, to) = s
        .split_once(" -> ")
        .ok_or(anyhow!("invalid segment: {}", s))?;
    Ok(Segment {
        from: parse_point(from)?,
        to: parse_point(to)?,
    })
}

fn parse_point(s: &str) -> Result<Point> {
    let (x, y) = s.split_once(',').ok_or(anyhow!("invalid point: {}", s))?;
    Ok(Point {
        x: x.parse()?,
        y: y.parse()?,
    })
}

fn read_input<P: AsRef<std::path::Path>>(path: P) -> Result<Vec<String>> {
    let file = File::open(path)?;
    BufReader::new(file).lines().map(|l| Ok(l?)).collect()
}

#[cfg(test)]
mod tests {
    use crate::{solve, Point, Segment};

    #[test]
    fn test_example() {
        let segments = vec![
            Segment {
                from: Point { x: 0, y: 9 },
                to: Point { x: 5, y: 9 },
            },
            Segment {
                from: Point { x: 8, y: 0 },
                to: Point { x: 0, y: 8 },
            },
            Segment {
                from: Point { x: 9, y: 4 },
                to: Point { x: 3, y: 4 },
            },
            Segment {
                from: Point { x: 2, y: 2 },
                to: Point { x: 2, y: 1 },
            },
            Segment {
                from: Point { x: 7, y: 0 },
                to: Point { x: 7, y: 4 },
            },
            Segment {
                from: Point { x: 6, y: 4 },
                to: Point { x: 2, y: 0 },
            },
            Segment {
                from: Point { x: 0, y: 9 },
                to: Point { x: 2, y: 9 },
            },
            Segment {
                from: Point { x: 3, y: 4 },
                to: Point { x: 1, y: 4 },
            },
            Segment {
                from: Point { x: 0, y: 0 },
                to: Point { x: 8, y: 8 },
            },
            Segment {
                from: Point { x: 5, y: 5 },
                to: Point { x: 8, y: 2 },
            },
        ];
        assert_eq!(solve(&segments), 12);
    }
}
