use aoc2022::read_file;

fn main()
{
    println!("{}", part1("test_input.txt"));
    println!("{}", part1("input.txt"));
    // println!("{}", part2("test_input.txt"));
    // println!("{}", part2("input.txt"));
}

fn get_stacks(input: String) -> Vec<Vec<char>>
{
    let mut stacks:Vec<Vec<char>> = Vec::new();
    let mut num_of_stacks = input.lines().next().unwrap().len() as u32;
    num_of_stacks += 1;
    num_of_stacks /= 4;

    for _i in 0..(num_of_stacks)
    {
        stacks.push(Vec::<char>::new());
    }

    let data = String::from(input.split("move").collect::<Vec<&str>>()[0]);

    for line in data.lines()
    {
        let mut pos = 0;

        for character in line.chars()
        {
            if character.is_alphabetic()
            {
                let stack = (pos - 1)/4;
                if stack < stacks.len()
                {
                    stacks[stack].push(character);
                }
            }
            pos += 1;
        }
    }

    println!("Number of stacks: {} -> {}", num_of_stacks, stacks.len());

    let mut row = 1;
    for stack in &stacks
    {
        print!("Stack {}: ", row);
        for character in stack
        {
            print!("[{}] ", character)
        }
        row += 1;
        println!()
    }

    return stacks
}

fn part1(file_path: &str) -> String
{
    let input = read_file(file_path);
    let mut stacks = get_stacks(input);

    return String::from("");
}