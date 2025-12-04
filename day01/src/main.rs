use std::fs;

fn main() {
    let input = fs::read_to_string("day01/input.txt").expect("input.txt not found");

    let part1_res = part1(&input);
    println!("Part 1: {}", part1_res);

    let part2_res = part2(&input);
    println!("Part 2: {}", part2_res);
}

fn part1(input: &str) -> i32 {
    let mut dial_pos = 50;
    let mut pointing_zero = 0;

    for (_, line) in input.lines().enumerate() {
        let (rot, distance_str) = line.trim().split_at(1);
        let distance = distance_str.parse::<i32>().unwrap();

        dial_pos = match rot {
            "L" => (dial_pos - distance).rem_euclid(100),
            "R" => (dial_pos + distance).rem_euclid(100),
            _ => panic!("Invalid rotation, use L|R"),
        };

        if dial_pos == 0 {
            pointing_zero += 1;
        }
    }

    pointing_zero
}

fn part2(input: &str) -> i32 {
    let mut dial_pos = 50;
    let mut pointing_zero = 0;

    for line in input.lines() {
        let (rot, distance_str) = line.trim().split_at(1);
        let distance = distance_str.parse::<i32>().unwrap();

        match rot {
            "L" => {
                for _ in 0..distance {
                    dial_pos = ((dial_pos - 1) as i32).rem_euclid(100);
                    if dial_pos == 0 {
                        pointing_zero += 1;
                    }
                }
            }
            "R" => {
                for _ in 0..distance {
                    dial_pos = ((dial_pos + 1) as i32).rem_euclid(100);
                    if dial_pos == 0 {
                        pointing_zero += 1;
                    }
                }
            }
            _ => panic!("Invalid rotation, use L|R"),
        }
    }

    pointing_zero
}
