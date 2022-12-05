use aoc2022::read_file;

fn main()
{
    println!("{}", part1("test_input.txt"));
    println!("{}", part1("input.txt"));
    println!("{}", part2("test_input.txt"));
    println!("{}", part2("input.txt"));
}

fn get_stacks(input: &String) -> Vec<Vec<char>>
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

    for stack in &mut stacks
    {
        stack.reverse();
    }

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

fn get_orders(input: String) -> Vec<Vec<i32>>
{
    let mut orders:Vec<Vec<i32>> = Vec::new();

    for line in input.lines()
    {
        if line.starts_with("move")
        {
            let mut line_formatted = line.replace("move ", "");
            line_formatted = line_formatted.replace("from ", "");
            line_formatted = line_formatted.replace("to ", "");

            let order: Vec<i32> = line_formatted.split(" ").map(|n| n.parse().unwrap()).collect();
            orders.push(order);
        }
    }

    orders
}

fn part1(file_path: &str) -> String
{
    let input = read_file(file_path);
    let mut stacks = get_stacks(&input);

    let orders = get_orders(input);

    for order in orders
    {
        let mut moves = 0;

        let from: usize = (order[1] - 1) as usize;
        let to: usize = (order[2] - 1) as usize;
        while moves < order[0]
        {
            let moved_crate = stacks[from].pop().unwrap();
            stacks[to].push(moved_crate);
            moves += 1
        }
    }

    println!("Stacks after mutation:");
    let mut stack_num = 0;
    for stack in &stacks
    {
        print!("Stack {}: ", stack_num+1);
        for character in stack
        {
            print!("[{}] ", character);
        }
        stack_num += 1;
        println!()
    }

    let mut tops = String::from(""); 
    for stack in &mut stacks
    {
        tops += &stack.pop().unwrap().to_string();
    }

    return tops;
}

fn part2(file_path: &str) -> String
{
    let input = read_file(file_path);
    let mut stacks = get_stacks(&input);

    let orders = get_orders(input);

    for order in orders
    {
        let mut moves = 0;

        let from: usize = (order[1] - 1) as usize;
        let to: usize = (order[2] - 1) as usize;

        let mut moved_crates: Vec<char> = Vec::<char>::new();
        while moves < order[0]
        {
            moved_crates.push(stacks[from].pop().unwrap());
            moves += 1
        }
        moved_crates.reverse();
        stacks[to].append(&mut moved_crates);
    }

    println!("Stacks after mutation:");
    let mut stack_num = 0;
    for stack in &stacks
    {
        print!("Stack {}: ", stack_num+1);
        for character in stack
        {
            print!("[{}] ", character);
        }
        stack_num += 1;
        println!()
    }

    let mut tops = String::from(""); 
    for stack in &mut stacks
    {
        tops += &stack.pop().unwrap().to_string();
    }

    return tops;
}