use std::{collections::{HashMap, HashSet}, fs, time::Instant};

fn count_path_part1(devices: &HashMap<String, Vec<String>>, current: &str) -> usize {
    if current == "out" {
        return 1;
    };

    let Some(neighbours) = devices.get(current) else {
        return 0;
    };

    neighbours.iter()
        .map(|n| count_path_part1(devices, n))
        .sum()
}

fn count_path_part2(
    devices: &HashMap<String, Vec<String>>,
    current: &str,
    cache: &mut HashMap<(String, bool, bool), usize>,
    visited: &mut HashSet<String>,
) -> usize {

    let has_dac = visited.contains("dac");
    let has_fft = visited.contains("fft");
    let cache_key = (current.to_string(), has_dac, has_fft);

    if let Some(&count) = cache.get(&cache_key) {
        return count;
    }

    visited.insert(current.to_string());

    if current == "out" {
        visited.remove(current);
        return if has_dac && has_fft { 1 } else { 0 }; 
    };

    let Some(neighbours) = devices.get(current) else {
        visited.remove(current);
        return 0;
    };

    let mut total = 0;

    for neighbour in neighbours {
        if !visited.contains(neighbour.as_str()) {
            total += count_path_part2(&devices, neighbour, cache, visited);
        }
    }

    cache.insert(cache_key, total);
    visited.remove(current);

    total
}

fn main() {
    let input = fs::read_to_string("day11/input.txt").expect("input.txt not found");

    let mut devices: HashMap<String, Vec<String>> = HashMap::new();

    for line in input.lines() {
        let mut parts = line.split(':');
        let machine = parts.next().unwrap_or("").trim().to_string();
        let connections: Vec<String> = parts
            .next()
            .unwrap()
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();

        devices.insert(machine.to_string(), connections);
    }

    let start1 = Instant::now();
    let part1_res = part1(&devices);
    let duration1 = start1.elapsed();
    println!("Part 1: {} ({:.2?})", part1_res, duration1);

    let start2 = Instant::now();
    let part2_res = part2(&devices);
    let duration2 = start2.elapsed();
    println!("Part 2: {} ({:.2?})", part2_res, duration2);
}

fn part1(devices: &HashMap<String, Vec<String>>) -> usize {
    count_path_part1(&devices, &"you".to_string())
}

fn part2(devices: &HashMap<String, Vec<String>>) -> usize {
    let mut cache: HashMap<(String, bool, bool), usize> = HashMap::new();
    let mut visited: HashSet<String> = HashSet::new();
    count_path_part2(&devices, &"svr".to_string(), &mut cache, &mut visited)
}
