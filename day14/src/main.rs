use std::i32::MAX;
use aoc2022::read_file;

fn main() {
    part1("test_input.txt");
    part1("input.txt")
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
enum Material {
    Air,
    Rock,
    Sand,
}

fn part1(file_path: &str) {
    let input = read_file(file_path);
    let mut left_bound = MAX;
    let mut right_bound = 0;
    let mut down_bound = 0;
    for line in input.lines() {
        let coords_x: Vec<i32> = line.split(" -> ").map(|s| s.split(",").collect::<Vec<&str>>()[0].parse().unwrap()).collect();
        if coords_x.iter().min().unwrap() < &left_bound
        {
            left_bound = *coords_x.iter().min().unwrap();
        }
        if coords_x.iter().max().unwrap() > &right_bound
        {
            right_bound = *coords_x.iter().max().unwrap();
        }
        let coords_y: Vec<i32> = line.split(" -> ").map(|s| s.split(",").collect::<Vec<&str>>()[1].parse().unwrap()).collect();
        if coords_y.iter().max().unwrap() > &down_bound
        {
            down_bound = *coords_y.iter().max().unwrap();
        }
    }
    println!("{}", left_bound);
    println!("{}", right_bound);
    println!("{}", down_bound);
    
    let mut scan: Vec<Vec<Material>> = Vec::new();

    for _i in 0..down_bound {
        scan.push(vec![Material::Air; (right_bound - left_bound).try_into().unwrap()])
    }

    for (y, row) in scan.iter().enumerate() {
        for (x, material) in row.iter().enumerate()
        {
            match material {
                Material::Air => print!("."),
                Material::Rock => print!("#"),
                Material::Sand => print!("o"),
            }
        }
        println!()
    }
}