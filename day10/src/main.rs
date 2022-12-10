use aoc2022::read_file;

fn main()
{
    day10("test_input.txt");
    day10("input.txt");
}

fn day10(file_path: &str)
{
    let input = read_file(file_path);
    let mut data = input.lines().into_iter();

    let mut completed = false;
    let mut curr_order: &str = "";
    let mut add = 0;
    let mut pixels: Vec<bool> = Vec::new();

    let mut cycle = 1;
    let mut x = 1;
    let mut sum = 0;

    while !completed
    {
        if cycle == 20 || (cycle - 20) % 40 == 0
        {
            sum += cycle * x;
        }

        if (cycle - 1) % 40  == x || (cycle - 1) % 40 == x - 1 || (cycle - 1) % 40 == x + 1
        {
            pixels.push(true)
        } else
        {
            pixels.push(false)
        }
        cycle += 1;

        let mut finished = false;
        
        if curr_order == "addx"
        {
            x += add;
            finished = true;
            curr_order = ""
        } else if curr_order == "noop"
        {
            curr_order = ""
        }

        if !finished
        {
            let order: Vec<&str> = data.next().unwrap().split(" ").collect();
            curr_order = order[0];
            if curr_order != "noop"
            {
                add = order[1].parse().unwrap()
            }
        }
        if cycle == 241
        {
            completed = true
        }
    }

    for i in 0..6
    {
        for z in 0..40
        {
            if pixels[i * 40 + z] == true
            {
                print!("#")
            } else {
                print!(".")
            }
        }
        println!()
    }
    println!();
    println!("Sum of relevant signals equals {sum}");
    println!()
}
