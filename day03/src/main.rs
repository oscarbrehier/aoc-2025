use std::fs;

fn main() {
    let input = fs::read_to_string("day03/input.txt").expect("input.txt not found");

    let part1_res = part1(&input);
    println!("Part 1: {}", part1_res);

    // let part2_res = part2(&input);
    // println!("Part 2: {}", part2_res);
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
