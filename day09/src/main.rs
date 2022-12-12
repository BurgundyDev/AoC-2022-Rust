use std::{collections::HashSet};

use aoc2022::read_file;

fn main()
{
    println!("{}", part1("test_input_p1.txt"));
    println!("{}", part1("input.txt"));
    println!("{}", part2("test_input_p2.txt"));
    println!("{}", part2("input.txt"));
}

#[derive(Clone, Copy, Debug)]
struct End
{
    x: i32,
    y: i32
}

impl End
{
    fn move_head(&mut self, direction: &str, steps: i32, acceptable_distance: i32, mut tails: Vec<&mut End>) -> HashSet::<Vec<i32>>
    {
        let mut base_move: Vec<i32> = [0, 0].to_vec();

        let mut tail_positions = HashSet::<Vec<i32>>::new();

        match direction
        {
            "R"=> base_move[0] += 1,
            "L"=> base_move[0] -= 1,
            "U"=> base_move[1] += 1,
            "D"=> base_move[1] -= 1,
            _=> print!("kuuuurwa mać"),
        }

        for _i in 0..steps
        {
            self.x += &base_move[0];
            self.y += &base_move[1];

            let mut curr_head = &mut self.clone();

            for tail in &mut tails
            {
                curr_head.check_move(tail, acceptable_distance);
                curr_head = tail;
            }
            tail_positions.insert(vec![tails[tails.len()-1].x, tails[tails.len()-1].y]);
        }

        tail_positions
    }
    fn check_move(&mut self, tail: &mut End, acceptable_distance: i32) -> Vec<i32>
    {
        if self.x - tail.x == 0 ||  self.y - tail.y == 0
            {
                if self.x - tail.x > acceptable_distance && self.y - tail.y == 0
                {
                    tail.x += 1
                } else if self.x - tail.x < -acceptable_distance && self.y - tail.y == 0
                {
                    tail.x -= 1
                } else if self.y - tail.y > acceptable_distance && self.x - tail.x == 0
                {
                    tail.y += 1
                } else if self.y - tail.y < -acceptable_distance && self.x - tail.x == 0
                {
                    tail.y -= 1
                }
            } else if !((self.x - tail.x).abs() == 1 && (self.y - tail.y).abs() == 1)
            {
                let distance = [self.x - tail.x, self.y - tail.y].to_vec();
                // print!("Distance: {:?}", distance);
                let mut move_tail = [0, 0].to_vec();

                if distance[0] > 0
                {
                    move_tail[0] = 1
                } else if distance[0] < 0
                {
                    move_tail[0] = -1
                }
                if distance[1] > 0
                {
                    move_tail[1] = 1
                } else if distance[1] < 0
                {
                    move_tail[1] = -1
                }
                
                tail.x += move_tail[0];
                tail.y += move_tail[1];
            }

            let tail_pos = vec![tail.x, tail.y];
            tail_pos
    }
}

fn part1(file_path: &str) -> i32
{
    let input = read_file(file_path);

    let mut unique_pos: HashSet::<Vec<i32>> = HashSet::new();

    let mut head = End
    {
        x: 0,
        y: 0
    };

    let mut tail = End
    {
        x: 0,
        y: 0
    };

    for line in input.lines()
    {

        let direction = line.split(" ").collect::<Vec<&str>>()[0];
        let steps: i32 = line.split(" ").collect::<Vec<&str>>()[1].parse().unwrap();
        
        let line_pos = head.move_head(direction, steps, 1, vec![&mut tail]);

        unique_pos.extend(line_pos);
    }

    unique_pos.len().try_into().unwrap()

}
fn part2(file_path: &str) -> i32
{
    let input = read_file(file_path);

    let mut unique_pos: HashSet::<Vec<i32>> = HashSet::new();

    let mut head = End
    {
        x: 0,
        y: 0
    };
    let mut tail1 = head.clone();
    let mut tail2 = head.clone();
    let mut tail3 = head.clone();
    let mut tail4 = head.clone();
    let mut tail5 = head.clone();
    let mut tail6 = head.clone();
    let mut tail7 = head.clone();
    let mut tail8 = head.clone();
    let mut tail9 = head.clone();

    for line in input.lines()
    {

        let direction = line.split(" ").collect::<Vec<&str>>()[0];
        let steps: i32 = line.split(" ").collect::<Vec<&str>>()[1].parse().unwrap();
        
        let line_pos1 = head.move_head(direction, steps, 1, vec![&mut tail1, &mut tail2, &mut tail3, &mut tail4, &mut tail5, &mut tail6, &mut tail7, &mut tail8, &mut tail9]);
        unique_pos.extend(line_pos1);
    }
    unique_pos.len().try_into().unwrap()

}