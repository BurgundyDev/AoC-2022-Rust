use aoc2022::read_file;

fn main()
{
    println!("Visible trees: {}", part1("test_input.txt"));
    println!("Visible trees: {}", part1("input.txt"));
    println!("Best view score: {}", part2("test_input.txt"));
    println!("Best view score: {}", part2("input.txt"));
}

fn part1(file_path: &str) -> u32
{
    let input = read_file(file_path);
    let mut treemap: Vec<Vec<u32>> = Vec::new();

    for line in input.lines()
    {
        let mut row: Vec<u32> = Vec::<u32>::new();
        for character in line.chars()
        {
            row.push(character.to_digit(10).unwrap())
        }
        treemap.push(row);
    }

    // debug print
    for row in &treemap
    {
        for tree in row
        {
            print!("{}", tree)
        }
        println!()
    }
    println!();

    let mut visible_trees: u32 = 0;
    let mut invisible_trees: u32 = 0;

    for (r_index, row) in treemap.iter().enumerate()
    {
        if r_index == 0 || r_index == treemap.len() - 1
        {
            visible_trees += row.len() as u32;
            for _i in row
            {
                print!("v")
            }
        } else 
        {
            for (t_index, tree) in row.iter().enumerate()
            {
                if t_index == 0 || t_index == row.len() - 1
                {
                    visible_trees += 1;
                    print!("v")
                } else 
                {
                    let mut visible_top = true;
                    let mut visible_bottom = true;
                    let mut visible_right = true;
                    let mut visible_left = true;
                    for i in 1..r_index+1
                    {
                        if &treemap[r_index - i][t_index] >= tree
                        {
                            visible_top = false;
                        }
                    }

                    for i in 1..treemap.len() - r_index
                    {
                        if &treemap[r_index + i][t_index] >= tree
                        {
                            visible_bottom = false;
                        }
                    }

                    for i in 1..t_index+1
                    {
                        if &row[t_index-i] >= tree
                        {
                            visible_left = false;
                        }
                    }

                    for i in 1..row.len() - t_index
                    {
                        if &row[t_index + i] >= tree
                        {
                            visible_right = false;
                        }
                    }

                    if visible_top || visible_bottom || visible_left || visible_right
                    {
                        visible_trees += 1;
                        print!("v")
                    } else {
                        invisible_trees += 1;
                        print!("i")
                    }
                }
            }
        }
        
        println!()
    }
    println!("Invisible trees: {invisible_trees}");
    return visible_trees
}

fn part2(file_path: &str) -> i32
{
    let input = read_file(file_path);
    let mut treemap: Vec<Vec<u32>> = Vec::new();

    for line in input.lines()
    {
        let mut row: Vec<u32> = Vec::<u32>::new();
        for character in line.chars()
        {
            row.push(character.to_digit(10).unwrap())
        }
        treemap.push(row);
    }

    // debug print
    for row in &treemap
    {
        for tree in row
        {
            print!("{}", tree)
        }
        println!()
    }
    println!();


    let mut max_score = 0;
    for (r_index, row) in treemap.iter().enumerate()
    {
        for (t_index, tree) in row.iter().enumerate()
        {
            let mut score_top = 0;
            let mut score_bot = 0;
            let mut score_left = 0;
            let mut score_right = 0;

            let mut blocked = false;
            for i in 1..r_index+1
            {
                if &treemap[r_index - i][t_index] >= tree
                {
                    if !blocked
                    {
                        score_top += 1;
                        blocked = true
                    }
                } else if !blocked
                {
                    score_top += 1
                }
            }

            blocked = false;
            for i in 1..treemap.len() - r_index
            {
                if &treemap[r_index + i][t_index] >= tree
                {
                    if !blocked
                    {
                        score_bot += 1;
                        blocked = true
                    }
                } else if !blocked
                {
                    score_bot += 1
                }
            }

            blocked = false;
            for i in 1..t_index+1
            {
                if &row[t_index-i] >= tree
                {
                    if !blocked
                    {
                        score_left += 1;
                        blocked = true
                    }
                } else if !blocked
                {
                    score_left += 1
                }
            }
            
            blocked = false;
            for i in 1..row.len() - t_index
            {
                if &row[t_index + i] >= tree
                {
                    if !blocked
                    {
                        score_right += 1;
                        blocked = true
                    }
                } else if !blocked
                {
                    score_right += 1
                }
            }

            let score = score_bot * score_top * score_left * score_right;

            if score > max_score
            {
                max_score = score;
            }
        }
    }
    println!();

    max_score
}