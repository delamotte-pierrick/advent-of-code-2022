use std::cmp::Ordering;
use std::cmp::Ordering::{Equal, Greater, Less};
use std::collections::HashSet;
use json::JsonValue;
use crate::utils::read_lines;

pub(crate) fn day_13_1() {
    if let Ok(lines) = read_lines("./data/input_13.txt") {
        let packets: Vec<String> = lines.map(|line| line.unwrap()).collect();
        let packets_group: Vec<Vec<String>> = packets.chunks(3).map(|x| Vec::from(x.clone())).collect();

        let mut counter: Vec<i32> = Vec::new();
        for (index, packet_group) in packets_group.into_iter().enumerate() {
            let packet_1 = get_elems(packet_group[0].clone());
            let packet_2 = get_elems(packet_group[1].clone());

            let result = compare(packet_1.clone(), packet_2.clone(), 0);
            if result == Less {
                counter.push(index as i32 + 1);
            }
        }
        println!("Number of element with correct order: {}", counter.iter().sum::<i32>());
    }
}

pub(crate) fn day_13_2() {
    if let Ok(lines) = read_lines("./data/input_13.txt") {
        let mut packets: Vec<String> = lines.map(|line| line.unwrap()).collect();
        packets.retain(|x| x != "");
        packets.push("[[2]]".to_string());
        packets.push("[[6]]".to_string());

        packets.sort_by(|a, b| {
            let a = get_elems(a.clone());
            let b = get_elems(b.clone());
            compare(a, b, 0)
        });

        let packet_1 = packets.iter().position(|x| x == "[[2]]").unwrap() + 1;
        let packet_2 = packets.iter().position(|x| x == "[[6]]").unwrap() + 1;

        println!("Number of element with correct order: {}", packet_1 * packet_2);
    }
}

fn get_elems(line: String) -> JsonValue {
    return json::parse(&*line).unwrap();
}

fn compare(mut elem_1: JsonValue, mut elem_2: JsonValue, depth: u32) -> Ordering {
    if elem_1.is_array() || elem_2.is_array() {
        if !elem_1.is_array() {
            elem_1 = json::array![elem_1];
        }
        if !elem_2.is_array() {
            elem_2 = json::array![elem_2];
        }

        for i in 0..elem_1.len() {
            //Case where elem_2 is shorter than elem_1
            if i >= elem_2.len() {
                return Greater;
            }

            let compared = compare(elem_1[i].clone(), elem_2[i].clone(), depth + 1);
            if [Greater, Less].contains(&compared) {
                return compared;
            }
        }

        //Case where all elements are equal but elem_1 is shorter than elem_2
        if elem_1.len() < elem_2.len() {
            return Less;
        }

        return Equal;
    }

    if elem_1.is_number() && elem_2.is_number() {
        let elem_1 = elem_1.as_i32();
        let elem_2 = elem_2.as_i32();
        return if elem_1 < elem_2 { Less } else if elem_1 == elem_2 { Equal } else { Greater };
    }

    panic!("Not implemented");
}
