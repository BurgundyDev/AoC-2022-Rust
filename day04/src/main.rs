use aoc2022::read_file;

fn main()
{
    println!("Amount of complete overlaps: {}", part1("test_input.txt"));
    println!("Amount of complete overlaps: {}", part1("input.txt"));
    println!("Amount of overlaps: {}", part2("test_input.txt"));
    println!("Amount of overlaps: {}", part2("input.txt"));
}

fn part1(file_path: &str) -> u32
{
    let input = read_file(file_path);
    let mut data:Vec<Vec<u32>> = Vec::new();
    let mut overlap: u32 = 0;
    
    for line in input.lines()
    {
        let mut row = Vec::<u32>::new();

        let areas: Vec<&str> = line.split(',').collect();

        for area in areas
        {
            let nums:Vec<&str> = area.split("-").collect();
            row.push(nums[0].parse::<u32>().unwrap());
            row.push(nums[1].parse::<u32>().unwrap())
        }
        
        data.push(row);
    }

    for row in data
    {
        if (row[0] >= row[2] && row[1] <= row[3]) || (row[0] <= row[2] && row[1] >= row[3])
        {
            overlap += 1;
        }
    }
    
    overlap
}

fn part2(file_path: &str) -> u32
{
    let input = read_file(file_path);
    let mut data:Vec<Vec<u32>> = Vec::new();
    let mut overlap: u32 = 0;
    
    for line in input.lines()
    {
        let mut row = Vec::<u32>::new();

        let areas: Vec<&str> = line.split(',').collect();

        for area in areas
        {
            let nums:Vec<&str> = area.split("-").collect();
            row.push(nums[0].parse::<u32>().unwrap());
            row.push(nums[1].parse::<u32>().unwrap())
        }
        
        data.push(row);
    }

    for row in data
    {
        if row[0] <= row[3] && row[1] >= row[2]
        {
            overlap += 1;
        }
    }

    overlap
}