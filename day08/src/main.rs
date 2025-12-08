use std::{collections::HashMap, fs};

struct Point {
    x: i32,
    y: i32,
    z: i32,
}

struct UnionFind {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        UnionFind {
            parent: (0..n).collect(),
            size: vec![1; n],
        }
    }
    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }
    fn union(&mut self, x: usize, y: usize) -> bool {
        let root_x = self.find(x);
        let root_y = self.find(y);

        if root_x == root_y {
            return false;
        }

        if self.size[root_x] < self.size[root_y] {
            self.parent[root_x] = root_y;
            self.size[root_y] += self.size[root_x];
        } else {
            self.parent[root_y] = root_x;
            self.size[root_x] += self.size[root_y];
        }

        true
    }
}

fn main() {
    let input = fs::read_to_string("day08/input.txt").expect("input.txt not found");
    let mut boxes: Vec<Point> = Vec::new();

    for line in input.lines() {
        let coords: Vec<i32> = line.split(',').map(|s| s.parse().unwrap()).collect();
        boxes.push(Point {
            x: coords[0],
            y: coords[1],
            z: coords[2],
        });
    }

    let mut box_distances: Vec<(f64, usize, usize)> = Vec::new();

    for i in 0..boxes.len() {
        for j in (i + 1)..boxes.len() {
            let dx = (boxes[j].x - boxes[i].x) as f64;
            let dy = (boxes[j].y - boxes[i].y) as f64;
            let dz = (boxes[j].z - boxes[i].z) as f64;
            let distance = (dx * dx + dy * dy + dz * dz).sqrt();
            box_distances.push((distance, i, j));
        }
    }

    box_distances.sort_by(|a, b| a.0.total_cmp(&b.0));

    let part1_res = part1(&box_distances, boxes.len());
    println!("Part 1: {}", part1_res);

    let part2_res = part2(&box_distances, &boxes);
    println!("Part 2: {}", part2_res);
}

fn part1(box_distances: &Vec<(f64, usize, usize)>, boxes_len: usize) -> usize {
    let mut uf = UnionFind::new(boxes_len);

    for (_, i, j) in box_distances.iter().take(boxes_len) {
        uf.union(*i, *j);
    }

    let mut circuit_sizes: HashMap<usize, usize> = HashMap::new();

    for i in 0..boxes_len {
        let root = uf.find(i);
        circuit_sizes.insert(root, uf.size[root]);
    }

    let mut sizes: Vec<usize> = circuit_sizes.values().copied().collect();
    sizes.sort();
    sizes.reverse();

    sizes[0] * sizes[1] * sizes[2]
}

fn part2(box_distances: &Vec<(f64, usize, usize)>, boxes: &Vec<Point>) -> i64 {
    let mut uf = UnionFind::new(boxes.len());
    let mut num_circuits = boxes.len();

    for (_, i, j) in box_distances.iter() {
        if uf.union(*i, *j) {
            num_circuits -= 1;

            if num_circuits == 1 {
                return (boxes[*i].x as i64) * (boxes[*j].x as i64);
            }
        }
    }

    0
}
