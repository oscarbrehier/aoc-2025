use std::{fs, u32};

fn main() {
    let input = fs::read_to_string("day04/input.txt").expect("input.txt not found");

    let mut grid: Vec<Vec<char>> = Vec::new();
    let mut height = 0usize;
    let mut width = 0usize;

    for line in input.lines() {
        let row: Vec<char> = line.chars().collect();
        if width == 0 {
            width = row.len();
        }
        height += 1;
        grid.push(row);
    }

    let part1_res = part1(&grid, height, width);
    println!("Part 1: {}", part1_res);

    let part2_res = part2(&mut grid, height, width);
    println!("Part 2: {}", part2_res);
}

fn count_adjacent_rolls(
    grid: &Vec<Vec<char>>,
    row: usize,
    col: usize,
    height: usize,
    width: usize,
) -> u32 {
    let mut adjacent_rolls = 0;

    let directions = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    for (row_offset, col_offset) in directions {
        let neighbor_row = row as isize + row_offset;
        let neighbor_col = col as isize + col_offset;

        if neighbor_row >= 0
            && neighbor_row < height as isize
            && neighbor_col >= 0
            && neighbor_col < width as isize
        {
            let nrow = neighbor_row as usize;
            let ncol = neighbor_col as usize;

            if grid[nrow][ncol] == '@' {
                adjacent_rolls += 1;
            }
        }
    }
    adjacent_rolls
}

fn part1(grid: &Vec<Vec<char>>, height: usize, width: usize) -> u32 {
    let mut accessible_rolls = 0;

    for row in 0..height {
        for col in 0..width {
            let cell = grid[row][col];

            if cell == '@' {
                let adjacent_rolls = count_adjacent_rolls(grid, row, col, height, width);

                if adjacent_rolls < 4 {
                    accessible_rolls += 1;
                }
            }
        }
    }

    accessible_rolls
}

fn part2(grid: &mut Vec<Vec<char>>, height: usize, width: usize) -> u32 {
    let mut removed_rolls = 0;
    let mut removed_rolls_in_round = 1;

    while removed_rolls_in_round >= 1 {
        removed_rolls_in_round = 0;
        for row in 0..height {
            for col in 0..width {
                let cell = grid[row][col];

                if cell == '@' {
                    let adjacent_rolls = count_adjacent_rolls(grid, row, col, height, width);

                    if adjacent_rolls < 4 {
                        grid[row][col] = 'x';
                        removed_rolls += 1;
                        removed_rolls_in_round += 1;
                    }
                }
            }
        }
    }

    removed_rolls
}
