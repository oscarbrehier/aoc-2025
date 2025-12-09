use std::{fs, time::Instant};

fn main() {
    let input = fs::read_to_string("day09/input.txt").expect("input.txt not found");

    let mut red_tiles: Vec<(i32, i32)> = Vec::new();

    for line in input.lines() {
        let coords: Vec<i32> = line.split(',').map(|s| s.trim().parse().unwrap()).collect();
        if coords.len() == 2 {
            red_tiles.push((coords[0], coords[1]));
        }
    }

    let start1 = Instant::now();
    let part1_res = part1(&red_tiles);
    let duration1 = start1.elapsed();
    println!("Part 1: {} ({:.2?})", part1_res, duration1);

    let start2 = Instant::now();
    let part2_res = part2(&red_tiles);
    let duration2 = start2.elapsed();
    println!("Part 2: {} ({:.2?})", part2_res, duration2);
}

fn calculate_area(tile_1: (i32, i32), tile_2: (i32, i32)) -> i64 {
    let (x1, y1) = tile_1;
    let (x2, y2) = tile_2;

    ((x2 - x1).abs() as i64 + 1) * ((y2 - y1).abs() as i64 + 1)
}

fn part1(red_tiles: &Vec<(i32, i32)>) -> i64 {
    let mut max_area = 0;

    for i in 0..red_tiles.len() {
        if i % 50 == 0 {}
        for j in i + 1..red_tiles.len() {
            let area = calculate_area(red_tiles[i], red_tiles[j]);
            max_area = max_area.max(area);
        }
    }

    max_area
}

fn build_row_segments(red_tiles: &[(i32, i32)]) -> (i32, i32, Vec<Vec<(i32, i32)>>) {
    let min_x = red_tiles.iter().map(|&(x, _)| x).min().unwrap();
    let min_y = red_tiles.iter().map(|&(_, y)| y).min().unwrap();
    let max_y = red_tiles.iter().map(|&(_, y)| y).max().unwrap();

    let height = (max_y - min_y + 1) as usize;

    let mut rows: Vec<Vec<(i32, i32)>> = vec![Vec::new(); height];
    let mut intersections: Vec<f64> = Vec::new();

    for (row_idx, y) in (min_y..=max_y).enumerate() {
        intersections.clear();
        let mut horiz_spans: Vec<(i32, i32)> = Vec::new();

        for i in 0..red_tiles.len() {
            let (x1, y1) = red_tiles[i];
            let (x2, y2) = red_tiles[(i + 1) % red_tiles.len()];

            if y1 == y2 {
                if y == y1 {
                    let sx = x1.min(x2);
                    let ex = x1.max(x2);

                    horiz_spans.push((sx, ex));
                }
                continue;
            }

            let y_min = y1.min(y2);
            let y_max = y1.max(y2);

            if y >= y_min && y < y_max {
                let dy = (y2 - y1) as f64;
                let dx = (x2 - x1) as f64;
                let t = (y as f64 - y1 as f64) / dy;
                let x_inter = x1 as f64 + dx * t;

                intersections.push(x_inter);
            }
        }

        if !intersections.is_empty() {
            intersections.sort_by(|a, b| a.partial_cmp(b).unwrap());
            let mut k = 0;

            while k + 1 < intersections.len() {
                let left_f = intersections[k];
                let right_f = intersections[k + 1];
                let x_start = left_f.ceil() as i32;
                let x_end = right_f.floor() as i32;

                if x_start <= x_end {
                    horiz_spans.push((x_start, x_end));
                }

                k += 2;
            }
        }

        if horiz_spans.is_empty() {
        } else {
            horiz_spans.sort_by_key(|s| s.0);
            let mut merged: Vec<(i32, i32)> = Vec::with_capacity(horiz_spans.len());
            let mut cur = horiz_spans[0];

            for &seg in &horiz_spans[1..] {
                if seg.0 <= cur.1 + 1 {
                    cur.1 = cur.1.max(seg.1);
                } else {
                    merged.push(cur);
                    cur = seg;
                }
            }

            merged.push(cur);
            rows[row_idx] = merged;
        }
    }

    (min_x, min_y, rows)
}

fn row_contains_span(segments: &[(i32, i32)], qx: i32, qy: i32) -> bool {
    if segments.is_empty() {
        return false;
    }

    let mut lo: isize = 0;
    let mut hi: isize = segments.len() as isize;

    while lo < hi {
        let mid = (lo + hi) / 2;

        if segments[mid as usize].0 <= qx {
            lo = mid + 1;
        } else {
            hi = mid;
        }
    }

    let ins = lo as usize;

    if ins == 0 {
        return false;
    }

    let cand = ins - 1;

    segments[cand].1 >= qy
}

fn part2(red_tiles: &Vec<(i32, i32)>) -> i64 {
    let (_, min_y, rows) = build_row_segments(red_tiles);
    let y_to_idx = |y: i32| -> usize { (y - min_y) as usize };
    let mut max_area: i64 = 0;
    let n = red_tiles.len();

    for i in 0..n {
        for j in i + 1..n {
            let (x1, y1) = red_tiles[i];
            let (x2, y2) = red_tiles[j];

            let min_x_r = x1.min(x2);
            let max_x_r = x1.max(x2);
            let min_y_r = y1.min(y2);
            let max_y_r = y1.max(y2);
            let area =
                ((max_x_r - min_x_r).abs() as i64 + 1) * ((max_y_r - min_y_r).abs() as i64 + 1);

            if area <= max_area {
                continue;
            }

            let mut ok = true;
            let start_idx = y_to_idx(min_y_r);
            let end_idx = y_to_idx(max_y_r);

            if start_idx >= rows.len() || end_idx >= rows.len() {
                ok = false;
            } else {
                for ridx in start_idx..=end_idx {
                    if rows[ridx].is_empty() {
                        ok = false;
                        break;
                    }
                    if !row_contains_span(&rows[ridx], min_x_r, max_x_r) {
                        ok = false;
                        break;
                    }
                }
            }

            if ok {
                max_area = max_area.max(area);
            }
        }
    }

    max_area
}
