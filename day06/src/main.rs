use std::hash::Hash;
use std::collections::HashSet;

use aoc2022::read_file;

fn main()
{
    println!("{}", part1("test_input.txt"));
    println!("{}", part1("input.txt"));
    println!("{}", part2("test_input.txt"));
    println!("{}", part2("input.txt"));
}

fn has_unique_elements<T>(iter: T) -> bool
where
    T: IntoIterator,
    T::Item: Eq + Hash,
{
    let mut uniq = HashSet::new();
    iter.into_iter().all(move |x| uniq.insert(x))
}

fn part1(file_path: &str) -> u32
{
    let input = read_file(file_path);
    println!("{}", input);
    let data: Vec<char> = input.chars().collect();

    let mut clear = false;
    let mut i: usize = 0;
    let mut cur_str = Vec::<char>::from(&data[i..i+4]);
    for character in &cur_str
    {
        print!("{}", character);
    }
    println!();

    while !clear
    {
        if has_unique_elements(&cur_str)
        {
            clear = true;
            for character in &cur_str
            {
                print!("{}", character);
            }
            println!()
        } else
        {
            i += 1;
            cur_str = Vec::<char>::from(&data[i..i+4])
        }
    }

    return (i+4).try_into().unwrap()
}

fn part2(file_path: &str) -> u32
{
    let input = read_file(file_path);
    println!("{}", input);
    let data: Vec<char> = input.chars().collect();

    let mut clear = false;
    let mut i: usize = 0;
    let message_size = 14;
    let mut cur_str = Vec::<char>::from(&data[i..i+message_size]);
    for character in &cur_str
    {
        print!("{}", character);
    }
    println!();

    while !clear
    {
        if has_unique_elements(&cur_str)
        {
            clear = true;
            for character in &cur_str
            {
                print!("{}", character);
            }
            println!()
        } else
        {
            i += 1;
            cur_str = Vec::<char>::from(&data[i..i+message_size])
        }
    }

    return (i+message_size).try_into().unwrap()
}