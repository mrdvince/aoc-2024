use std::collections::HashMap;
use std::fs::read_to_string;

fn calculate_part1(input: &str) -> Result<i32, Box<dyn std::error::Error>> {
    let mut numbers_a = Vec::new();
    let mut numbers_b = Vec::new();

    for line in input.lines() {
        let nums: Vec<&str> = line.split_whitespace().collect();
        numbers_a.push(nums[0].parse::<i32>()?);
        numbers_b.push(nums.last().unwrap().parse::<i32>()?);
    }

    numbers_a.sort();
    numbers_b.sort();

    Ok(numbers_a
        .iter()
        .zip(numbers_b.iter())
        .map(|(a, b)| (a - b).abs())
        .sum())
}

fn calculate_part2(input: &str) -> Result<i32, Box<dyn std::error::Error>> {
    let mut numbers_a = Vec::new();
    let mut numbers_b = Vec::new();

    for line in input.lines() {
        let nums: Vec<&str> = line.split_whitespace().collect();
        numbers_a.push(nums[0].parse::<i32>()?);
        numbers_b.push(nums.last().unwrap().parse::<i32>()?);
    }

    let counter: HashMap<i32, i32> = numbers_b.iter().fold(HashMap::new(), |mut acc, &num| {
        *acc.entry(num).or_insert(0) += 1;
        acc
    });

    Ok(numbers_a
        .iter()
        .map(|&a| a * counter.get(&a).unwrap_or(&0))
        .sum())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = read_to_string("day01/input")?;
    println!("Part 1: {}", calculate_part1(&input)?);
    println!("Part 2: {}", calculate_part2(&input)?);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "3   4
4   3
2   5
1   3
3   9
3   3";

    #[test]
    fn test_part1() -> Result<(), Box<dyn std::error::Error>> {
        assert_eq!(calculate_part1(TEST_INPUT)?, 11);
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<(), Box<dyn std::error::Error>> {
        assert_eq!(calculate_part2(TEST_INPUT)?, 31);
        Ok(())
    }

    #[test]
    fn test_empty_input() {
        let result = calculate_part1("");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 0);
    }

    #[test]
    fn test_invalid_input() {
        let invalid_input = "not a number";
        assert!(calculate_part1(invalid_input).is_err());
        assert!(calculate_part2(invalid_input).is_err());
    }

    #[test]
    fn test_single_line() -> Result<(), Box<dyn std::error::Error>> {
        assert_eq!(calculate_part1("1 2")?, 1);
        Ok(())
    }
}
