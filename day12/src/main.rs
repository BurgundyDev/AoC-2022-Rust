use aoc2022::read_file;
use std::collections::{ BinaryHeap, HashSet};
use core::{cmp::Ordering, panic};

fn main() {
    println!("{}", part1("test_input.txt"));
    println!("{}", part1("input.txt"));
    println!("{}", part2("test_input.txt"));
    println!("{}", part2("input.txt"))
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
struct Node {
    cost: u32,
    location: Point,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn get_neighbours(coord: Point, map: &Vec<Vec<i32>>) -> Vec<Point> {
    let mut res: Vec<Point> = Vec::new();
    if coord.x > 0 {
        res.push(Point {x: coord.x - 1, y: coord.y});
    }
    if coord.y > 0 {
        res.push(Point {x: coord.x, y: coord.y - 1})
    }
    if coord.x < (map[0].len() - 1).try_into().unwrap() {
        res.push(Point {x: coord.x + 1, y: coord.y})
    }
    if coord.y < (map.len() - 1).try_into().unwrap() {
        res.push(Point {x: coord.x, y: coord.y + 1})
    }
    res
}

fn part1(file_path: &str) -> u32 {
    let input = read_file(file_path);
    let mut map: Vec<Vec<i32>> = Vec::new();

    for line in input.lines()
    {
        let row: Vec<i32> = line.chars().map(|c| c as i32 - 97).collect();
        map.push(row);
    }

    let mut start_point = Point { x: 0, y: 0};
    let mut end_point = Point { x: 0, y: 0};

    for (y, row) in map.iter_mut().enumerate() {
        for (x, loc) in row.iter_mut().enumerate() {
            if *loc == -14 {
                *loc = 0;
                start_point = Point { x: x, y: y};
            } else if *loc == -28 {
                *loc = 25;
                end_point = Point { x: x, y: y};
            }
        }
    }

    println!("{:?}", map);
    println!("Start point: {:?}", start_point);
    println!("End point: {:?}", end_point);

    let mut pq: BinaryHeap<Node> = BinaryHeap::new();
    let mut visited: HashSet<Point> = HashSet::new();

    pq.push(Node { cost: (0), location: (start_point) });
    visited.insert(start_point);

    println!("{:?}", get_neighbours(start_point, &map));

    while let Some(Node { cost, location }) = pq.pop() {
        if location == end_point {
            return cost;
        }

        let curr_height = map[location.y][location.x];
        let neighbours = get_neighbours(location, &map);
        let candidates: Vec<_> = neighbours
            .iter()
            .filter(|coord| {
                let height = map[coord.y][coord.x];
                height <= curr_height + 1
            })
            .collect();

        for candidate in candidates {
            if visited.insert(*candidate) {
                pq.push(Node {cost: cost + 1, location: *candidate})
            }
        }
    }

    panic!("Cannot find way through.")
}

fn part2(file_path: &str) -> u32 {
    let input = read_file(file_path);
    let mut map: Vec<Vec<i32>> = Vec::new();

    for line in input.lines()
    {
        let row: Vec<i32> = line.chars().map(|c| c as i32 - 97).collect();
        map.push(row);
    }

    let mut start_point = Point { x: 0, y: 0};
    let mut end_point = Point { x: 0, y: 0};

    for (y, row) in map.iter_mut().enumerate() {
        for (x, loc) in row.iter_mut().enumerate() {
            if *loc == -14 {
                *loc = 0;
                start_point = Point { x: x, y: y};
            } else if *loc == -28 {
                *loc = 25;
                end_point = Point { x: x, y: y};
            }
        }
    }

    // println!("{:?}", map);
    println!("Start point: {:?}", start_point);
    println!("End point: {:?}", end_point);

    let mut pq: BinaryHeap<Node> = BinaryHeap::new();
    let mut visited: HashSet<Point> = HashSet::new();

    pq.push(Node { cost: (0), location: (end_point) });
    visited.insert(end_point);

    // println!("{:?}", get_neighbours(start_point, &map));

    while let Some(Node { cost, location }) = pq.pop() {
        if map[location.y][location.x] == 0 {
            return cost;
        }

        let curr_height = map[location.y][location.x];
        let neighbours = get_neighbours(location, &map);
        let candidates: Vec<_> = neighbours
            .iter()
            .filter(|coord| {
                let height = map[coord.y][coord.x];
                height >= curr_height - 1
            })
            .collect();

        for candidate in candidates {
            if visited.insert(*candidate) {
                pq.push(Node {cost: cost + 1, location: *candidate})
            }
        }
    }

    panic!("Cannot find way through.")
}