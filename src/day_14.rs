use crate::utils::read_lines;

pub(crate) fn day_14_1() {
    let sand_start_point = (500, 0);
    let grid = generate_grid("./data/input_14.txt", sand_start_point, false);
    let (_result_grid, time) = simulate_cave(grid.clone(), false);

    println!("Number of sand grain who fell: {}", time);
}

pub(crate) fn day_14_2() {
    let sand_start_point = (500, 0);
    let grid = generate_grid("./data/input_14.txt", sand_start_point, true);
    let (result_grid, mut time) = simulate_cave(grid.clone(), true);

    //Now that the grid is filled, we can count the number of tile who are filled but out of the grid
    let left_edge: Vec<char> = result_grid.iter().map(|elem| elem[0]).collect();
    let right_edge: Vec<char> = result_grid.iter().map(|elem| elem[elem.len() - 1]).collect();

    let results: Vec<usize> = [left_edge, right_edge].into_iter().map(|row| {
        //use factorial
        let outside_blocks = row.into_iter().filter(|x| x == &'O').collect::<Vec<char>>().len() - 1;
        return (1..=outside_blocks).into_iter().sum();
    }).collect();

    println!("Number of sand grain who fell: {}", time + results.iter().sum::<usize>() as i32);
}

fn generate_grid(filename: &str, sand_start_point: (usize, usize), part_2: bool) -> Vec<Vec<char>> {
    let mut coordinates: Vec<Vec<(usize, usize)>> = Vec::new();
    if let Ok(lines) = read_lines(filename) {
        lines.for_each(|line| {
            let mut coordinate: Vec<(usize, usize)> = Vec::new();
            line.unwrap().split(" -> ").for_each(|c| {
                coordinate.push((c.split(",").nth(0).unwrap().parse().unwrap(), c.split(",").nth(1).unwrap().parse().unwrap()));
            });
            coordinates.push(coordinate);
        });

        let mut xoffset = 0;
        let mut yoffset = 0;
        if part_2 {
            xoffset = 1;
            yoffset = 1;
        }

        let xmin = coordinates.iter().map(|c| c.iter().map(|c| c.0).min().unwrap()).min().unwrap();
        let xmax = coordinates.iter().map(|c| c.iter().map(|c| c.0).max().unwrap()).max().unwrap();
        let ymin = 0;
        let ymax = coordinates.iter().map(|c| c.iter().map(|c| c.1).max().unwrap()).max().unwrap();

        let mut grid: Vec<Vec<char>> = vec![vec!['.'; xmax - xmin + 1 + xoffset * 2]; ymax - ymin + 1 + yoffset];

        coordinates.iter().for_each(|coordinate_group| {
            (0..coordinate_group.len() - 1).for_each(|c| {
                let current = coordinate_group[c];
                let next = coordinate_group[c + 1];
                let mut x1 = current.0 - xmin + xoffset;
                let mut y1 = current.1 - ymin;
                let mut x2 = next.0 - xmin + xoffset;
                let mut y2 = next.1 - ymin;

                if y1 > y2 {
                    let temp = y1;
                    y1 = y2;
                    y2 = temp;
                }
                if x1 > x2 {
                    let temp = x1;
                    x1 = x2;
                    x2 = temp;
                }

                (y1..=y2).for_each(|y| {
                    (x1..=x2).for_each(|x| {
                        grid[y][x] = '█';
                    });
                });
            });
        });

        //Now add the grain filler
        grid[sand_start_point.1 - ymin][sand_start_point.0 - xmin + xoffset] = '+';

        return grid;
    }

    panic!("Could not read file");
}

fn simulate_cave(mut grid: Vec<Vec<char>>, part_2: bool) -> (Vec<Vec<char>>, i32) {
    let mut time = 0;

    let start_point = grid[0].iter().position(|&c| c == '+').unwrap();

    loop {
        //the first wall from ground
        let data = move_sand_grain(&grid, start_point, 0, part_2);
        if data.is_none() {
            break;
        }

        let (x, y) = data.unwrap();
        grid[y][x] = 'O';

        time += 1;

        if grid[0][start_point] == 'O' {
            break;
        }
    }

    return (grid, time);
}

fn move_sand_grain(grid: &Vec<Vec<char>>, x: usize, y: usize, part_2: bool) -> Option<(usize, usize)> {
    if y + 1 == grid.len() {
        return if part_2 { Some((x, y)) } else { None };
    }

    if grid[y + 1][x] == '.' {
        return move_sand_grain(grid, x, y + 1, part_2);
    } else if ['█', 'O'].contains(&grid[y + 1][x]) {
        if x == 0 {
            if grid[y + 1][x + 1] == '.' {
                return move_sand_grain(grid, x + 1, y + 1, part_2); //Fix a bug when the sand grain is on the left edge
            }

            return if part_2 { Some((x, y)) } else { None };
        }

        if grid[y + 1][x - 1] == '.' {
            return move_sand_grain(grid, x - 1, y + 1, part_2);
        }

        if x + 1 >= grid[0].len() {
            return if part_2 { Some((x, y)) } else { None };
        }

        if grid[y + 1][x + 1] == '.' {
            return move_sand_grain(grid, x + 1, y + 1, part_2);
        }

        return Some((x, y));
    }

    panic!("Could not move sand grain");
}
