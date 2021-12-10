use anyhow;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> anyhow::Result<()> {
    let input = read_input("input.txt")?;
    dbg!(syntax_error_score(&input));
    dbg!(autocomplete_score(&input));
    Ok(())
}

#[derive(Debug)]
enum ValidationErr {
    Corrupted(char),
    Incomplete(Vec<char>),
}

impl ValidationErr {
    fn score(&self) -> u64 {
        match self {
            Self::Corrupted(')') => 3,
            Self::Corrupted(']') => 57,
            Self::Corrupted('}') => 1197,
            Self::Corrupted('>') => 25137,
            Self::Corrupted(_) => 0,
            Self::Incomplete(cs) => cs.iter().fold(0, |s, c| {
                s * 5
                    + match c {
                        ')' => 1,
                        ']' => 2,
                        '}' => 3,
                        '>' => 4,
                        _ => 0,
                    }
            }),
        }
    }
}

fn syntax_error_score<'a, I: IntoIterator<Item = &'a String>>(input: I) -> u64 {
    input
        .into_iter()
        .map(|l| validate_line(l))
        .filter_map(|r| r.err())
        .map(|e| match e {
            ValidationErr::Corrupted(_) => e.score(),
            _ => 0,
        })
        .sum()
}

fn autocomplete_score<'a, I: IntoIterator<Item = &'a String>>(input: I) -> u64 {
    let mut scores = input
        .into_iter()
        .map(|l| validate_line(l))
        .filter_map(|r| match r {
            Err(e @ ValidationErr::Incomplete(_)) => Some(e.score()),
            _ => None,
        })
        .collect::<Vec<u64>>();
    scores.sort();
    scores[scores.len() / 2]
}

fn validate_line(s: &str) -> Result<(), ValidationErr> {
    let mut stack = vec![];

    for c in s.chars() {
        if c == '(' || c == '[' || c == '{' || c == '<' {
            stack.push(c);
        } else if let Some(&last) = stack.last() {
            if (c == ')' && last == '(')
                || (c == ']' && last == '[')
                || (c == '}' && last == '{')
                || (c == '>' && last == '<')
            {
                stack.pop();
            } else {
                return Err(ValidationErr::Corrupted(c));
            }
        } else {
            return Err(ValidationErr::Corrupted(c));
        }
    }

    if !stack.is_empty() {
        stack.reverse();
        return Err(ValidationErr::Incomplete(
            stack
                .iter()
                .map(|c| match c {
                    '(' => ')',
                    '[' => ']',
                    '{' => '}',
                    '<' => '>',
                    _ => *c,
                })
                .collect(),
        ));
    }

    Ok(())
}

fn read_input<P: AsRef<std::path::Path>>(path: P) -> anyhow::Result<Vec<String>> {
    let file = File::open(path)?;
    BufReader::new(file).lines().map(|l| Ok(l?)).collect()
}

#[cfg(test)]
mod tests {
    use crate::{autocomplete_score, syntax_error_score};

    #[test]
    fn test_scores() {
        let input = vec![
            "[({(<(())[]>[[{[]{<()<>>".to_string(),
            "[(()[<>])]({[<{<<[]>>(".to_string(),
            "{([(<{}[<>[]}>{[]{[(<()>".to_string(),
            "(((({<>}<{<{<>}{[]{[]{}".to_string(),
            "[[<[([]))<([[{}[[()]]]".to_string(),
            "[{[{({}]{}}([{[{{{}}([]".to_string(),
            "{<[[]]>}<{[{[{[]{()[[[]".to_string(),
            "[<(<(<(<{}))><([]([]()".to_string(),
            "<{([([[(<>()){}]>(<<{{".to_string(),
            "<{([{{}}[<[[[<>{}]]]>[]]".to_string(),
        ];

        assert_eq!(syntax_error_score(&input), 26397);
        assert_eq!(autocomplete_score(&input), 288957);
    }
}
