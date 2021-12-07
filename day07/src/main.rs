use anyhow::Result;
use std::fs;

fn main() -> Result<()> {
    let input = fs::read_to_string("input.txt")?;
    let nums = parse_nums(input.trim())?;
    dbg!(min_cost(&nums, &const_diff));
    dbg!(min_cost(&nums, &lin_diff));
    Ok(())
}

fn min_cost<F: Fn(i32, i32) -> i32>(nums: &Vec<i32>, cost_fn: &F) -> Option<i32> {
    let min = *nums.iter().min()?;
    let max = *nums.iter().max()?;
    (min..max)
        .map(|n| total_cost(nums, n, cost_fn))
        .collect::<Option<Vec<i32>>>()?
        .into_iter()
        .min()
}

fn total_cost<F: Fn(i32, i32) -> i32>(nums: &Vec<i32>, num: i32, cost_fn: F) -> Option<i32> {
    nums.iter().map(|n| cost_fn(num, *n)).reduce(|s, n| s + n)
}

fn const_diff(x: i32, y: i32) -> i32 {
    (x - y).abs()
}

fn lin_diff(x: i32, y: i32) -> i32 {
    let d = const_diff(x, y);
    d * (d + 1) / 2
}

fn parse_nums(input: &str) -> Result<Vec<i32>> {
    input.split(',').map(|n| parse_num(n)).collect()
}

fn parse_num(input: &str) -> Result<i32> {
    let num = input.parse()?;
    Ok(num)
}

#[cfg(test)]
mod tests {
    use crate::{const_diff, lin_diff, min_cost};

    #[test]
    fn test_min_cost() {
        let nums = vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14];
        assert_eq!(min_cost(&nums, &const_diff), Some(37));
        assert_eq!(min_cost(&nums, &lin_diff), Some(168));
    }
}
