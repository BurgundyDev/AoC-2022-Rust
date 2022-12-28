use std::{collections::HashSet};
use aoc2022::read_file;

fn main() {
    part1("test_input.txt");
}

fn part1(file_path: &str) {
    let data: Vec<i32> = read_file(file_path).chars().map(|c| if c == '>' {1} else {-1} ).collect();

    let mut height = 0;

    let mut solid: HashSet<[i32; 2]> = HashSet::new();
    for x in 0..7 {
        solid.insert([x, -1]);
    }
    
    let rocks = vec![
        vec![[0, 0], [1, 0], [2, 0], [3, 0]],
        vec![[1, 0], [0, 1], [1, 1], [2, 1], [1, 2]],
        vec![[0, 0], [1, 0], [2, 0], [2, 1], [2, 2]],
        vec![[0, 0], [0, 1], [0, 2], [0, 3]],
        vec![[0, 0], [1, 0], [0, 1], [1, 1]],
    ];

    let mut rock_count = 0;
    let mut rock_id = 0;

    let mut rock: HashSet<[i32; 2]> = HashSet::new();

    for stone in &rocks[rock_id] {
        rock.insert([stone[0] + 2, stone[1] + height + 3]);
    }

    // println!("{:?}", rock);

    while rock_count < 2022 {
        for jet in &data {
            let mut moved = rock.iter().map(|x| [x[0] + jet, x[1]]).collect::<HashSet<[i32; 2]>>();
            if (moved.iter().all(|x| 0 <= x[0] && x[0] < 7)) && !(moved.intersection(&solid).count() > 0) {
                rock = moved;
            }
            moved = rock.iter().map(|x| [x[0], x[1] - 1]).collect::<HashSet<[i32; 2]>>();

            if moved.intersection(&solid).count() > 0 {
                solid.extend(rock.clone());
                rock_count += 1;
                for stone in &solid {
                    height = height.max(stone[1])
                }
                if rock_count >= 2022 {
                    break
                }
                rock_id = (rock_id + 1) % 5;
                println!("{}", height);
                rock = HashSet::new();
                for stone in &rocks[rock_id] {
                    rock.insert([stone[0] + 2, stone[1] + height + 3]);
                }
            } else {
                rock = moved;
            }
        }
    }

    print!("part 1: {}", height)
}

fn summarize(solid: &HashSet<[i32; 2]> ) -> Vec<i32> {
    let mut o = vec![-20, -20, -20, -20, -20, -20, -20];

    for x in solid {
        let row = x[1] as usize;
        let col = x[0];
        o[row] = o[row].max(col)
    }

    let mut top = 0;

    for x in &o {
        top = top.max(*x)
    }
    
    o.into_iter().map(|f| f - top).collect()
}