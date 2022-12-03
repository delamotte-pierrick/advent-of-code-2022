use crate::utils::read_lines;
use array_tool::vec::Intersect;

pub(crate) fn day_3_1() {
    let mut priority_sum = 0;

    if let Ok(lines) = read_lines("./data/input_3.txt") {
        for line in lines {
            if let Ok(rucksacks) = line {
                let packages = rucksacks.chars().collect::<Vec<char>>();

                let first_compartment = packages[0..packages.len() / 2].to_vec();
                let second_compartment = packages[packages.len() / 2..packages.len()].to_vec();

                let duplicate_package = first_compartment.intersect(second_compartment);
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
            let first_rucksack = elk_group[0].chars().collect::<Vec<char>>();
            let second_rucksack = elk_group[1].chars().collect::<Vec<char>>();
            let third_rucksack = elk_group[2].chars().collect::<Vec<char>>();

            let tag = first_rucksack.intersect(second_rucksack).intersect(third_rucksack);
            for package in tag {
                let package_value = package as usize;
                priority_sum += if package_value > 96 { package_value - 96 } else { package_value - 64 + 26 };
            }
        }
    }

    println!("Priority sum: {}", priority_sum);
}
