use std::fs;

fn main() {
    let input = fs::read_to_string("day06/input.txt").expect("input.txt not found");
    let rows: Vec<&str> = input.lines().collect();

    let part1_res = part1(&rows);
    println!("Part 1: {}", part1_res);

    // let part2_res = part2(&rows);
    // println!("Part 2: {}", part2_res);
}

fn evalutate(nums: &Vec<u128>, operator: char) -> u128 {
    match operator {
        '+' => nums.iter().sum(),
        '*' => nums.iter().product(),
        _ => unreachable!(),
    }
}

fn part1(rows: &Vec<&str>) -> u128 {
    let col_len = rows.iter().map(|r| r.len()).max().unwrap();
    let row_len = rows.len();

    let mut total: u128 = 0;
    let mut col = 0;

    while col < col_len {
        let is_empty = rows
            .iter()
            .all(|row| row.chars().nth(col).unwrap_or(' ') == ' ');

        if is_empty {
            col += 1;
            continue;
        }

        let start_col = col;
        let mut end_col = col;

        while end_col < col_len {
            let is_empty = rows
                .iter()
                .all(|row| row.chars().nth(end_col).unwrap_or(' ') == ' ');

            if is_empty {
                break;
            }
            end_col += 1;
        }

        let mut numbers: Vec<u128> = Vec::new();
        let mut operator = '+';

        for row_idx in 0..row_len {
            let row = rows[row_idx];

            let mut text = String::new();
            for c in start_col..end_col.min(row.len()) {
                text.push(row.chars().nth(c).unwrap_or(' '));
            }

            let text = text.trim();

            if text.is_empty() {
                continue;
            }

            if text == "+" || text == "*" {
                operator = text.chars().next().unwrap();
            } else {
                numbers.push(text.parse().unwrap());
            }
        }

        if !numbers.is_empty() {
            total += evalutate(&numbers, operator);
        }

        col = end_col;
    }

    total
}