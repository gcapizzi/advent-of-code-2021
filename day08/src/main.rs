use anyhow::{anyhow, Result};
use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> Result<()> {
    let digits = parse_digits("abcefg cf acdeg acdfg bcdf abdfg abdefg acf abcdefg abcdfg")
        .into_iter()
        .collect();
    let input = read_input("input.txt")?;
    let entries = parse_entries(input).ok_or(anyhow!("empty input!"))?;
    let solution_one = solve_one(&entries).ok_or(anyhow!("empty input!"))?;
    dbg!(solution_one);
    let solution_two = solve_two(digits, &entries).ok_or(anyhow!("empty input!"))?;
    dbg!(solution_two);
    Ok(())
}

type Entry = (Vec<String>, Vec<String>);

fn solve_one(entries: &Vec<Entry>) -> Option<usize> {
    entries
        .iter()
        .map(|(_, o)| {
            o.iter()
                .filter(|d| d.len() == 2 || d.len() == 3 || d.len() == 4 || d.len() == 7)
                .count()
        })
        .reduce(|s, c| s + c)
}

fn solve_two(digits: Vec<String>, entries: &Vec<Entry>) -> Option<usize> {
    let os = entries
        .iter()
        .map(|e| solve_two_entry(&digits, e))
        .collect::<Option<Vec<usize>>>()?;
    Some(os.iter().fold(0, |s, n| s + *n))
}

fn solve_two_entry(digits: &Vec<String>, entry: &Entry) -> Option<usize> {
    let mappings = ('a'..'h')
        .permutations(7)
        .map(|p| ('a'..'h').zip(p).collect::<HashMap<char, char>>())
        .collect::<Vec<HashMap<char, char>>>();
    let (patterns, output) = entry;
    let mapping = mappings.iter().find(|m| {
        apply_mapping(patterns, m)
            .iter()
            .collect::<HashSet<&String>>()
            == digits.iter().collect::<HashSet<&String>>()
    })?;
    apply_mapping(output, mapping)
        .iter()
        .map(|s| digits.iter().position(|d| d == s).unwrap())
        .zip((0..output.len()).rev())
        .map(|(d, e)| d * 10_usize.pow(e.try_into().unwrap()))
        .reduce(|s, n| s + n)
}

fn apply_mapping<'a, I: IntoIterator<Item = &'a String>>(
    patterns: I,
    mapping: &HashMap<char, char>,
) -> Vec<String> {
    patterns
        .into_iter()
        .map(|p| {
            let mut np = p
                .chars()
                .map(|d| *mapping.get(&d).unwrap())
                .collect::<Vec<char>>();
            np.sort();
            np.iter().collect()
        })
        .collect()
}

fn parse_entries(ss: Vec<String>) -> Option<Vec<Entry>> {
    ss.iter().map(parse_entry).collect::<Option<Vec<Entry>>>()
}

fn parse_entry(s: &String) -> Option<Entry> {
    let (signal_patterns_str, output_str) = s.split_once('|')?;
    let signal_patterns = parse_digits(signal_patterns_str).into_iter().collect();
    let output = parse_digits(output_str);
    Some((signal_patterns, output))
}

fn parse_digits(s: &str) -> Vec<String> {
    s.trim().split_whitespace().map(|s| s.to_string()).collect()
}

fn read_input<P: AsRef<std::path::Path>>(path: P) -> Result<Vec<String>> {
    let file = File::open(path)?;
    BufReader::new(file).lines().map(|l| Ok(l?)).collect()
}

#[cfg(test)]
mod tests {
    use crate::{parse_digits, parse_entries, parse_entry, solve_one, solve_two_entry};

    #[test]
    fn test_solution() {
        let digits = parse_digits("abcefg cf acdeg acdfg bcdf abdfg abdefg acf abcdefg abcdfg")
            .into_iter()
            .collect();
        let input = vec![
            "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe".to_string(),
            "edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc".to_string(),
            "fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg".to_string(),
            "fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb".to_string(),
            "aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea".to_string(),
            "fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb".to_string(),
            "dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe".to_string(),
            "bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef".to_string(),
            "egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb".to_string(),
            "gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce".to_string(),
        ];
        let entries = parse_entries(input).unwrap();
        assert_eq!(solve_one(&entries).unwrap(), 26);
        assert_eq!(
            solve_two_entry(
                &digits,
                &parse_entry(
                    &"acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab |
cdfeb fcadb cdfeb cdbaf"
                        .to_string()
                )
                .unwrap()
            ),
            Some(5353)
        );
    }
}
