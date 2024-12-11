use std::fs::read_to_string;

fn is_safe(levels: &[i32]) -> bool {
    if levels.len() < 2 {
        return true;
    }

    // Check if sequence is increasing or decreasing
    let is_increasing = levels[1] > levels[0];
    let mut prev = levels[0];

    for &curr in &levels[1..] {
        let diff = (curr - prev).abs();

        // Check if difference is between 1 and 3
        if !(1..=3).contains(&diff) {
            return false;
        }

        // Check if sequence maintains direction
        if is_increasing && curr <= prev {
            return false;
        }
        if !is_increasing && curr >= prev {
            return false;
        }

        prev = curr;
    }

    true
}

fn is_safe_with_dampener(levels: &[i32]) -> bool {
    if is_safe(levels) {
        return true;
    }

    // Try removing each level one at a time
    for i in 0..levels.len() {
        let mut dampened: Vec<i32> = levels.to_vec();
        dampened.remove(i);
        if is_safe(&dampened) {
            return true;
        }
    }
    false
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = read_to_string("day02/input")?;
    let levels: Vec<Vec<_>> = input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            line.split_whitespace()
                .filter_map(|n| n.parse().ok())
                .collect()
        })
        .collect();

    let safe_count = levels
        .iter()
        .filter(|level| is_safe(level.as_slice())) // Convert Vec reference to slice
        .count();

    let safe_with_dampeners = levels
        .iter()
        .filter(|level| is_safe_with_dampener(level.as_slice())) // Convert Vec reference to slice
        .count();

    println!("Safe reports: {}", safe_count);
    println!("Safe reports with dampener: {}", safe_with_dampeners);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

    #[test]
    fn test_safe_sequences() {
        assert!(is_safe(&[7, 6, 4, 2, 1])); // Decreasing by 1 or 2
        assert!(is_safe(&[1, 3, 6, 7, 9])); // Increasing by 1, 2, or 3
    }

    #[test]
    fn test_unsafe_sequences() {
        assert!(!is_safe(&[1, 2, 7, 8, 9])); // Jump too large
        assert!(!is_safe(&[9, 7, 6, 2, 1])); // Jump too large
        assert!(!is_safe(&[1, 3, 2, 4, 5])); // Changes direction
        assert!(!is_safe(&[8, 6, 4, 4, 1])); // No change between numbers
    }

    #[test]
    fn test_example_input() {
        let safe_count = TEST_INPUT
            .lines()
            .filter(|line| !line.trim().is_empty())
            .filter(|line| {
                let levels: Vec<i32> = line
                    .split_whitespace()
                    .filter_map(|n| n.parse().ok())
                    .collect();
                is_safe(&levels)
            })
            .count();

        assert_eq!(safe_count, 2);
    }

    #[test]
    fn test_safe_with_dampener() {
        assert!(is_safe_with_dampener(&[7, 6, 4, 2, 1])); // Already safe
        assert!(!is_safe_with_dampener(&[1, 2, 7, 8, 9])); // Can't fix with one removal
        assert!(!is_safe_with_dampener(&[9, 7, 6, 2, 1])); // Can't fix with one removal
        assert!(is_safe_with_dampener(&[1, 3, 2, 4, 5])); // Safe by removing 3
        assert!(is_safe_with_dampener(&[8, 6, 4, 4, 1])); // Safe by removing one 4
        assert!(is_safe_with_dampener(&[1, 3, 6, 7, 9])); // Already safe
    }

    #[test]
    fn test_example_input_part2() {
        let safe_count = TEST_INPUT
            .lines()
            .filter(|line| !line.trim().is_empty())
            .filter(|line| {
                let levels: Vec<i32> = line
                    .split_whitespace()
                    .filter_map(|n| n.parse().ok())
                    .collect();
                is_safe_with_dampener(&levels)
            })
            .count();

        assert_eq!(safe_count, 4);
    }
}
