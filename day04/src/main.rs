use anyhow::{anyhow, Result};
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> Result<()> {
    let input = read_input("input.txt")?;

    let mut game1 = Game::parse(&input)?;
    let result1 = game1.play().ok_or(anyhow!("no board wins!"))?;
    println!("{}", result1);

    let mut game2 = Game::parse(&input)?;
    let result2 = game2.play_to_lose().ok_or(anyhow!("no board wins!"))?;
    println!("{}", result2);

    Ok(())
}

#[derive(Clone, Debug)]
struct Number {
    value: i32,
    marked: bool,
}

impl Number {
    fn new(value: i32) -> Number {
        Number {
            value,
            marked: false,
        }
    }
}

#[derive(Clone, Debug)]
struct Board {
    last_number: i32,
    rows: Vec<Vec<Number>>,
}

impl Board {
    fn new(rows: Vec<Vec<Number>>) -> Board {
        Board {
            rows,
            last_number: 0,
        }
    }

    fn play<'a, I: IntoIterator<Item = &'a i32>>(&mut self, numbers: I) -> Option<usize> {
        for (i, n) in numbers.into_iter().enumerate() {
            self.last_number = *n;
            for r in self.rows.iter_mut() {
                mark_row(r, *n);
            }

            for r in &self.rows {
                if r.iter().all(|n| n.marked) {
                    return Some(i);
                }
            }

            for x in 0..self.rows.first()?.len() - 1 {
                let mut all = true;
                for r in &self.rows {
                    if !r.get(x)?.marked {
                        all = false;
                    }
                }
                if all {
                    return Some(i);
                }
            }
        }
        None
    }

    fn score(&self) -> i32 {
        self.last_number
            * self
                .rows
                .iter()
                .map(|r| r.iter().filter(|n| !n.marked).fold(0, |s, n| s + n.value))
                .fold(0, |s1, s2| s1 + s2)
    }
}

fn mark_row(row: &mut Vec<Number>, n: i32) {
    for number in row.iter_mut() {
        if number.value == n {
            number.marked = true;
        }
    }
}

#[derive(Debug)]
struct Game {
    numbers: Vec<i32>,
    boards: Vec<Board>,
}

impl Game {
    fn parse<'a, I: IntoIterator<Item = &'a String>>(input: I) -> Result<Self> {
        let mut iter = input.into_iter();
        let numbers = parse_numbers(iter.next().ok_or(anyhow!("empty input"))?)?;
        iter.next();

        let mut boards: Vec<Board> = vec![];
        loop {
            let rows = iter
                .by_ref()
                .take_while(|l| !l.is_empty())
                .map(|l| parse_line(l))
                .collect::<Result<Vec<Vec<Number>>>>()?;

            if rows.is_empty() {
                break;
            }

            boards.push(Board::new(rows));
        }

        Ok(Game { numbers, boards })
    }

    fn play(&mut self) -> Option<i32> {
        let mut winning_board = None;
        let mut winning_n = usize::MAX;

        for b in self.boards.iter_mut() {
            if let Some(n) = b.play(&self.numbers) {
                if n < winning_n {
                    winning_n = n;
                    winning_board = Some(b);
                }
            }
        }

        winning_board.map(|b| b.score())
    }

    fn play_to_lose(&mut self) -> Option<i32> {
        let mut losing_board = None;
        let mut losing_n = 0;

        for b in self.boards.iter_mut() {
            if let Some(n) = b.play(&self.numbers) {
                if n > losing_n {
                    losing_n = n;
                    losing_board = Some(b);
                }
            }
        }

        losing_board.map(|b| b.score())
    }
}

fn parse_line(input: &String) -> Result<Vec<Number>> {
    input.split_whitespace().map(|n| parse_number(n)).collect()
}

fn parse_numbers(input: &String) -> Result<Vec<i32>> {
    input.split(',').map(|n| parse_i32(n)).collect()
}

fn parse_number(input: &str) -> Result<Number> {
    let n = parse_i32(input)?;
    Ok(Number::new(n))
}

fn parse_i32(input: &str) -> Result<i32> {
    let n = input.parse()?;
    Ok(n)
}

fn read_input<P: AsRef<std::path::Path>>(path: P) -> Result<Vec<String>> {
    let file = File::open(path)?;
    BufReader::new(file).lines().map(|l| Ok(l?)).collect()
}

#[cfg(test)]
mod tests {
    use crate::{Board, Game, Number};

    #[test]
    fn test_game() {
        let nums = vec![
            7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24, 10, 16, 13, 6, 15, 25, 12, 22, 18, 20, 8, 19,
            3, 26, 1,
        ];
        let boards = vec![
            Board::new(vec![
                vec![
                    Number::new(22),
                    Number::new(13),
                    Number::new(17),
                    Number::new(11),
                    Number::new(0),
                ],
                vec![
                    Number::new(8),
                    Number::new(2),
                    Number::new(23),
                    Number::new(4),
                    Number::new(24),
                ],
                vec![
                    Number::new(21),
                    Number::new(9),
                    Number::new(14),
                    Number::new(16),
                    Number::new(7),
                ],
                vec![
                    Number::new(6),
                    Number::new(10),
                    Number::new(3),
                    Number::new(18),
                    Number::new(5),
                ],
                vec![
                    Number::new(1),
                    Number::new(12),
                    Number::new(20),
                    Number::new(15),
                    Number::new(19),
                ],
            ]),
            Board::new(vec![
                vec![
                    Number::new(3),
                    Number::new(15),
                    Number::new(0),
                    Number::new(2),
                    Number::new(22),
                ],
                vec![
                    Number::new(9),
                    Number::new(18),
                    Number::new(13),
                    Number::new(17),
                    Number::new(5),
                ],
                vec![
                    Number::new(19),
                    Number::new(8),
                    Number::new(7),
                    Number::new(25),
                    Number::new(23),
                ],
                vec![
                    Number::new(20),
                    Number::new(11),
                    Number::new(10),
                    Number::new(24),
                    Number::new(4),
                ],
                vec![
                    Number::new(14),
                    Number::new(21),
                    Number::new(16),
                    Number::new(12),
                    Number::new(6),
                ],
            ]),
            Board::new(vec![
                vec![
                    Number::new(14),
                    Number::new(21),
                    Number::new(17),
                    Number::new(24),
                    Number::new(4),
                ],
                vec![
                    Number::new(10),
                    Number::new(16),
                    Number::new(15),
                    Number::new(9),
                    Number::new(19),
                ],
                vec![
                    Number::new(18),
                    Number::new(8),
                    Number::new(23),
                    Number::new(26),
                    Number::new(20),
                ],
                vec![
                    Number::new(22),
                    Number::new(11),
                    Number::new(13),
                    Number::new(6),
                    Number::new(5),
                ],
                vec![
                    Number::new(2),
                    Number::new(0),
                    Number::new(12),
                    Number::new(3),
                    Number::new(7),
                ],
            ]),
        ];
        let mut game = Game {
            numbers: nums,
            boards,
        };

        assert_eq!(game.play(), Some(4512));
    }
}
