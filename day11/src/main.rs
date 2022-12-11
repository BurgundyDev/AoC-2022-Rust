use aoc2022::read_file;

fn main() {
    part1("test_input.txt");
}

#[derive(Debug)]
enum Operation {
    Add,
    Mul,
    AddOld,
    MulOld,
}

struct Monkey {
    items: Vec<i32>,
    operator: i32,
    operation: Operation,
    test: i32,
    true_target: usize,
    false_target: usize,
}

fn unworry(item: i32) -> i32
{
    (item + 3 - 1) / 3
}

impl Monkey {
    fn test_items(&mut self) -> Vec<Vec<i32>> {
        let mut to_true: Vec<i32> = Vec::new();
        let mut to_false: Vec<i32> = Vec::new();
        for (index, item) in self.items.iter_mut().enumerate() {
            match self.operation {
                Operation::Add => *item += self.operator,
                Operation::Mul => *item *= self.operator,
                Operation::AddOld => *item += *item,
                Operation::MulOld => *item *= *item,
            };

            *item = *item / 3;

            if *item % self.test == 0 {
                to_true.push(*item);
            } else {
                to_false.push(*item)
            }
        }
        self.items = Vec::new();
        vec![to_true, to_false]
    }
}

fn part1(file_path: &str) {
    let input = read_file(file_path);

    // init monkeys!
    let data: Vec<&str> = input.split("\n\r\n").collect();

    let mut monkeys: Vec<Monkey> = Vec::new();

    for dec in data {
        let monkey_info: Vec<&str> = dec.lines().collect();

        let prepped_operator = match monkey_info[2].strip_prefix("  Operation: new = old ").unwrap().split(" ").collect::<Vec<&str>>()[1].parse() {
            Ok(operator) => operator,
            Err(error) => 1,
        };

        let prepped_operation: Operation = match monkey_info[2].strip_prefix("  Operation: new = old ").unwrap().split(" ").collect::<Vec<&str>>()[1] {
            "old" => match monkey_info[2].strip_prefix("  Operation: new = old ").unwrap().split(" ").collect::<Vec<&str>>()[0] {
                "*" => Operation::MulOld,
                "+" => Operation::AddOld,
                _ => Operation::Add,
            },
            _ => match monkey_info[2].strip_prefix("  Operation: new = old ").unwrap().split(" ").collect::<Vec<&str>>()[0] {
                "*" => Operation::Mul,
                "+" => Operation::Add,
                _ => Operation::Add,
            },
        };
        let monkey = Monkey {
            items: monkey_info[1].strip_prefix("  Starting items: ").unwrap().split(", ").map(|n| n.parse().unwrap()).collect(),
            operator: prepped_operator,
            operation: prepped_operation,
            test: monkey_info[3].strip_prefix("  Test: divisible by ").unwrap().parse().unwrap(),
            true_target: monkey_info[4].strip_prefix("    If true: throw to monkey ").unwrap().parse().unwrap(),
            false_target: monkey_info[5].strip_prefix("    If false: throw to monkey ").unwrap().parse().unwrap(),
        };
        monkeys.push(monkey);
    }

    for mut monkey in &mut monkeys {
        let mut items_passed = monkey.test_items();
    }

}