use std::fs;

fn main() {
    let input = fs::read_to_string("day03/input.txt").expect("input.txt not found");

    let part1_res = part1(&input);
    println!("Part 1: {}", part1_res);

    let part2_res = part2(&input);
    println!("Part 2: {}", part2_res);
}

fn part1(input: &str) -> u32 {
    let mut total_joltage = 0;

    for line in input.lines() {
        let mut max_bank_joltage = 0;

        for i in 0..line.len() {
            let first_digit = line.chars().nth(i).unwrap().to_digit(10).unwrap();
            for j in i + 1..line.len() {
                let second_digit = line.chars().nth(j).unwrap().to_digit(10).unwrap();

                let joltage = 10 * first_digit + second_digit;
                if joltage > max_bank_joltage {
                    max_bank_joltage = joltage;
                }
            }
        }
        total_joltage += max_bank_joltage;
    }

    total_joltage
}

fn max_joltage_12_digits(line: &str) -> Vec<u32> {

    let k = 12;
    let mut result: Vec<u32> = Vec::new();
    let mut remaining = line.len();

    for c in line.chars() {
        let digit = c.to_digit(10).unwrap();
        remaining -= 1;

        while !result.is_empty() && *result.last().unwrap() < digit && result.len() + remaining >= k {
            result.pop();
        }

        if result.len() < k {
            result.push(digit);
        }
    }

    result
 
}

fn part2(input: &str) -> u64 {
    let mut total_joltage = 0;

    for line in input.lines() {
        let digits = max_joltage_12_digits(line);

        let mut max_joltage: u64 = 0;
        for d in digits {
            max_joltage = max_joltage * 10 + d as u64;
        }
        total_joltage += max_joltage;
    }

    total_joltage
}
