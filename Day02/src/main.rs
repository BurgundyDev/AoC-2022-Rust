/*
A/X - Rock
B/Y - Paper
C/Z - Scissors
*/

use std::fs;

fn main()
{
    day02("test_input.txt");
    day02("input.txt");
}

fn day02(file_path: &str) -> u32
{
    let mut points: u32 = 0;
    let input: String = if let Ok(input) = fs::read_to_string(file_path) { input } else { String::from("Loading failed!") };
    
    for line in input.lines()
    {
        if line.contains("A")
        {
            println!("Enemy played Rock");
            if line.contains("X")
            {
                println!("You played Rock");
                points += 1 + 3
                
            }else if line.contains("Y")
            {
                println!("You played Paper");
                points += 2 + 6
            } else if  line.contains("Z")
            {
                println!("You played Scissors");
                points += 3
            }

        }else if line.contains("B")
        {
            println!("Enemy played Paper");
            if line.contains("X")
            {
                println!("You played Rock");
                points += 1 + 0
                
            }else if line.contains("Y")
            {
                println!("You played Paper");
                points += 2 + 3
            } else if  line.contains("Z")
            {
                println!("You played Scissors");
                points += 3 + 6
            }
        } else if  line.contains("C")
        {
            println!("Enemy played Scissors");
            if line.contains("X")
            {
                println!("You played Rock");
                points += 1 + 6
                
            }else if line.contains("Y")
            {
                println!("You played Paper");
                points += 2 + 0
            } else if  line.contains("Z")
            {
                println!("You played Scissors");
                points += 3 + 3
            }
        }
    }
    println!("{}", points);
    points
}