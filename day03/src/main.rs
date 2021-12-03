use anyhow::{anyhow, Result};
use std::collections::HashMap;
use std::fs::File;
use std::hash::Hash;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> Result<()> {
    let input = read_input("input.txt")?;
    let gamma_rate_str = most_common_chars(&input)?;
    let gamma_rate = i32::from_str_radix(&gamma_rate_str, 2)?;
    let epsilon_rate_str = least_common_chars(&input)?;
    let epsilon_rate = i32::from_str_radix(&epsilon_rate_str, 2)?;
    println!("power consumption: {}", gamma_rate * epsilon_rate);

    let oxigen_generator_rating_str = most_common_string(&input)?;
    let oxigen_generator_rating = i32::from_str_radix(&oxigen_generator_rating_str, 2)?;
    let co2_scrubber_rating_str = least_common_string(&input)?;
    let co2_scrubber_rating = i32::from_str_radix(&co2_scrubber_rating_str, 2)?;
    println!(
        "life support rating: {}",
        oxigen_generator_rating * co2_scrubber_rating
    );

    Ok(())
}

fn read_input<P: AsRef<std::path::Path>>(path: P) -> Result<Vec<String>> {
    let file = File::open(path)?;
    BufReader::new(file).lines().map(|l| Ok(l?)).collect()
}

fn most_common_chars(strings: &Vec<String>) -> Result<String> {
    let str_len = strings.first().ok_or(anyhow!("empty input!"))?.len();
    let chars = (0..str_len)
        .map(|i| {
            let chars = chars_at(i, strings)?;
            most_common_char(chars)
        })
        .collect::<Result<Vec<char>>>()?;
    Ok(chars.into_iter().collect())
}

fn least_common_chars(strings: &Vec<String>) -> Result<String> {
    let str_len = strings.first().ok_or(anyhow!("empty input!"))?.len();
    let chars = (0..str_len)
        .map(|i| {
            let chars = chars_at(i, strings)?;
            least_common_char(chars)
        })
        .collect::<Result<Vec<char>>>()?;
    Ok(chars.into_iter().collect())
}

fn most_common_string(strings: &Vec<String>) -> Result<String> {
    select_string_by_char(strings, most_common_char, '1')
}

fn least_common_string(strings: &Vec<String>) -> Result<String> {
    select_string_by_char(strings, least_common_char, '0')
}

fn select_string_by_char<F>(strings: &Vec<String>, select_char: F, default: char) -> Result<String>
where
    F: Fn(Vec<char>) -> Result<char>,
{
    let mut result: Vec<String> = strings.to_owned();
    let mut i = 0;
    while result.len() > 1 {
        let selected_char = select_char(chars_at(i, &result)?).unwrap_or(default);
        result = result
            .to_owned()
            .into_iter()
            .filter_map(|s| {
                let c = s.chars().nth(i)?;
                if c == selected_char {
                    Some(s)
                } else {
                    None
                }
            })
            .collect();
        i += 1;
    }
    result
        .into_iter()
        .nth(0)
        .ok_or(anyhow!("failed to find most common string"))
}

fn chars_at(index: usize, strings: &Vec<String>) -> Result<Vec<char>> {
    strings
        .iter()
        .map(|s| {
            s.chars()
                .nth(index)
                .ok_or(anyhow!("string too short: {:?}", s))
        })
        .collect()
}

fn most_common_char<I: IntoIterator<Item = char>>(chars: I) -> Result<char> {
    select_element_by_count(chars, |n1, n2| n1 > n2)
}

fn least_common_char<I: IntoIterator<Item = char>>(chars: I) -> Result<char> {
    select_element_by_count(chars, |n1, n2| n1 < n2)
}

fn select_element_by_count<T, I, F>(i: I, cmp_count: F) -> Result<T>
where
    I: IntoIterator<Item = T>,
    T: Eq + Hash + Copy,
    F: Fn(&usize, &usize) -> bool,
{
    let counts = count(i);
    let selected = counts
        .iter()
        .reduce(|p1, p2| if cmp_count(p1.1, p2.1) { p1 } else { p2 })
        .ok_or(anyhow!("empty input"))?;
    let n_of_selected = counts
        .iter()
        .filter(|&(_, count)| count == selected.1)
        .count();
    if n_of_selected > 1 {
        return Err(anyhow!("more than one selected elements"));
    }
    Ok(*selected.0)
}

fn count<T, I>(i: I) -> HashMap<T, usize>
where
    T: Eq + Hash,
    I: IntoIterator<Item = T>,
{
    let mut counts = HashMap::<T, usize>::new();
    for c in i {
        counts.entry(c).and_modify(|e| *e += 1).or_insert(0);
    }
    counts
}

#[cfg(test)]
mod tests {
    use crate::{least_common_chars, least_common_string, most_common_chars, most_common_string};
    use anyhow::Result;

    const NUMS: [&str; 12] = [
        "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000", "11001",
        "00010", "01010",
    ];

    #[test]
    fn test_least_and_most_common_chars() -> Result<()> {
        let nums = NUMS.to_vec().iter().map(|s| s.to_string()).collect();
        assert_eq!(most_common_chars(&nums)?, "10110");
        assert_eq!(least_common_chars(&nums)?, "01001");
        Ok(())
    }

    #[test]
    fn test_least_and_most_common_strings() -> Result<()> {
        let nums = NUMS.to_vec().iter().map(|s| s.to_string()).collect();
        assert_eq!(most_common_string(&nums)?, "10111");
        assert_eq!(least_common_string(&nums)?, "01010");
        Ok(())
    }
}
