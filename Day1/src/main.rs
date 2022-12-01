use std::fs::File;
use std::io::{self, BufRead};

fn main()
{
    day1("test_input.txt");
    day1("input.txt");
}

fn read_file_by_line(file_path: &str) -> io::Result<io::Lines<io::BufReader<File>>>
{
    let file = File::open(file_path)?;
    Ok(io::BufReader::new(file).lines())
}

fn day1(file_path: &str)
{
    let mut elfs: Vec<u32> = Vec::<u32>::new();
    let mut current_elf: u32 = 0;

    if let Ok(lines) = read_file_by_line(file_path)
    {
        for line in lines
        {
            if let Ok(line_formatted) = line
            {
                if let Ok(food) = line_formatted.parse::<u32>()
                {
                    current_elf += food;
                }
                else
                {
                    elfs.push(current_elf);
                    current_elf = 0;
                }
            }
        }
        elfs.push(current_elf);
        current_elf = 0;

        elfs.sort();
        elfs.reverse();

        for calories in &elfs
        {
            println!("{}", calories)
        }
        println!("");

        println!("The top elf carries food of this many calories: {}", elfs[0]);

        let sum_of_top_3: u32 = elfs[0] + elfs[1] + elfs[2];

        println!("The top 3 elves carry: {}", sum_of_top_3);
    }
}