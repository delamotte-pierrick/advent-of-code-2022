use std::collections::{HashMap, VecDeque};
use std::string::String;
use crate::utils::read_lines;

pub(crate) fn day_12_1() {
    let (maze, start, end) = parse_input("./data/input_12.txt", 'S', 'E');

    if start.is_none() || end.len() == 0 {
        panic!("No start or end found");
    }

    let path = pathfinder_bfs(maze, start.unwrap(), end, false);

    println!("Shortest path is: {} cell long", path.len());
}

pub(crate) fn day_12_2() {
    let (maze, start, end) = parse_input("./data/input_12.txt", 'E', 'a');

    if start.is_none() || end.len() == 0 {
        panic!("No start or end found");
    }

    let path = pathfinder_bfs(maze, start.unwrap(), end, true);

    // visualize_maze(maze.clone(), path.clone(), Option::from(start.clone().unwrap()), end.clone());

    println!("Shortest path to lowest elevation is: {} cell long", path.len());
}

fn pathfinder_bfs(maze: Vec<Vec<char>>, start: String, end: Vec<String>, is_from_end: bool) -> Vec<(String, String)> {
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

fn maze_to_graph(maze: Vec<Vec<char>>, is_from_end: bool) -> HashMap<String, Vec<(String, String)>> {
    let mut graph: HashMap<String, Vec<(String, String)>> = HashMap::new();
    (0..maze.len()).map(|i| i).for_each(|i| {
        (0..maze[0].len()).for_each(|j| {
            let i = i as i32;
            let j = j as i32;
            let cell_address = format!("{},{}", i, j);

            if !graph.contains_key(cell_address.as_str()) {
                graph.insert(cell_address.clone(), Vec::new());
            }

            let cell_value = maze[i as usize][j as usize] as usize;

            [(i + 1, j, "↑"), (i - 1, j, "↓"), (i, j + 1, "←"), (i, j - 1, "→")].into_iter().for_each(|(x, y, direction)| {
                if x >= 0 && x < maze.len() as i32 && y >= 0 && y < maze[0].len() as i32 {
                    let neighbour_value = maze[x as usize][y as usize] as usize;

                    let mut can_go_node = cell_value + 1 == neighbour_value || cell_value >= neighbour_value;
                    if is_from_end {
                        can_go_node = cell_value - 1 == neighbour_value || cell_value <= neighbour_value;
                    }

                    if can_go_node {
                        graph.get_mut(cell_address.as_str()).unwrap().push((direction.to_string(), format!("{},{}", x, y)));
                    }
                }
            });
        });
    });

    return graph;
}

fn parse_input(file_path: &str, start_char: char, end_char: char) -> (Vec<Vec<char>>, Option<String>, Vec<String>) {
    let mut maze: Vec<Vec<char>> = Vec::new();
    let mut start: Option<String> = None;
    let mut end: Vec<String> = Vec::new();

    if let Ok(lines) = read_lines(file_path) {
        lines.for_each(|line| {
            maze.push(line.unwrap().chars().collect());
        });
    }

    (0..maze.len()).for_each(|i| {
        (0..maze[i].len()).for_each(|j| {
            if maze[i][j] == start_char {
                start = Some(format!("{},{}", i, j));
                maze[i][j] = replace_char(maze[i][j]);
            } else if maze[i][j] == end_char {
                end.push(format!("{},{}", i, j));
                maze[i][j] = replace_char(maze[i][j]);
            }
        });
    });

    return (maze, start, end);
}

fn replace_char(char: char) -> char {
    match char {
        'S' => 'a',
        'E' => 'z',
        _ => char
    }
}

fn visualize_maze(maze: Vec<Vec<char>>, path: Vec<(String, String)>, start: Option<String>, ends: Vec<String>) {
    let mut maze: Vec<Vec<String>> = (0..maze.len())
        .into_iter()
        .map(|i| (0..maze[i].len())
            .into_iter()
            .map(|_| " ".to_string())
            .collect::<Vec<String>>())
        .collect();

    //Write start and end
    if let Some(start) = start {
        let start: Vec<usize> = start.split(",").map(|x| x.parse::<usize>().unwrap()).collect();
        maze[start[0]][start[1]] = "S".to_string();
    }
    for end in ends {
        let end: Vec<usize> = end.split(",").map(|x| x.parse::<usize>().unwrap()).collect();
        maze[end[0]][end[1]] = "E".to_string();
    }

    for (direction, cell) in path {
        let coords: Vec<usize> = cell.split(",").map(|x| x.parse::<usize>().unwrap()).collect();
        maze[coords[0]][coords[1]] = direction.chars().next().unwrap().to_string();
    }

    for row in maze {
        println!("{:?}", row.join(""));
    }
}
