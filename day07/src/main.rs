use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::fs;

fn main() {
    let input = fs::read_to_string("day07/input.txt").expect("input.txt not found");

    let mut width = 0;
    let height = input.lines().count();
    let mut start_col = 0;
    let mut splitters: HashSet<(usize, usize)> = HashSet::new();

    for (i, line) in input.lines().enumerate() {
        if width == 0 {
            width = line.len();
        }
        for (j, ch) in line.bytes().enumerate() {
            match ch {
                b'S' => start_col = j,
                b'^' => {
                    splitters.insert((i, j));
                }
                _ => {}
            }
        }
    }

    let part1_res = part1(&splitters, height, width, start_col);
    println!("Part 1: {}", part1_res);

    let part2_res = part2(&splitters, height, width, start_col);
    println!("Part 2: {}", part2_res);
}

fn part1(
    splitters: &HashSet<(usize, usize)>,
    height: usize,
    width: usize,
    start_col: usize,
) -> i32 {
    let mut splits = 0;

    let mut beams: VecDeque<(usize, usize)> = VecDeque::new();
    let mut visited_splitter: HashSet<(usize, usize)> = HashSet::new();
    let mut visited_positions: HashSet<(usize, usize)> = HashSet::with_capacity(height * width / 4);

    beams.push_back((0, start_col));

    while let Some((row, col)) = beams.pop_front() {
        if !visited_positions.insert((row, col)) {
            continue;
        }

        let next_row = row + 1;

        if next_row >= height {
            continue;
        }

        if splitters.contains(&(next_row, col)) {
            if visited_splitter.insert((next_row, col)) {
                splits += 1;
            }
            if col > 0 {
                beams.push_back((next_row, col - 1));
            }
            if col + 1 < width {
                beams.push_back((next_row, col + 1));
            }
        } else {
            beams.push_back((next_row, col));
        }
    }

    splits
}

fn part2(
    splitters: &HashSet<(usize, usize)>,
    height: usize,
    width: usize,
    start_col: usize,
) -> u64 {
    let mut total_exits = 0;

    let mut beams: VecDeque<(usize, usize)> = VecDeque::new();
    let mut visited_positions: HashSet<(usize, usize)> = HashSet::with_capacity(height * width / 4);
    let mut timeline_counts: HashMap<(usize, usize), u64> = HashMap::new();

    beams.push_back((0, start_col));
    timeline_counts.insert((0, start_col), 1);

    while let Some((row, col)) = beams.pop_front() {
        if !visited_positions.insert((row, col)) {
            continue;
        }

        let next_row = row + 1;
        let current_timeline_count = timeline_counts[&(row, col)];

        if next_row >= height {
            total_exits += current_timeline_count;
            continue;
        }

        let mut add_beam = |new_row: usize, new_col: usize| {
            *timeline_counts.entry((new_row, new_col)).or_insert(0) += current_timeline_count;
            beams.push_back((new_row, new_col));
        };

        if splitters.contains(&(next_row, col)) {
            if col > 0 {
                add_beam(next_row, col - 1);
            }
            if col + 1 < width {
                add_beam(next_row, col + 1);
            }
        } else {
            add_beam(next_row, col);
        }
    }

    total_exits
}
