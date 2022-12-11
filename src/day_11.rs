use evalexpr::{context_map, eval_int_with_context};
use regex::Regex;
use crate::utils::read_lines;

pub(crate) fn day_11_1() {
    let monkeys = parse_input("./data/input_11.txt");
    let monkey_inspections = simulate_monkeys(monkeys.clone(), 20, None);

    println!("Result: {}", monkey_inspections[0] * monkey_inspections[1]);
}

pub(crate) fn day_11_2() {
    let monkeys = parse_input("./data/input_11.txt");
    let modulo: i64 = monkeys.iter().map(|monkey| monkey.condition).product();

    let monkey_inspections = simulate_monkeys(monkeys.clone(), 10000, Some(modulo));

    println!("Result: {}", monkey_inspections[0] * monkey_inspections[1]);
}

fn simulate_monkeys(mut monkeys: Vec<Monkey>, rounds: i64, modulo: Option<i64>) -> Vec<i64> {
    for _ in 0..rounds {
        for i_monkey in 0..monkeys.len() {
            let monkey = monkeys[i_monkey].clone();
            for old in monkey.items.iter() {
                let context = context_map! {"old" => old.clone() as i64}.unwrap();
                let mut result = eval_int_with_context(&*monkey.operation, &context).unwrap() as i64;

                if let Some(modulo) = modulo {
                    result = result % modulo;
                } else {
                    result = result / 3;
                }

                if result % monkey.condition == 0 {
                    monkeys[monkey.cond_true].items.push(result);
                } else {
                    monkeys[monkey.cond_false].items.push(result);
                }
            }

            monkeys[i_monkey].number_inspected_items += monkeys[i_monkey].items.len() as i64;
            monkeys[i_monkey].items.clear();
        }
    }

    let mut monkey_inspections = monkeys.clone().into_iter().map(|monkey| monkey.number_inspected_items).collect::<Vec<i64>>();
    monkey_inspections.sort();
    monkey_inspections.reverse();

    return monkey_inspections.clone();
}

fn parse_input(file_path: &str) -> Vec<Monkey> {
    let mut monkeys: Vec<Monkey> = Vec::new();
    let mut current_monkey: Monkey = Monkey {
        operation: "".to_string(),
        items: vec![],
        condition: 0,
        cond_true: 0,
        cond_false: 0,
        number_inspected_items: 0,
    };
    let re_items = Regex::new(r"\d+").unwrap();

    if let Ok(lines) = read_lines(file_path) {
        for line in lines {
            let line = line.unwrap().trim().to_string();
            if line.starts_with("Monkey") {
                if current_monkey.operation != "" {
                    monkeys.push(current_monkey);
                }

                current_monkey = Monkey {
                    operation: "".to_string(),
                    items: vec![],
                    condition: 0,
                    cond_true: 0,
                    cond_false: 0,
                    number_inspected_items: 0,
                };
            }

            if line.starts_with("Starting items:") {
                for item in re_items.captures_iter(&line) {
                    current_monkey.items.push(item[0].to_string().parse::<i64>().unwrap());
                }
            }

            if line.starts_with("Operation: new =") {
                current_monkey.operation = line[17..].to_string();
            }

            if line.starts_with("Test: divisible by") {
                current_monkey.condition = line[19..].to_string().parse::<i64>().unwrap();
            }

            if line.starts_with("If true: throw to monkey") {
                current_monkey.cond_true = line[25..].parse().unwrap();
            }

            if line.starts_with("If false: throw to monkey") {
                current_monkey.cond_false = line[26..].parse().unwrap();
            }
        }
    }

    monkeys.push(current_monkey);

    return monkeys;
}

#[derive(Debug, Clone)]
struct Monkey {
    operation: String,
    items: Vec<i64>,
    condition: i64,
    cond_true: usize,
    cond_false: usize,
    number_inspected_items: i64,
}
