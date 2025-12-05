use std::fs;

fn main() {
    let input = fs::read_to_string("day05/input.txt").expect("input.txt not found");

    let parts: Vec<&str> = input.split("\r\n\r\n").collect();
    let fresh_ingredients = parse_fresh_ingredients(parts.get(0).unwrap_or(&""));
    let available_ingredients = parse_available_ingredients(parts.get(1).unwrap_or(&""));

    let part1_res = part1_binary_search(&fresh_ingredients, &available_ingredients);
    println!("Part 1: {}", part1_res);

    let part2_res = part2(&fresh_ingredients);
    println!("Part 2: {}", part2_res);
}

fn parse_fresh_ingredients(input: &str) -> Vec<(u64, u64)> {
    let mut fresh_ingredients: Vec<(u64, u64)> = Vec::new();

    for line in input.lines() {
        let range: Vec<&str> = line.split("-").collect();
        let start = range
            .get(0)
            .unwrap()
            .trim()
            .parse::<u64>()
            .expect("Invalid range");
        let end = range
            .get(1)
            .unwrap()
            .trim()
            .parse::<u64>()
            .expect("Invalid range");

        fresh_ingredients.push((start, end));
    }

    fresh_ingredients.sort_by_key(|r| r.0);

    let mut merged: Vec<(u64, u64)> = Vec::new();

    for (start, end) in fresh_ingredients {
        if let Some(last) = merged.last_mut() {
            if start <= last.1 + 1 {
                last.1 = last.1.max(end);
                continue;
            }
        }
        merged.push((start, end));
    }

    merged
}

fn parse_available_ingredients(input: &str) -> Vec<u64> {
    input
        .lines()
        .map(|line| line.trim().parse::<u64>().expect("Invalid ID"))
        .collect()
}

fn part1(fresh_ingredients: &Vec<(u64, u64)>, available_ingredients: &Vec<u64>) -> u32 {
    let mut fresh_available_ingredients = 0;

    for ingredient in available_ingredients {

        for (start, end) in fresh_ingredients {
            if ingredient >= start && ingredient <= end {
                fresh_available_ingredients += 1;
            }
        }

    }

    fresh_available_ingredients
}

fn part1_binary_search(fresh_ingredients: &Vec<(u64, u64)>, available_ingredients: &Vec<u64>) -> u32 {
    let mut fresh_available_ingredients = 0;

    for ingredient in available_ingredients {

        let i = fresh_ingredients.partition_point(|&(start, _)| start <= *ingredient);
        
        if i > 0 {
            let (_, end) = fresh_ingredients[i - 1];
            if *ingredient <= end {
                fresh_available_ingredients += 1;
            }
        }

    }

    fresh_available_ingredients
}

fn part2(fresh_ingredients: &Vec<(u64, u64)>) -> u64 {

    let mut fresh_ingredient_count = 0;

    for (start, end) in fresh_ingredients {
        let count = end - start + 1;
        fresh_ingredient_count += count;
    }

    fresh_ingredient_count

}