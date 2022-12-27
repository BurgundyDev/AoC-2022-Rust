use aoc2022::read_file;

fn main() {
    part1("test_input.txt");
}

fn part1(file_path: &str) {
    let data: Vec<i32> = read_file(file_path).chars().map(|c| if c == '>' {1} else {-1} ).collect();
    print!("{:?}", data)
}