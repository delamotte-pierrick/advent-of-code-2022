use crate::utils::{read_lines};

pub(crate) fn day_9_1() {
    let instructions: Vec<Vec<String>> = read_input("./data/input_9.txt");
    let positions_visited: Vec<Position> = resolve_inputs(instructions.clone(), 1);

    println!("Number of position visited by tail 1: {}", positions_visited.len());
}

pub(crate) fn day_9_2() {
    let instructions: Vec<Vec<String>> = read_input("./data/input_9.txt");
    let positions_visited: Vec<Position> = resolve_inputs(instructions.clone(), 9);

    println!("Number of position visited by tail 9: {}", positions_visited.len());
}

fn resolve_inputs(instructions: Vec<Vec<String>>, number_of_tails: i32) -> Vec<Position> {
    let mut positions_visited: Vec<Position> = Vec::new();
    let mut position_tails: Vec<Position> = Vec::new();
    let mut position_head = Position { x: 0, y: 0 };
    for _ in 0..number_of_tails {
        position_tails.push(position_head.clone());
    }

    let mut is_first_instruction = true;
    for instruction in instructions {
        let direction = instruction[0].clone();
        let distance = instruction[1].parse::<i32>().unwrap();
        for _ in 0..distance {
            if direction == "U" {
                position_head.y += 1;
            } else if direction == "D" {
                position_head.y -= 1;
            } else if direction == "L" {
                position_head.x -= 1;
            } else if direction == "R" {
                position_head.x += 1;
            } else {
                panic!("Unknown direction");
            }

            if !is_first_instruction {
                for tail in 0..position_tails.len() {
                    position_tails[tail] = resolve_position(
                        if tail == 0 { position_head } else { position_tails[tail - 1] },
                        position_tails[tail],
                    );
                }
            }

            positions_visited.push(position_tails.last().unwrap().clone());
            is_first_instruction = false;
        }
    }

    positions_visited.sort(); //If I don't sort, dedup doesn't work
    positions_visited.dedup();

    return positions_visited.clone();
}

fn read_input(file_path: &str) -> Vec<Vec<String>> {
    let lines = read_lines(file_path).unwrap();
    return lines.map(|l| {
        let line = l.unwrap();
        return line.split(" ").map(|x| x.to_string()).collect::<Vec<String>>();
    }).collect();
}

fn resolve_position(position_head: Position, position_tail: Position) -> Position {
    let x_diff = position_head.x - position_tail.x;
    let y_diff = position_head.y - position_tail.y;

    let mut new_position = position_tail.clone();

    if x_diff > 0 {
        new_position.x += 1;
    } else if x_diff < 0 {
        new_position.x -= 1;
    }

    if y_diff > 0 {
        new_position.y += 1;
    } else if y_diff < 0 {
        new_position.y -= 1;
    }

    if new_position == position_head {
        new_position = position_tail.clone();
    }

    return new_position;
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
struct Position {
    x: i32,
    y: i32,
}
