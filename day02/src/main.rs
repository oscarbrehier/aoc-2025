use std::fs;

fn main() {
    let input = fs::read_to_string("day02/input.txt").expect("input.txt not found");

    let part1_res = get_invalid_ids_sum(&input, part1_is_invalid_id);
    println!("Part 1: {}", part1_res);

    let part2_res = get_invalid_ids_sum(&input, part2_is_invalid_id);
    println!("Part 2: {}", part2_res);
}

fn part1_is_invalid_id(id: u64) -> bool {
    let id_str = id.to_string();
    let len = id_str.len();

    if len % 2 != 0 {
        return false;
    }

    let half = len / 2;
    &id_str[..half] == &id_str[half..]
}

fn part2_is_invalid_id(id: u64) -> bool {
    let id_str = id.to_string();
    let len = id_str.len();

    for d in 1..=len / 2 {
        if len % d != 0 {
            continue;
        }

        let first_seq = &id_str[..d];

        if (1..(len / d)).all(|i| &id_str[i * d..(i + 1) * d] == first_seq) {
            return true
        }
    }

    false
}

fn get_invalid_ids_sum(input: &str, f: fn(u64) -> bool) -> u64 {
    let mut invalid_ids_sum = 0;

    for range in input.split(",") {
        let mut parts = range.split("-");

        let start = parts
            .next()
            .unwrap()
            .trim()
            .parse::<u64>()
            .expect("Invalid range");
        let end = parts
            .next()
            .unwrap()
            .trim()
            .parse::<u64>()
            .expect("Invalid range");

        for id in start..=end {
            if f(id) {
                invalid_ids_sum += id;
            }
        }
    }

    invalid_ids_sum
}
