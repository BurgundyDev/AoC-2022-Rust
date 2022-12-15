use std::{path::Path, env, ffi::OsStr};

use aoc2022::read_file;

fn main() {
    let input = read_file("test_input");
    println!("{}", input);
}

fn parse(file_path: &str) -> (Vec<Vec<i32>>, Vec<Vec<i32>>) {
    let input = read_file(file_path);
    let mut sensors: Vec<Vec<i32>> = Vec::new();
    let mut beacons: Vec<Vec<i32>> = Vec::new();
    
    for line in input.lines() {
        let stripped = line.strip_prefix("Sensor at ").unwrap().split(": closest beacon is at ").collect::<Vec<&str>>();

    }

    (sensors, beacons)
}

fn part1(file_path: &str) {

}