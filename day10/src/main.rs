use std::{collections::HashSet, fs, time::Instant, vec};

struct Matrix {
    data: Vec<Vec<bool>>,
    num_buttons: usize,
}

impl Matrix {
    fn from_machine(machine: &Machine) -> Self {
        let m = machine.num_lights();
        let n = machine.num_buttons();

        let mut data = vec![vec![false; n + 1]; m];

        for (btn_idx, button) in machine.buttons.iter().enumerate() {
            for &light_idx in button {
                data[light_idx][btn_idx] = true;
            }
        }

        for (light_idx, &target) in machine.target_lights.iter().enumerate() {
            data[light_idx][n] = target;
        }

        Self {
            data,
            num_buttons: n,
        }
    }

    fn target(&self, row: usize) -> bool {
        self.data[row][self.num_buttons]
    }

    fn rows(&self) -> usize {
        self.data.len()
    }

    fn cols(&self) -> usize {
        self.num_buttons + 1
    }

    fn get(&self, row: usize, col: usize) -> bool {
        self.data[row][col]
    }

    fn set(&mut self, row: usize, col: usize, val: bool) {
        self.data[row][col] = val;
    }

    fn swap_rows(&mut self, i: usize, j: usize) {
        self.data.swap(i, j);
    }

    fn add_row(&mut self, src: usize, dest: usize) {
        for col in 0..self.cols() {
            self.data[dest][col] ^= self.data[src][col];
        }
    }

    fn to_ref(&mut self) -> usize {
        let rows = self.rows();
        let cols = self.num_buttons;

        let mut current_row = 0;

        for col in 0..cols {
            let pivot_row = (current_row..rows).find(|&row| self.get(row, col));

            let Some(pivot_row) = pivot_row else {
                continue;
            };

            if pivot_row != current_row {
                self.swap_rows(current_row, pivot_row);
            }

            for row in (current_row + 1)..rows {
                if self.get(row, col) {
                    self.add_row(current_row, row);
                }
            }

            current_row += 1;
        }

        current_row
    }

    fn to_rref(&mut self) {
        let rows = self.rows();
        let cols = self.num_buttons;

        let mut pivots = vec![None; rows];

        for row in 0..rows {
            pivots[row] = (0..cols).find(|&col| self.get(row, col));
        }

        for row in (0..rows).rev() {
            if let Some(pivot_col) = pivots[row] {
                for upper_row in 0..row {
                    if self.get(upper_row, pivot_col) {
                        self.add_row(row, upper_row);
                    }
                }
            }
        }
    }

    fn has_no_solution(&self) -> bool {
        let rows = self.rows();
        let cols = self.num_buttons;

        for row in 0..rows {
            let all_zeros = (0..cols).all(|col| !self.get(row, col));
            let target_one = self.target(row);

            if all_zeros && target_one {
                return true;
            }
        }

        false
    }
}

struct Machine {
    target_lights: Vec<bool>,
    buttons: Vec<HashSet<usize>>,
    joltages: Vec<u32>,
}

impl Machine {
    fn from_line(line: &str) -> Result<Self, String> {
        let lights_start = line.find('[').ok_or("Invalid lights format")?;
        let lights_end = line.find(']').ok_or("Invalid lights format")?;
        let joltages_start = line.find('{').ok_or("Invalid joltages format")?;
        let joltages_end = line.find('}').ok_or("Invalid joltages format")?;

        let lights_str = &line[lights_start + 1..lights_end];
        let target_lights: Vec<bool> = lights_str.chars().map(|c| c == '#').collect();

        let buttons_section = &line[lights_end + 1..joltages_start];
        let buttons: Vec<HashSet<usize>> = buttons_section
            .split(')')
            .filter_map(|s| {
                let s = s.trim();
                if s.is_empty() {
                    return None;
                }

                let s = s.strip_prefix('(').unwrap_or(s);

                let indicies: HashSet<usize> =
                    s.split(',').filter_map(|n| n.trim().parse().ok()).collect();

                if indicies.is_empty() {
                    None
                } else {
                    Some(indicies)
                }
            })
            .collect();

        let joltages_str = &line[joltages_start + 1..joltages_end];
        let joltages: Vec<u32> = joltages_str
            .split(',')
            .filter_map(|s| s.trim().parse().ok())
            .collect();

        Ok(Machine {
            target_lights,
            buttons,
            joltages,
        })
    }

    fn num_lights(&self) -> usize {
        self.target_lights.len()
    }

    fn num_buttons(&self) -> usize {
        self.buttons.len()
    }
}

fn find_minimum_solution(matrix: &Matrix) -> Option<usize> {
    let rows = matrix.rows();
    let num_buttons = matrix.num_buttons;

    let mut pivot_cols = vec![None; rows];
    let mut is_pivot = vec![false; num_buttons];

    for row in 0..rows {
        for col in 0..num_buttons {
            if matrix.get(row, col) {
                pivot_cols[row] = Some(col);
                is_pivot[col] = true;
                break;
            }
        }
    }

    let free_vars: Vec<usize> = (0..num_buttons).filter(|&col| !is_pivot[col]).collect();

    let num_free = free_vars.len();
    let mut min_presses = usize::MAX;

    for mask in 0..(1 << num_free) {
        let mut solution = vec![false; num_buttons];

        for (i, &free_var) in free_vars.iter().enumerate() {
            solution[free_var] = (mask >> i) & 1 == 1;
        }

        for row in (0..rows).rev() {
            if let Some(pivot_col) = pivot_cols[row] {
                let mut sum = matrix.target(row);

                for col in (pivot_col + 1)..num_buttons {
                    if matrix.get(row, col) && solution[col] {
                        sum ^= true;
                    }
                }

                solution[pivot_col] = sum;
            }
        }

        let presses = solution.iter().filter(|&&x| x).count();
        min_presses = min_presses.min(presses);
    }

    Some(min_presses)
}

fn solve_machine(machine: &Machine) -> Option<usize> {
    let mut matrix = Matrix::from_machine(machine);

    matrix.to_ref();

    if matrix.has_no_solution() {
        return None;
    }

    matrix.to_rref();

    find_minimum_solution(&matrix)
}

fn main() {
    let input = fs::read_to_string("day10/input.txt").expect("input.txt not found");

    let machines: Vec<Machine> = input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .filter_map(|line| Machine::from_line(line).ok())
        .collect();

    let start = Instant::now();

    let total_presses: usize = machines
        .iter()
        .filter_map(|machine| solve_machine(machine))
        .sum();

    let duration = start.elapsed();

    println!("Total minimum button presses: {}", total_presses);
    println!("Time: {:.2?}", duration);

    // let start1 = Instant::now();
    // let part1_res = part1(&input);
    // let duration1 = start1.elapsed();
    // println!("Part 1: {} ({:.2?})", part1_res, duration1);

    // let start2 = Instant::now();
    // let part2_res = part2(&input);
    // let duration2 = start2.elapsed();
    // println!("Part 2: {} ({:.2?})", part2_res, duration2);
}
