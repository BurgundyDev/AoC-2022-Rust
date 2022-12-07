use std::collections::{VecDeque, HashMap};

use aoc2022::read_file;

fn main()
{
    println!("Sum of all ddirectories smaller than 1000000: {}", part1("input.txt"));
    println!("Smallest deletable dir: {}", part2("input.txt"));
}

fn part1(file_path: &str) -> i32
{
    let input = read_file(file_path);

    let mut curr_path: String = "".to_string();
    let mut entered_paths: VecDeque<String> = VecDeque::new();

    let mut directories: HashMap<String, i32> = HashMap::new();
    
    for line in input.lines()
    {
        if line.contains("/")
        {
            curr_path = "/".to_string();
            // println!("{}", curr_path);
        } else if line.contains("cd ..")
        {
            curr_path = curr_path.strip_suffix(&entered_paths.pop_back().unwrap()).unwrap().to_string();
            directories.insert(curr_path.clone(), if directories.contains_key(&curr_path.clone()) {directories[&curr_path.clone()]} else {0});
            // println!("{}", curr_path);
        } else if line.starts_with("$ cd ")
        {
            let mut temp = line.strip_prefix("$ cd ").unwrap().to_string();
            temp = temp + "/";
            directories.insert(curr_path.clone(), if directories.contains_key(&curr_path.clone()) {directories[&curr_path.clone()]} else {0});
            // println!("New path used: {}", temp);
            curr_path += &temp;
            entered_paths.push_back(temp.clone()); 
            // println!("{}", curr_path);
        } else if !line.starts_with("$") && !line.starts_with("dir")
        {
            let value: i32 = line.split(" ").collect::<Vec<&str>>()[0].parse().unwrap();
            directories.insert(curr_path.clone(), value + if directories.contains_key(&curr_path.clone()) {directories[&curr_path.clone()]} else {0});
            // println!("{}", value)
        } else
        {
            directories.insert(curr_path.clone(), if directories.contains_key(&curr_path.clone()) {directories[&curr_path.clone()]} else {0});
        }
    }

    let mut full_directories: HashMap<String, i32> = HashMap::new();

    for (dir, val) in &directories
    {
        println!("{dir}: \"{val}\"");
        let mut value = 0;
        for (checked_dir, checked_val) in &directories
        {
            if checked_dir.contains(dir)
            {
                // println!("{dir} contains {checked_dir}");
                value += checked_val;
            }
        }

        full_directories.insert(dir.to_string(), value);
    }
    
    let mut final_val = 0;
    for (dir, val) in &full_directories
    {
        // println!("{dir}: \"{val}\"");
        if *val <= 100000
        {
            println!("Used: {dir}: \"{val}\"");
            final_val += *val;
        } else
        {
            println!("Unused: {dir}: \"{val}\"");
        }
    }

    final_val
}

fn part2(file_path: &str) -> i32
{
    let input = read_file(file_path);

    let mut curr_path: String = "".to_string();
    let mut entered_paths: VecDeque<String> = VecDeque::new();

    let mut directories: HashMap<String, i32> = HashMap::new();
    
    for line in input.lines()
    {
        if line.contains("/")
        {
            curr_path = "/".to_string();
            // println!("{}", curr_path);
        } else if line.contains("cd ..")
        {
            curr_path = curr_path.strip_suffix(&entered_paths.pop_back().unwrap()).unwrap().to_string();
            directories.insert(curr_path.clone(), if directories.contains_key(&curr_path.clone()) {directories[&curr_path.clone()]} else {0});
            // println!("{}", curr_path);
        } else if line.starts_with("$ cd ")
        {
            let mut temp = line.strip_prefix("$ cd ").unwrap().to_string();
            temp = temp + "/";
            directories.insert(curr_path.clone(), if directories.contains_key(&curr_path.clone()) {directories[&curr_path.clone()]} else {0});
            // println!("New path used: {}", temp);
            curr_path += &temp;
            entered_paths.push_back(temp.clone()); 
            // println!("{}", curr_path);
        } else if !line.starts_with("$") && !line.starts_with("dir")
        {
            let value: i32 = line.split(" ").collect::<Vec<&str>>()[0].parse().unwrap();
            directories.insert(curr_path.clone(), value + if directories.contains_key(&curr_path.clone()) {directories[&curr_path.clone()]} else {0});
            // println!("{}", value)
        } else
        {
            directories.insert(curr_path.clone(), if directories.contains_key(&curr_path.clone()) {directories[&curr_path.clone()]} else {0});
        }
    }

    let mut full_directories: HashMap<String, i32> = HashMap::new();

    for (dir, val) in &directories
    {
        println!("{dir}: \"{val}\"");
        let mut value = 0;
        for (checked_dir, checked_val) in &directories
        {
            if checked_dir.contains(dir)
            {
                // println!("{dir} contains {checked_dir}");
                value += checked_val;
            }
        }

        full_directories.insert(dir.to_string(), value);
    }

    let base_fs = 70000000;
    let space_upd = 30000000;
    let space_needed = space_upd - base_fs + full_directories["/"];
    let mut smallest_size = base_fs;
    println!("Space needed: {space_needed}");
    
    for (_dir, val) in &full_directories
    {
        if *val < smallest_size && *val > space_needed
        {
            smallest_size = *val
        }
    }
    
    smallest_size
}