use std::collections::HashSet;
use aoc2022::read_file;

fn main() {
    println!("{}", part1("test_input.txt"));
    println!("{}", part1("input.txt"));
    println!("{}", part2("test_input.txt"));
    println!("{}", part2("input.txt"));
}

fn day14(file_path: &str) -> (i32, HashSet<Vec<i32>>) {
    let mut down_bound = 0;
    let input = read_file(file_path);
    let mut blocked_tiles: HashSet<Vec<i32>> = HashSet::new();

    for line in input.lines() {
        let coords_y: Vec<i32> = line.split(" -> ").map(|s| s.split(",").collect::<Vec<&str>>()[1].parse().unwrap()).collect();
        if coords_y.iter().max().unwrap() > &down_bound
        {
            down_bound = *coords_y.iter().max().unwrap();
        }

        let data = line.split(" -> ").collect::<Vec<&str>>();
        let mut blocks: Vec<Vec<i32>> = Vec::new();

        for block in data {
            blocks.push(block.split(",").map(|s| s.parse().unwrap()).collect::<Vec<i32>>())
        }
        // println!("{:?}", blocks);

        for i in 0..(blocks.len() - 1) {
            let start_block = &blocks[i];
            let end_block = &blocks[i+1];

            if start_block[0] == end_block[0] {
                if start_block[1] > end_block[1] {
                    for j in 0..(start_block[1] - end_block[1] + 1)
                    {
                        blocked_tiles.insert(vec![start_block[0], end_block[1] + j]);
                    }
                } else {
                    for j in 0..(end_block[1] - start_block[1] + 1)
                    {
                        blocked_tiles.insert(vec![start_block[0], start_block[1] + j]);
                    }
                }
            } else {
                if start_block[0] > end_block[0] {
                    for j in 0..(start_block[0] - end_block[0] + 1)
                    {
                        blocked_tiles.insert(vec![end_block[0] + j, end_block[1]]);
                    }
                } else {
                    for j in 0..(end_block[0] - start_block[0] + 1)
                    {
                        blocked_tiles.insert(vec![start_block[0] + j, end_block[1]]);
                    }
                }
            }
        }
    }

    (down_bound, blocked_tiles)
}

fn part1(file_path: &str) -> i32 {

    let (down_bound, mut blocked_tiles) = day14(file_path);
    let mut sand: HashSet<Vec<i32>> = HashSet::new();
    let mut filled = false;
    let mut grains = 0;

    while !filled {
        let mut curr_pos = vec![500, 0];
        let mut curr_rest = false;
        while !curr_rest {
            if curr_pos[1] > down_bound {
                curr_rest = true;
                filled = true;
            } else if !blocked_tiles.contains(&vec![curr_pos[0], curr_pos[1] + 1]) {
                curr_pos[1] += 1;
            } else if !blocked_tiles.contains(&vec![curr_pos[0] - 1, curr_pos[1] + 1]) {
                curr_pos[0] -= 1;
                curr_pos[1] += 1;
            } else if !blocked_tiles.contains(&vec![curr_pos[0] + 1, curr_pos[1] + 1]) {
                curr_pos[0] += 1;
                curr_pos[1] += 1;
            } else {
                blocked_tiles.insert(curr_pos.clone());
                sand.insert(curr_pos.clone());
                grains += 1;
                curr_rest = true;
            }
        }
    }
    grains
}

fn part2(file_path: &str) -> i32 {
    let (down_bound, mut blocked_tiles) = day14(file_path);
    let floor = down_bound + 2;
    let mut sand: HashSet<Vec<i32>> = HashSet::new();
    let mut filled = false;
    let mut grains = 0;

    while !filled {
        let mut curr_pos = vec![500, 0];
        let mut curr_rest = false;
        while !curr_rest {
            if curr_pos[1] + 1 == floor {
                blocked_tiles.insert(curr_pos.clone());
                sand.insert(curr_pos.clone());
                grains += 1;
                curr_rest = true;
            } else if !blocked_tiles.contains(&vec![curr_pos[0], curr_pos[1] + 1]) {
                curr_pos[1] += 1;
            } else if !blocked_tiles.contains(&vec![curr_pos[0] - 1, curr_pos[1] + 1]) {
                curr_pos[0] -= 1;
                curr_pos[1] += 1;
            } else if !blocked_tiles.contains(&vec![curr_pos[0] + 1, curr_pos[1] + 1]) {
                curr_pos[0] += 1;
                curr_pos[1] += 1;
            } else {
                if curr_pos == vec![500, 0] {
                    grains += 1;
                    curr_rest = true;
                    filled = true;
                } else {  
                    blocked_tiles.insert(curr_pos.clone());
                    sand.insert(curr_pos.clone());
                    grains += 1;
                    curr_rest = true;
                }
            }
        }
    }
    grains
}