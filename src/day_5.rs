use crate::utils::read_lines;

pub(crate) fn day_5_1() {
    let mut result: Vec<char> = Vec::new();

    if let Ok(lines) = read_lines("./data/input_5.txt") {
        //Manual parsing of the input
        let mut towers: Vec<Vec<char>> = Vec::new();
        towers.push(Vec::from(['N', 'V', 'C', 'S', ]));
        towers.push(Vec::from(['S', 'N', 'H', 'J', 'M', 'Z', ]));
        towers.push(Vec::from(['D', 'N', 'J', 'G', 'T', 'C', 'M', ]));
        towers.push(Vec::from(['M', 'R', 'W', 'J', 'F', 'D', 'T', ]));
        towers.push(Vec::from(['H', 'F', 'P', ]));
        towers.push(Vec::from(['J', 'H', 'Z', 'T', 'C', ]));
        towers.push(Vec::from(['Z', 'L', 'S', 'F', 'Q', 'R', 'P', 'D', ]));
        towers.push(Vec::from(['W', 'P', 'F', 'D', 'H', 'L', 'S', 'C', ]));
        towers.push(Vec::from(['Z', 'G', 'N', 'F', 'P', 'M', 'S', 'D', ]));

        // Now reverse because we want to start from the bottom
        for tower in towers.iter_mut() {
            tower.reverse();
        }

        let re = regex::Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
        let instructions = lines.map(|instruction| instruction.unwrap()).collect::<Vec<String>>();
        for instruction in &instructions[10..] {
            re.captures_iter(instruction).for_each(|cap| {
                let quantity = cap[1].parse::<usize>().unwrap();
                let from = cap[2].parse::<usize>().unwrap() - 1;
                let to = cap[3].parse::<usize>().unwrap() - 1;

                let from_tower = &mut towers[from];
                let mut craned_boxes = from_tower.split_off(from_tower.len() - quantity);
                craned_boxes.reverse();

                towers[to].append(&mut craned_boxes);
            });
        }

        result = towers.iter().map(|tower: &Vec<char>| tower.last().unwrap().clone()).collect();
    }

    println!("Crate on top of towers: {}", result.iter().collect::<String>());
}

pub(crate) fn day_5_2() {
    let mut result: Vec<char> = Vec::new();

    if let Ok(lines) = read_lines("./data/input_5.txt") {
        //Manual parsing of the input
        let mut towers: Vec<Vec<char>> = Vec::new();
        towers.push(Vec::from(['N', 'V', 'C', 'S', ]));
        towers.push(Vec::from(['S', 'N', 'H', 'J', 'M', 'Z', ]));
        towers.push(Vec::from(['D', 'N', 'J', 'G', 'T', 'C', 'M', ]));
        towers.push(Vec::from(['M', 'R', 'W', 'J', 'F', 'D', 'T', ]));
        towers.push(Vec::from(['H', 'F', 'P', ]));
        towers.push(Vec::from(['J', 'H', 'Z', 'T', 'C', ]));
        towers.push(Vec::from(['Z', 'L', 'S', 'F', 'Q', 'R', 'P', 'D', ]));
        towers.push(Vec::from(['W', 'P', 'F', 'D', 'H', 'L', 'S', 'C', ]));
        towers.push(Vec::from(['Z', 'G', 'N', 'F', 'P', 'M', 'S', 'D', ]));

        // Now reverse because we want to start from the bottom (And because i'm to lazy)
        for tower in towers.iter_mut() {
            tower.reverse();
        }

        let re = regex::Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
        let instructions = lines.map(|instruction| instruction.unwrap()).collect::<Vec<String>>();
        for instruction in &instructions[10..] {
            re.captures_iter(instruction).for_each(|cap| {
                let quantity = cap[1].parse::<usize>().unwrap();
                let from = cap[2].parse::<usize>().unwrap() - 1; //Remove 1 because the input is 1-indexed
                let to = cap[3].parse::<usize>().unwrap() - 1; //Remove 1 because the input is 1-indexed

                let from_tower = &mut towers[from];
                let mut craned_boxes = from_tower.split_off(from_tower.len() - quantity);

                towers[to].append(&mut craned_boxes);
            });
        }

        result = towers.iter().map(|tower: &Vec<char>| tower.last().unwrap().clone()).collect();
    }

    println!("Crate on top of towers with simultaneous move: {}", result.iter().collect::<String>());
}
