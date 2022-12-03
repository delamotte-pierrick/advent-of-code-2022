use crate::utils::read_lines;
use array_tool::vec::Intersect;

pub(crate) fn day_3_1() {
    let mut priority_sum = 0;

    if let Ok(lines) = read_lines("./data/input_3.txt") {
        for line in lines {
            if let Ok(rucksacks) = line {
                let compartments = rucksacks.chars().collect::<Vec<char>>()
                    .chunks(rucksacks.len() / 2)
                    .map(|x| Vec::from(x))
                    .collect::<Vec<Vec<char>>>();
                let duplicate_package = compartments[0].intersect(compartments[1].clone());
                for package in duplicate_package {
                    let package_value = package as usize;
                    priority_sum += if package_value > 96 { package_value - 96 } else { package_value - 64 + 26 };
                }
            }
        }
    }
    println!("Priority sum: {}", priority_sum);
}

pub(crate) fn day_3_2() {
    let mut priority_sum = 0;

    if let Ok(lines) = read_lines("./data/input_3.txt") {
        let elks = lines.map(|x| x.unwrap()).collect::<Vec<String>>();
        let elk_groups = elks.chunks(3).collect::<Vec<&[String]>>();

        for elk_group in &elk_groups {
            let rucksacks = elk_group.iter().map(|x| x.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
            let tag = rucksacks[0].intersect(rucksacks[1].clone()).intersect(rucksacks[2].clone());
            for package in tag {
                let package_value = package as usize;
                priority_sum += if package_value > 96 { package_value - 96 } else { package_value - 64 + 26 };
            }
        }
    }

    println!("Priority sum: {}", priority_sum);
}
