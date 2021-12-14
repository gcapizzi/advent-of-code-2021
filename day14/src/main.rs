use anyhow::{anyhow, Result};
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> Result<()> {
    let input = read_input("input.txt")?;
    let manual = parse_manual(&input)?;
    dbg!(manual.run(10)?);
    dbg!(manual.run(40)?);
    Ok(())
}

struct Manual {
    template: String,
    rules: HashMap<(char, char), char>,
}

impl Manual {
    fn run(&self, steps: usize) -> Result<u64> {
        let mut char_counts = HashMap::new();
        for c in self.template.chars() {
            *char_counts.entry(c).or_insert(0) += 1;
        }

        let mut pair_counts = HashMap::new();
        for p in self.template.chars().zip(self.template.chars().skip(1)) {
            *pair_counts.entry(p).or_insert(0) += 1;
        }

        for _ in 0..steps {
            for ((l, r), c) in pair_counts.clone().iter().filter(|(_, v)| *v > &0) {
                let i = self.rules.get(&(*l, *r)).ok_or(anyhow!("invalid input"))?;

                *pair_counts.entry((*l, *r)).or_insert(0) -= c;
                *pair_counts.entry((*l, *i)).or_insert(0) += c;
                *pair_counts.entry((*i, *r)).or_insert(0) += c;
                *char_counts.entry(*i).or_insert(0) += c;
            }
        }

        let max = char_counts.values().max().ok_or(anyhow!("no max"))?;
        let min = char_counts.values().min().ok_or(anyhow!("no min"))?;
        Ok(max - min)
    }
}

fn read_input<P: AsRef<std::path::Path>>(path: P) -> Result<Vec<String>> {
    let file = File::open(path)?;
    BufReader::new(file).lines().map(|l| Ok(l?)).collect()
}

fn parse_manual(lines: &Vec<String>) -> Result<Manual> {
    let mut chunks = lines.splitn(2, |l| l.is_empty());
    let template = chunks
        .next()
        .ok_or(anyhow!("invalid input"))?
        .first()
        .ok_or(anyhow!("invalid input"))?
        .to_string();
    let rules = chunks
        .next()
        .ok_or(anyhow!("invalid input"))?
        .iter()
        .map(parse_rule)
        .collect::<Result<HashMap<(char, char), char>>>()?;
    Ok(Manual { template, rules })
}

fn parse_rule(s: &String) -> Result<((char, char), char)> {
    let (pair_str, ins_str) = s
        .split_once(" -> ")
        .ok_or(anyhow!("invalid rule string: {:?}", s))?;
    let left = pair_str
        .chars()
        .nth(0)
        .ok_or(anyhow!("invalid rule string: {:?}", s))?;
    let right = pair_str
        .chars()
        .nth(1)
        .ok_or(anyhow!("invalid rule string: {:?}", s))?;
    let ins = ins_str
        .chars()
        .nth(0)
        .ok_or(anyhow!("invalid rule string: {:?}", s))?;
    Ok(((left, right), ins))
}

#[cfg(test)]
mod tests {
    use crate::Manual;
    use anyhow::Result;
    use std::collections::HashMap;

    #[test]
    fn test() -> Result<()> {
        let manual = Manual {
            template: "NNCB".to_string(),
            rules: HashMap::from([
                (('C', 'H'), 'B'),
                (('H', 'H'), 'N'),
                (('C', 'B'), 'H'),
                (('N', 'H'), 'C'),
                (('H', 'B'), 'C'),
                (('H', 'C'), 'B'),
                (('H', 'N'), 'C'),
                (('N', 'N'), 'C'),
                (('B', 'H'), 'H'),
                (('N', 'C'), 'B'),
                (('N', 'B'), 'B'),
                (('B', 'N'), 'B'),
                (('B', 'B'), 'N'),
                (('B', 'C'), 'B'),
                (('C', 'C'), 'N'),
                (('C', 'N'), 'C'),
            ]),
        };

        assert_eq!(manual.run(10)?, 1588);
        assert_eq!(manual.run(40)?, 2188189693529);

        Ok(())
    }
}
