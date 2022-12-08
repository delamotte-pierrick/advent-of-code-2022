use std::borrow::Borrow;
use crate::utils::read_lines;

pub(crate) fn day_8_1() {
    if let Ok(lines) = read_lines("./data/input_8.txt") {
        let inputs: Vec<String> = lines.map(|line| line.unwrap()).collect();
        let mut visible_trees = create_matrices(inputs[0].len() as i32, inputs.len() as i32, 0);
        let input_matrix = parse_inputs(inputs);

        for i in 0..visible_trees.len() {
            for j in 0..visible_trees[0].len() {
                let east: Vec<&i32> = input_matrix[i][0..j].iter().collect();
                let west: Vec<&i32> = input_matrix[i][j + 1..].iter().collect();
                let north: Vec<&i32> = input_matrix[0..i].iter().map(|x| x[j].borrow()).collect();
                let south: Vec<&i32> = input_matrix[i + 1..].iter().map(|x| x[j].borrow()).collect();

                [east, west, north, south].iter().for_each(|x| {
                    let result = x.iter().max().unwrap_or(&&-1);
                    if result < &&input_matrix[i][j] {
                        visible_trees[i][j] = 1;
                    }
                });
            }
        }

        // for tree in &visible_trees {
        //     println!("{:?}", tree);
        // }

        println!("Number of visible tree: {}", visible_trees.iter().map(|x| x.iter().sum::<i32>()).sum::<i32>());
    }
}

pub(crate) fn day_8_2() {
    if let Ok(lines) = read_lines("./data/input_8.txt") {
        let inputs: Vec<String> = lines.map(|line| line.unwrap()).collect();
        let mut visible_trees = create_matrices(inputs[0].len() as i32, inputs.len() as i32, 1);
        let input_matrix = parse_inputs(inputs);

        for i in 1..visible_trees.len() - 1 {
            for j in 1..visible_trees[0].len() - 1 {
                let mut east: Vec<&i32> = input_matrix[i][0..j].iter().collect();
                let west: Vec<&i32> = input_matrix[i][j + 1..].iter().collect();
                let mut north: Vec<&i32> = input_matrix[0..i].iter().map(|x| x[j].borrow()).collect();
                let south: Vec<&i32> = input_matrix[i + 1..].iter().map(|x| x[j].borrow()).collect();

                east.reverse();
                north.reverse();

                [east, north, west, south].iter().for_each(|x| {
                    let mut result = 0;
                    for y in x {
                        if y < &&input_matrix[i][j] {
                            result += 1;
                            continue;
                        } else if y >= &&input_matrix[i][j] {
                            result += 1;
                        }
                        break;
                    }

                    if result == 0 {
                        result = 1;
                    }
                    visible_trees[i][j] *= result;
                });
            }
        }

        println!("Number of visible tree: {}", visible_trees.iter().map(|x| x.iter().max().unwrap_or(&0)).max().unwrap_or(&0));
    }
}

fn create_matrices(x: i32, y: i32, value: i32) -> Vec<Vec<i32>> {
    let mut matrix = Vec::new();
    for _ in 0..y {
        let mut row = Vec::new();
        for _ in 0..x {
            row.push(value);
        }
        matrix.push(row);
    }
    return matrix;
}

fn parse_inputs(inputs: Vec<String>) -> Vec<Vec<i32>> {
    let mut matrix = Vec::new();
    for input in inputs {
        let mut row = Vec::new();
        for c in input.chars() {
            row.push(c.to_digit(10).unwrap() as i32);
        }
        matrix.push(row);
    }
    return matrix;
}
