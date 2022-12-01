use std::fs;

fn main()
{
    day1("test_input.txt");
    day1("input.txt");
}

fn day1(file_path: &str)
{
    let mut elfs: Vec<u32> = Vec::<u32>::new();
    let mut current_elf: u32 = 0;
    let input: String = if let Ok(input) = fs::read_to_string(file_path) { input } else { String::from("Loading failed!") };
    
    for line in input.lines()
    {
        if let Ok(food) = line.parse::<u32>()
        {
            current_elf += food;
        } else
        {
            elfs.push(current_elf);
            current_elf = 0;
        }
    };

    elfs.push(current_elf);

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