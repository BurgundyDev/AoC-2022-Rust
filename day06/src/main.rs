use std::hash::Hash;
use std::collections::HashSet;

use aoc2022::read_file;

fn main()
{
    println!("{}", day06("test_input.txt", 4));
    println!("{}", day06("input.txt", 4));
    println!("{}", day06("test_input.txt", 14));
    println!("{}", day06("input.txt", 14));
}

fn has_unique_elements<T>(iter: T) -> bool
where
    T: IntoIterator,
    T::Item: Eq + Hash,
{
    let mut uniq = HashSet::new();
    iter.into_iter().all(move |x| uniq.insert(x))
}

fn day06(file_path: &str, message_size: usize) -> u32
{
    let input = read_file(file_path);
    println!("{}", input);
    let data: Vec<char> = input.chars().collect();

    let mut clear = false;
    let mut i: usize = 0;
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