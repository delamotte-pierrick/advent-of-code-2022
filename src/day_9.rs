use crate::utils::{read_lines};

pub(crate) fn day_9_1() {
    if let Ok(lines) = read_lines("./data/input_9.txt") {
        let mut positions_visited: Vec<Position> = Vec::new();
        let mut position_tail = Position { x: 0, y: 0 };
        let mut position_head = position_tail.clone();

        let instructions: Vec<Vec<String>> = lines.map(|l| {
            let line = l.unwrap();
            return line.split(" ").map(|x| x.to_string()).collect::<Vec<String>>();
        }).collect();

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
                    position_tail = resolve_position(position_head, position_tail);
                }
                positions_visited.push(position_tail.clone());
                is_first_instruction = false;
            }
        }

        positions_visited.sort();
        positions_visited.dedup();

        println!("Number of position visited: {}", positions_visited.len());
    }
}

pub(crate) fn day_9_2() {
    if let Ok(lines) = read_lines("./data/input_9.txt") {
        let mut positions_visited: Vec<Position> = Vec::new();
        let mut position_tails: Vec<Position> = Vec::new();
        let mut position_head = Position { x: 0, y: 0 };
        for _ in 1..=9 {
            position_tails.push(position_head.clone());
        }

        let instructions: Vec<Vec<String>> = lines.map(|l| {
            let line = l.unwrap();
            return line.split(" ").map(|x| x.to_string()).collect::<Vec<String>>();
        }).collect();

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
                        if tail == 0 {
                            position_tails[tail] = resolve_position(position_head, position_tails[tail]);
                        } else {
                            position_tails[tail] = resolve_position(position_tails[tail - 1], position_tails[tail]);
                        }
                    }
                }

                positions_visited.push(position_tails.last().unwrap().clone());
                is_first_instruction = false;
            }
        }

        positions_visited.sort();
        positions_visited.dedup();

        println!("Number of position visited: {}", positions_visited.len());
    }
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
