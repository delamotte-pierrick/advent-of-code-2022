use range_ext::intersect::Intersect;
use range_ext::intersect::Intersection::{Above, Bellow};
use crate::utils::read_lines;

pub(crate) fn day_4_1() {
    let mut contained = 0;
    if let Ok(lines) = read_lines("./data/input_4.txt") {
        let elf_groups = lines.map(|x| x.unwrap()).collect::<Vec<String>>();

        for elf_group in elf_groups {
            let elf_group_split = elf_group.split(",")
                .map(|y: &str| y.split("-").map(|w| w.parse::<i32>().unwrap()).collect::<Vec<i32>>()
                ).collect::<Vec<Vec<i32>>>();

            if elf_group_split[0][0] <= elf_group_split[1][0] && elf_group_split[0][1] >= elf_group_split[1][1] ||
                elf_group_split[0][0] >= elf_group_split[1][0] && elf_group_split[0][1] <= elf_group_split[1][1] {
                contained += 1;
            }
        }
    }
    println!("Number of fully overlapping assignment: {}", contained);
}

pub(crate) fn day_4_2() {
    let mut contained = 0;
    if let Ok(lines) = read_lines("./data/input_4.txt") {
        let elf_groups = lines.map(|x| x.unwrap()).collect::<Vec<String>>();

        for elf_group in elf_groups {
            let elf_group_split = elf_group.split(",")
                .map(|y: &str| y.split("-").map(|w| w.parse::<i32>().unwrap()).collect::<Vec<i32>>()
                ).collect::<Vec<Vec<i32>>>();

            let range_1 = elf_group_split[0][0] - 1..elf_group_split[0][1];
            let range_2 = elf_group_split[1][0] - 1..elf_group_split[1][1];

            let inter = range_1.intersect(&range_2);

            if ![Bellow, Above].contains(&inter) {
                contained += 1;
            }

            println!("{:?}", inter);
        }
    }
    println!("Number of overlapping assignment: {}", contained);
}
