use regex::Regex;
use std::fs::read_to_string;

fn sum_muls(muls: &[(i32, i32)]) -> i32 {
    let res: i32 = muls.iter().map(|(x, y)| x * y).sum();
    res
}

fn get_multiplications(input: &str) -> Vec<(i32, i32)> {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    re.captures_iter(input)
        .map(|cap| {
            // parse the 2 numbers
            // println!("{:#?}", cap);
            let x = cap[1].parse::<i32>().ok().unwrap();
            let y = cap[2].parse::<i32>().ok().unwrap();
            (x, y)
        })
        .collect()
}

fn process_instructions(input: &str) -> i32 {
    let re = Regex::new(r"(mul\(\d{1,3},\d{1,3}\)|don't\(\)|do\(\))").unwrap();
    let mut enabled = true;
    let mut total = 0;

    for cap in re.find_iter(input) {
        let instruction = cap.as_str();
        match instruction {
            "do()" => {
                enabled = true;
            }
            "don't()" => {
                enabled = false;
            }
            mul_expr if mul_expr.starts_with("mul") => {
                if enabled {
                    if let Some(nums) = get_nums(mul_expr) {
                        total += nums.0 * nums.1;
                    }
                }
            }
            _ => {}
        }
    }
    total
}

fn get_nums(mul_expr: &str) -> Option<(i32, i32)> {
    let nums: Vec<i32> = mul_expr[4..mul_expr.len() - 1]
        .split(",")
        .filter_map(|n| n.parse().ok())
        .collect();
    if nums.len() == 2 {
        Some((nums[0], nums[1]))
    } else {
        None
    }
}

fn main() {
    let input = read_to_string("day03/input").expect("Couldn't open file");
    let muls = get_multiplications(&input);

    println!("Sum of multiplications: {}", sum_muls(&muls));
    println!("Enabled instructions: {}", process_instructions(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        let muls = get_multiplications(input);
        assert_eq!(muls.len(), 4);
        assert_eq!(sum_muls(&muls), 161);
    }

    #[test]
    fn test_invalid_patterns() {
        let input = "mul(4* mul(6,9! ?(12,34) mul ( 2 , 4 )";
        let muls = get_multiplications(input);
        assert_eq!(muls.len(), 0);
    }

    #[test]
    fn test_valid_numbers() {
        let input = "mul(123,4)mul(44,46)";
        let muls = get_multiplications(input);
        assert_eq!(muls, vec![(123, 4), (44, 46)]);
    }
}
