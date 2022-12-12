use std::collections::{HashMap, VecDeque};
use std::string::String;
use crate::utils::read_lines;

pub(crate) fn day_12_1() {
    let maze = parse_input("./data/input_12.txt");
    let mut start: Option<String> = None;
    let mut end: Option<String> = None;

    for (i, row) in maze.iter().enumerate() {
        if row.contains(&'S') {
            for (j, cell) in row.iter().enumerate() {
                if *cell == 'S' {
                    start = Option::from(i.to_string() + "," + &j.to_string());
                }
            }
        }

        if row.contains(&'E') {
            for (j, cell) in row.iter().enumerate() {
                if *cell == 'E' {
                    end = Option::from(i.to_string() + "," + &j.to_string());
                }
            }
        }
    }
    if start.is_none() || end.is_none() {
        panic!("No start or end found");
    }

    let path = pathfinder_bfs(maze.clone(), start.clone().unwrap(), end.clone().unwrap());

    println!("Result: {}", path);
    println!("Shortest path is: {} cell long", path.len());
}

pub(crate) fn day_12_2() {
    println!("Result 2: {}", 0);
}

fn pathfinder_bfs(maze: Vec<Vec<char>>, start: String, end: String) -> String {
    let mut queue: VecDeque<(String, String)> = VecDeque::from([("".to_string(), start)]);
    let mut visited: Vec<String> = Vec::new();
    let graph = maze_to_graph(maze.clone());

    while queue.len() > 0 {
        let (path, current) = queue.pop_back().unwrap();
        if current == end {
            return path;
        }

        if visited.contains(&current) {
            continue;
        }

        visited.push(current.clone());
        for (direction, neighbour) in graph.get(&current).unwrap().iter() {
            queue.push_front((path.clone() + direction, neighbour.clone()));
        }
    }

    panic!("No path found");
}

fn maze_to_graph(maze: Vec<Vec<char>>) -> HashMap<String, Vec<(String, String)>> {
    let mut graph: HashMap<String, Vec<(String, String)>> = HashMap::new();
    let height = maze.len();
    let width = if height > 0 { maze[0].len() } else { 0 };

    for i in 0..height {
        for j in 0..width {
            let cell_address = i.to_string() + "," + &j.to_string();

            if !graph.contains_key(cell_address.as_str()) {
                graph.insert(cell_address.clone(), Vec::new());
            }
        }
    }

    for (i, row) in maze.iter().enumerate() {
        for (j, cell) in row.iter().enumerate() {
            let cell_value = replace_char(*cell) as usize;

            let cell_south = if i + 1 >= height { 0 as usize } else { replace_char(maze[i + 1][j]) as usize };
            let cell_east = if j + 1 >= width { 0 as usize } else { replace_char(maze[i][j + 1]) as usize };

            if [cell_value + 1, cell_value, cell_value - 1].contains(&cell_south) {
                let cell_address = i.to_string() + "," + &j.to_string();
                let cell_south_address = (i + 1).to_string() + "," + &j.to_string();

                graph.get_mut(&cell_address).unwrap().push(("v".to_string(), cell_south_address.clone()));
                graph.get_mut(&cell_south_address).unwrap().push(("^".to_string(), cell_address.clone()));
            }

            if [cell_value + 1, cell_value, cell_value - 1].contains(&cell_east) {
                let cell_address = i.to_string() + "," + &j.to_string();
                let cell_east_address = i.to_string() + "," + &(j + 1).to_string();

                graph.get_mut(&cell_address).unwrap().push((">".to_string(), cell_east_address.clone()));
                graph.get_mut(&cell_east_address).unwrap().push(("<".to_string(), cell_address.clone()));
            }
        }
    }

    return graph;
}

fn parse_input(file_path: &str) -> Vec<Vec<char>> {
    let mut maze: Vec<Vec<char>> = Vec::new();
    if let Ok(lines) = read_lines(file_path) {
        for line in lines {
            maze.push(line.unwrap().chars().collect());
        }
    }

    return maze;
}

fn replace_char(char: char) -> char {
    match char {
        'S' => 'a',
        'E' => 'z',
        _ => char
    }
}
