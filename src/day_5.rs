use crate::utils::read_lines;

pub(crate) fn day_5_1() {
    let mut result: Vec<char> = Vec::new();
    let tower_length = 8;

    if let Ok(lines) = read_lines("./data/input_5.txt") {
        let inputs = lines.map(|instruction| instruction.unwrap()).collect::<Vec<String>>();
        let mut towers: Vec<Vec<char>> = parse_towers(inputs.clone(), tower_length);

        let re = regex::Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
        for instruction in &inputs[10..] {
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
    let tower_length = 8;

    if let Ok(lines) = read_lines("./data/input_5.txt") {
        let inputs = lines.map(|instruction| instruction.unwrap()).collect::<Vec<String>>();
        let mut towers: Vec<Vec<char>> = parse_towers(inputs.clone(), tower_length);

        let re = regex::Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
        for instruction in &inputs[10..] {
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

fn parse_towers(inputs: Vec<String>, tower_length: usize) -> Vec<Vec<char>> {
    let mut towers: Vec<Vec<char>> = Vec::new();
    let re = regex::Regex::new(r"(\[([A-Z])]|\s{3})\s?").unwrap();

    //Init towers
    re.captures_iter(&inputs[0]).for_each(|_| {
        let tower: Vec<char> = Vec::new();
        towers.push(tower);
    });

    //Add boxes to towers
    for input in &inputs[..=tower_length] {
        re.captures_iter(input).enumerate().for_each(|(index, cap)| {
            if &cap[1] != "   " {
                towers[index].push(cap[2].chars().next().unwrap());
            }
        });
    }

    towers.iter_mut().for_each(|tower| tower.reverse());

    return towers;
}
