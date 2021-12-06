use anyhow::Result;
use std::collections::HashMap;
use std::fs;

fn main() -> Result<()> {
    let input = fs::read_to_string("input.txt")?;
    let nums = parse_nums(input.trim())?;
    dbg!(evolve(80, &nums).len());
    let mut counts = count(&nums);
    evolve_counts(256, &mut counts);
    let sum = counts.iter().fold(0, |s, (_, c)| s + c);
    dbg!(sum);
    Ok(())
}

fn evolve_counts(times: usize, counts: &mut HashMap<&i32, usize>) {
    for _ in 0..times {
        evolve_counts_step(counts);
    }
}

fn evolve_counts_step(counts: &mut HashMap<&i32, usize>) {
    let spawns = *counts.get(&0).unwrap_or(&0);
    counts.insert(&0, *counts.get(&1).unwrap_or(&0));
    counts.insert(&1, *counts.get(&2).unwrap_or(&0));
    counts.insert(&2, *counts.get(&3).unwrap_or(&0));
    counts.insert(&3, *counts.get(&4).unwrap_or(&0));
    counts.insert(&4, *counts.get(&5).unwrap_or(&0));
    counts.insert(&5, *counts.get(&6).unwrap_or(&0));
    counts.insert(&6, *counts.get(&7).unwrap_or(&0) + spawns);
    counts.insert(&7, *counts.get(&8).unwrap_or(&0));
    counts.insert(&8, spawns);
}

fn count(nums: &Vec<i32>) -> HashMap<&i32, usize> {
    let mut map = HashMap::new();
    for n in nums {
        *map.entry(n).or_insert(0) += 1;
    }
    map
}

fn parse_nums(input: &str) -> Result<Vec<i32>> {
    input.split(',').map(|n| parse_num(n)).collect()
}

fn parse_num(input: &str) -> Result<i32> {
    let num = input.parse()?;
    Ok(num)
}

fn evolve(times: usize, nums: &Vec<i32>) -> Vec<i32> {
    if times > 0 {
        evolve(times - 1, &evolve_step(nums))
    } else {
        nums.iter().copied().collect()
    }
}

fn evolve_step(nums: &Vec<i32>) -> Vec<i32> {
    nums.iter().flat_map(|n| evolve_num(n)).collect()
}

fn evolve_num(n: &i32) -> Vec<i32> {
    if *n == 0 {
        vec![6, 8]
    } else {
        vec![n - 1]
    }
}

#[cfg(test)]
mod tests {
    use crate::{count, evolve, evolve_counts};

    #[test]
    fn test_evolve() {
        let input = vec![3, 4, 3, 1, 2];
        assert_eq!(evolve(1, &input).sort(), vec![2, 3, 2, 0, 1].sort());
        assert_eq!(
            evolve(18, &input).sort(),
            vec![6, 0, 6, 4, 5, 6, 0, 1, 1, 2, 6, 0, 1, 1, 1, 2, 2, 3, 3, 4, 6, 7, 8, 8, 8, 8]
                .sort()
        );
        assert_eq!(evolve(80, &input).len(), 5934);
    }

    #[test]
    fn test_evolve_counts() {
        let input = vec![3, 4, 3, 1, 2];
        let mut counts = count(&input);
        evolve_counts(256, &mut counts);
        assert_eq!(counts.iter().fold(0, |s, (_, n)| s + n), 26984457539);
    }
}
