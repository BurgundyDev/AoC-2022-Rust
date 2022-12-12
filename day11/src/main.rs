use std::collections::HashSet;

use aoc2022::read_file;

fn main() {
    println!("{}", part1("test_input.txt"));
    println!("{}", part1("input.txt"));
    println!("{}", part2("test_input.txt"));   
    println!("{}", part2("input.txt"));
}

#[derive(Debug, Clone, Copy)]
enum Operation {
    Add,
    Mul,
    AddOld,
    MulOld,
}

#[derive(Debug, Clone)]
struct Monkey {
    items: Vec<u64>,
    operator: u64,
    operation: Operation,
    test: u64,
    true_target: usize,
    false_target: usize,
    monkey_business: u64,
}

impl Monkey {
    fn test_items(&mut self, worry: bool, lcm: u64) -> Vec<Vec<u64>> {
        let mut to_true: Vec<u64> = Vec::new();
        let mut to_false: Vec<u64> = Vec::new();
        for item in self.items.iter_mut() {
            let mut curr_item = *item;
            match self.operation {
                Operation::Add => curr_item += self.operator,
                Operation::Mul => curr_item *= self.operator,
                Operation::AddOld => curr_item += curr_item,
                Operation::MulOld => curr_item *= curr_item,
            };

            if worry {
                curr_item %= lcm
            } else {    
                curr_item = curr_item / 3
            }

            if curr_item % self.test == 0 {
                to_true.push(curr_item);
            } else {
                to_false.push(curr_item)
            }
            self.monkey_business += 1
        }
        self.items = Vec::new();
        vec![to_true, to_false]
    }
}

fn part1(file_path: &str) -> u64 {
    let input = read_file(file_path);

    // init monkeys!
    let data: Vec<&str> = input.split("\r\n\r\n").collect();

    let mut monkeys: Vec<Monkey> = Vec::new();

    for dec in data {
        let monkey_info: Vec<&str> = dec.lines().collect();

        let prepped_operator = match monkey_info[2].strip_prefix("  Operation: new = old ").unwrap().split(" ").collect::<Vec<&str>>()[1].parse() {
            Ok(operator) => operator,
            Err(_error) => 1,
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
            monkey_business: 0
        };
        monkeys.push(monkey);
    }

    for _round in 0..20 {
        for i in 0..monkeys.len() {
            let mut items_passed = monkeys[i].test_items(false, 0);
            let &Monkey { true_target, false_target, .. } = &monkeys[i];
            monkeys[true_target].items.append(&mut items_passed[0]);
            monkeys[false_target].items.append(&mut items_passed[1]);
        }
    }
    
    let mut all_monkey_business: Vec<u64> = Vec::new();
    for monke in monkeys {
        // println!("{:?} - {}", monke.items, monke.monkey_business);
        all_monkey_business.push(monke.monkey_business.try_into().unwrap())
    }
    all_monkey_business.sort();
    all_monkey_business.reverse();
    println!("{:?}", all_monkey_business);

    all_monkey_business[0] * all_monkey_business[1]

}

fn part2(file_path: &str) -> u64 {
    let input = read_file(file_path);

    let mut tests: HashSet<u64> = HashSet::new();
    // init monkeys!
    let data: Vec<&str> = input.split("\r\n\r\n").collect();

    let mut monkeys: Vec<Monkey> = Vec::new();

    for dec in data {
        let monkey_info: Vec<&str> = dec.lines().collect();

        let prepped_operator = match monkey_info[2].strip_prefix("  Operation: new = old ").unwrap().split(" ").collect::<Vec<&str>>()[1].parse() {
            Ok(operator) => operator,
            Err(_error) => 1,
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
            monkey_business: 0
        };
        tests.insert(monkey.test.clone());
        monkeys.push(monkey);
    }

    let lcm = tests.iter().copied().reduce(|a, b| a*b).unwrap();

    for _round in 0..10000 {
        for i in 0..monkeys.len() {
            let mut items_passed = monkeys[i].test_items(true, lcm);
            let &Monkey { true_target, false_target, .. } = &monkeys[i];
            monkeys[true_target].items.append(&mut items_passed[0]);
            monkeys[false_target].items.append(&mut items_passed[1]);
        }
    }
    
    let mut all_monkey_business: Vec<u64> = Vec::new();
    for monkey in monkeys {
        // println!("{:?} - {}", monke.items, monke.monkey_business);
        all_monkey_business.push(monkey.monkey_business)
    }
    all_monkey_business.sort();
    all_monkey_business.reverse();
    println!("{:?}", all_monkey_business);

    // println!("{:?}", tests);

    all_monkey_business[0] * all_monkey_business[1]

}