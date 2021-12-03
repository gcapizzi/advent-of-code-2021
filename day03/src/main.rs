use anyhow::{anyhow, Result};
use std::collections::HashMap;
use std::fs::File;
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
    let mut result: Vec<String> = strings.to_owned();
    let mut i = 0;
    while result.len() > 1 {
        let mcc = most_common_char(chars_at(i, &result)?).unwrap_or('1');
        result = result
            .to_owned()
            .into_iter()
            .filter_map(|s| {
                let c = s.chars().nth(i)?;
                if c == mcc {
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

fn least_common_string(strings: &Vec<String>) -> Result<String> {
    let mut result: Vec<String> = strings.to_owned();
    let mut i = 0;
    while result.len() > 1 {
        let lcc = least_common_char(chars_at(i, &result)?).unwrap_or('0');
        result = result
            .to_owned()
            .into_iter()
            .filter_map(|s| {
                let c = s.chars().nth(i)?;
                if c == lcc {
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
        .ok_or(anyhow!("failed to find least common string"))
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
    let counts = count_chars(chars);
    let max = counts
        .iter()
        .max_by_key(|&(_, count)| count)
        .ok_or(anyhow!("empty input"))?;
    let n_of_maxes = counts.iter().filter(|&(_, count)| count == max.1).count();
    if n_of_maxes > 1 {
        return Err(anyhow!("more than one most common char"));
    }
    Ok(*max.0)
}

fn least_common_char<I: IntoIterator<Item = char>>(chars: I) -> Result<char> {
    let counts = count_chars(chars);
    let min = counts
        .iter()
        .min_by_key(|&(_, count)| count)
        .ok_or(anyhow!("empty input"))?;
    let n_of_mins = counts.iter().filter(|&(_, count)| count == min.1).count();
    if n_of_mins > 1 {
        return Err(anyhow!("more than one least common char"));
    }
    Ok(*min.0)
}

fn count_chars<I: IntoIterator<Item = char>>(chars: I) -> HashMap<char, usize> {
    let mut counts = HashMap::<char, usize>::new();
    for c in chars {
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
