use std::fs::read_to_string;

const DIRECTIONS: [(i32, i32); 8] = [
    (0, 1), // right
    (1, 0), // down
    (0, -1), // left
    (-1, 0), // up
    (1, 1), // down-right
    (-1, 1), // up-right
    (1, -1), // down-left
    (-1, -1), // up-left
];

fn count_xmas_part1(grid: &[Vec<char>]) -> i32 {
    let rows = grid.len() as i32;
    let cols = grid[0].len() as i32;
    let mut count = 0;

    for i in 0..rows {
        for j in 0..cols {
            for &(dx, dy) in &DIRECTIONS {
                if check_xmas(grid, i, j, dx, dy, rows, cols) {
                    count += 1;
                }
            }
        }
    }
    count
}

fn check_xmas(grid: &[Vec<char>], i: i32, j: i32, dx: i32, dy: i32, rows: i32, cols: i32) -> bool {
    let pattern = ['X', 'M', 'A', 'S'];

    for k in 0..4 {
        let x = i + k * dx;
        let y = j + k * dy;

        if x < 0 || x >= rows || y < 0 || y >= cols {
            return false;
        }

        if grid[x as usize][y as usize] != pattern[k as usize] {
            return false;
        }
    }
    true
}

fn count_xmas_part2(grid: &[Vec<char>]) -> i32 {
    let rows = grid.len() as i32;
    let cols = grid[0].len() as i32;
    let mut count = 0;

    for i in 1..rows - 1 {
        for j in 1..cols - 1 {
            if check_x_pattern(grid, i, j, rows, cols) {
                count += 1;
            }
        }
    }
    count
}

fn check_x_pattern(grid: &[Vec<char>], i: i32, j: i32, rows: i32, cols: i32) -> bool {
    if grid[i as usize][j as usize] != 'A' {
        return false;
    }

    let check_diagonal = |x: i32, y: i32, dx: i32, dy: i32| -> bool {
        let x1 = x - dx;
        let y1 = y - dy;
        let x2 = x + dx;
        let y2 = y + dy;

        if
            x1 < 0 ||
            x1 >= rows ||
            y1 < 0 ||
            y1 >= cols ||
            x2 < 0 ||
            x2 >= rows ||
            y2 < 0 ||
            y2 >= cols
        {
            return false;
        }

        let c1 = grid[x1 as usize][y1 as usize];
        let c2 = grid[x2 as usize][y2 as usize];
        (c1 == 'M' && c2 == 'S') || (c1 == 'S' && c2 == 'M')
    };

    check_diagonal(i, j, 1, 1) && check_diagonal(i, j, 1, -1)
}

fn parse_grid(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}

fn solve(grid: &[Vec<char>], part: i32) -> i32 {
    match part {
        1 => count_xmas_part1(grid),
        2 => count_xmas_part2(grid),
        _ => panic!("Invalid part"),
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = read_to_string("day04/input")?;
    let grid = parse_grid(&input);

    println!("XMAS patterns found: {}", solve(&grid, 1));
    println!("MAS but in X found: {}", solve(&grid, 2));
    Ok(())
}
