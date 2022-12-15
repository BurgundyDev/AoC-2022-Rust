use std::collections::HashSet;

use aoc2022::read_file;

fn main() {
    part1("test_input.txt", 10);
    part1("input.txt", 2000000);
    part2("test_input.txt"); // This, for whatever reason, does not work for the example input
    part2("input.txt")
}

fn parse(file_path: &str) -> (Vec<Vec<i64>>, Vec<Vec<i64>>) {
    let input = read_file(file_path);
    let mut sensors: Vec<Vec<i64>> = Vec::new();
    let mut beacons: Vec<Vec<i64>> = Vec::new();
    
    for line in input.lines() {
        let stripped = line.strip_prefix("Sensor at ").unwrap().split(": closest beacon is at ").collect::<Vec<&str>>();
        let mut sensor: Vec<i64> = Vec::new();
        let mut sensor_splitter = stripped[0].split(", ");
        sensor.push(sensor_splitter.next().unwrap().strip_prefix("x=").unwrap().parse().unwrap());
        sensor.push(sensor_splitter.next().unwrap().strip_prefix("y=").unwrap().parse().unwrap());
        sensors.push(sensor);

        let mut beacon: Vec<i64> = Vec::new();
        let mut beacon_splitter = stripped[1].split(", ");
        beacon.push(beacon_splitter.next().unwrap().strip_prefix("x=").unwrap().parse().unwrap());
        beacon.push(beacon_splitter.next().unwrap().strip_prefix("y=").unwrap().parse().unwrap());
        beacons.push(beacon)
    }

    (sensors, beacons)
}

fn calculate_distance(a: &Vec<i64>, b: &Vec<i64>) -> i64 {
    (a[0].abs_diff(b[0]) + a[1].abs_diff(b[1])).try_into().unwrap()
}

fn part1(file_path: &str, row: i64) {
    let (sensors, beacons) = parse(file_path);
    let mut rel_sensors: Vec<Vec<i64>> = Vec::new();

    let sensors_iterator = sensors.iter().map(|sensor| sensor[0]);
    let beacons_iterator = beacons.iter().map(|beacon| beacon[0]);
    let all = sensors_iterator.chain(beacons_iterator);

    let min_row = all.clone().min().unwrap();
    let max_row = all.max().unwrap();

    let mut distances: Vec<i64> = Vec::new();
    let mut rel_distances: Vec<i64> = Vec::new();

    for (idx, sensor) in sensors.iter().enumerate() {
        let distance = calculate_distance(sensor, &beacons[idx]);
        distances.push(distance);
        if distance >= calculate_distance(sensor, &vec![sensor[0], row]) {
            rel_distances.push(distance);
            rel_sensors.push(sensor.clone());
        }
    }

    let mut impossibilites: HashSet<Vec<i64>> = HashSet::new();

    for i in (min_row-1)..(max_row) {
        let measured_point = vec![i, row];
        if !beacons.contains(&measured_point)
        {
            for (idx, sensor) in rel_sensors.iter().enumerate() {
                if calculate_distance(&measured_point, sensor) <= rel_distances[idx] {
                    impossibilites.insert(measured_point.clone());
                    break;
                }
            }
        }
    }

    println!("{}", impossibilites.len());
}

fn part2(file_path: &str) {
    let (sensors, beacons) = parse(file_path);

    let mut distances: Vec<i64> = Vec::new();

    for (idx, sensor) in sensors.iter().enumerate() {
        let distance = calculate_distance(sensor, &beacons[idx]);
        distances.push(distance);
    }

    
    let num_of_sensors = sensors.len() as usize;

    let mut pos_lines: Vec<i64> = Vec::new();
    let mut neg_lines: Vec<i64> = Vec::new();

    for (idx, sensor) in sensors.iter().enumerate() {
        let d = distances[idx];
        pos_lines.push(sensor[0] + sensor[1] - d);
        pos_lines.push(sensor[0] + sensor[1] + d);
        neg_lines.push(sensor[0] - sensor[1] - d);
        neg_lines.push(sensor[0] - sensor[1] + d);
    }

    let mut pos: i64 = 0;
    let mut neg: i64 = 0;

    for i in 0..(2 * num_of_sensors) {
        for j in i+1..(2 * num_of_sensors) {
            let a = &pos_lines[i];
            let b = &pos_lines[j];

            if (a - b).abs() == 2 {
                pos = *a.min(b) + 1
            }

            let c = &neg_lines[i];
            let d = &neg_lines[j];

            if (c - d).abs() == 2 {
                neg = *c.min(d) + 1
            }
        }
    }

    let x = (pos + neg) / 2;
    let y = (pos - neg) / 2;

    println!("{}", (x * 4000000) + y);
}