use std::collections::{HashMap, VecDeque};
use std::string::String;
use crate::utils::read_lines;

pub(crate) fn day_12_1() {
    let maze = parse_input("./data/input_12.txt");
    let mut start: Option<String> = None;
    let mut end: Option<Vec<String>> = None;

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
                    end = Option::from(Vec::from([i.to_string() + "," + &j.to_string()]));
                }
            }
        }
    }
    if start.is_none() || end.is_none() {
        panic!("No start or end found");
    }

    let path = pathfinder_bfs(maze.clone(), start.clone().unwrap(), end.clone().unwrap(), false);

    // visualize_maze(maze.clone(), path.clone());

    println!("Shortest path is: {} cell long", path.len());
}

pub(crate) fn day_12_2() {
    let maze = parse_input("./data/input_12.txt");
    let mut start: Option<String> = None;
    let mut end: Vec<String> = Vec::new();

    for (i, row) in maze.iter().enumerate() {
        if row.contains(&'E') {
            for (j, cell) in row.iter().enumerate() {
                if *cell == 'E' {
                    start = Option::from(i.to_string() + "," + &j.to_string());
                }
            }
        }

        if row.contains(&'a') || row.contains(&'S') {
            for (j, cell) in row.iter().enumerate() {
                if ['S', 'a'].contains(cell) {
                    end.push(i.to_string() + "," + &j.to_string());
                }
            }
        }
    }
    if start.is_none() || end.len() == 0 {
        panic!("No start or end found");
    }

    let path = pathfinder_bfs(maze.clone(), start.clone().unwrap(), end.clone(), true);

    // visualize_maze(maze.clone(), path.clone());

    println!("Shortest path to lowest elevation is: {} cell long", path.len());
}

fn pathfinder_bfs(maze: Vec<Vec<char>>, start: String, end: Vec<String>, is_from_end : bool) -> Vec<(String, String)> {
    let mut queue: VecDeque<(Vec<(String, String)>, String)> = VecDeque::from([(Vec::new(), start.clone())]);
    let mut visited: Vec<String> = Vec::new();
    let graph = maze_to_graph(maze.clone(), is_from_end);

    while queue.len() > 0 {
        let (path, current) = queue.pop_back().unwrap();
        if end.contains(&current) {
            return path;
        }

        if visited.contains(&current) {
            continue;
        }

        visited.push(current.clone());
        for (direction, neighbour) in graph.get(&current).unwrap().iter() {
            let mut new_path = path.clone();
            new_path.push((direction.clone(), current.clone()));

            queue.push_front((new_path, neighbour.clone()));
        }
    }

    panic!("No path found");
}

fn maze_to_graph(maze: Vec<Vec<char>>, is_from_end : bool) -> HashMap<String, Vec<(String, String)>> {
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

            let i = i as i32;
            let j = j as i32;

            [(i + 1, j), (i - 1, j), (i, j + 1), (i, j - 1)].iter().for_each(|(x, y)| {
                if *x >= 0 && *x < height as i32 && *y >= 0 && *y < width as i32 {
                    let neighbour_value = replace_char(maze[*x as usize][*y as usize]) as usize;
                    let cell_address = i.to_string() + "," + &j.to_string();
                    let neighbour_address = x.to_string() + "," + &y.to_string();

                    let from_end = cell_value - 1 == neighbour_value || cell_value <= neighbour_value;
                    let from_start = cell_value + 1 == neighbour_value || cell_value >= neighbour_value;

                    if (is_from_end && from_end) || (!is_from_end && from_start) {
                        let direction = match (i as i32 - *x as i32, j as i32 - *y as i32) {
                            (1, 0) => "↑",
                            (-1, 0) => "↓",
                            (0, 1) => "←",
                            (0, -1) => "→",
                            _ => panic!("Invalid direction")
                        };
                        graph.get_mut(cell_address.as_str()).unwrap().push((direction.parse().unwrap(), neighbour_address));
                    }
                }
            });
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

fn visualize_maze(maze: Vec<Vec<char>>, path: Vec<(String, String)>) {
    let mut maze = maze.clone().into_iter()
        .map(|row| row.into_iter().map(|cell| if ['S', 'E'].contains(&cell) { cell } else { ' ' }).collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    for (direction, cell) in path {
        let coords: Vec<&str> = cell.split(",").collect();
        let x = coords[0].parse::<usize>().unwrap();
        let y = coords[1].parse::<usize>().unwrap();
        maze[x][y] = direction.chars().next().unwrap();
    }

    for row in maze {
        println!("{:?}", row);
    }
}
