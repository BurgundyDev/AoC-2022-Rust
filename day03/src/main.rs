use std::str::Chars;

use aoc2022::read_file;

fn main()
{
    println!("{}", part1("test_input.txt"));
    println!("{}", part1("input.txt"));
    println!("{}", part2("test_input.txt"));
    println!("{}", part2("input.txt"));
}

fn part1(file_path: &str) -> i32
{
    let input = read_file(file_path);

    let mut data: Vec<Chars<>> = Vec::new();
    let mut fin_data: Vec<Vec<i32>> = Vec::new();

    let mut sum_of_priorities: i32 = 0;

    for line in input.lines()
    {
        data.push(line.chars())
    }

    for line in data
    {
        let mut line_of_ints: Vec<i32> = Vec::<i32>::new();

        for character in line
        {
            let mut char_as_int = character as i32;
            char_as_int -= 64;
            if char_as_int>26
            {
                char_as_int -= 32;
            }else
            {
                char_as_int += 26;
            }
            // print!("{} ", char_as_int);
            line_of_ints.push(char_as_int)
        }
        fin_data.push(line_of_ints);
        // println!();
    }

    for rucksack in fin_data
    {
        let (half1, half2) = rucksack.split_at(rucksack.len()/2);
        let mut repeated_numbers = Vec::<i32>::new();

        for number in half1
        {
            if half2.contains(number)
            {
                if !repeated_numbers.contains(number)
                {
                    repeated_numbers.push(*number);
                    println!("{}", number);
                    sum_of_priorities += *number;
                }
            }
        }
    }

    return sum_of_priorities;
}

fn part2(file_path: &str) -> i32
{
    let input = read_file(file_path);

    let mut data: Vec<Chars<>> = Vec::new();
    let mut fin_data: Vec<Vec<i32>> = Vec::new();

    let mut sum_of_priorities: i32 = 0;

    for line in input.lines()
    {
        data.push(line.chars())
    }

    for line in data
    {
        let mut line_of_ints: Vec<i32> = Vec::<i32>::new();

        for character in line
        {
            let mut char_as_int = character as i32;
            char_as_int -= 64;
            if char_as_int>26
            {
                char_as_int -= 32;
            }else
            {
                char_as_int += 26;
            }
            // print!("{} ", char_as_int);
            line_of_ints.push(char_as_int)
        }
        fin_data.push(line_of_ints);
        // println!();
    }

    let mut finit = 0;
    for rucksack in 0..(fin_data.len()/3)
    {
        let mut done = false;
        for number in &fin_data[finit]
        {
            if fin_data[finit + 1].contains(number) && fin_data[finit + 2].contains(number) && !done
            {
                sum_of_priorities += number;
                done = true;
            }
        }
        finit += 3;
    }

    return sum_of_priorities;
}