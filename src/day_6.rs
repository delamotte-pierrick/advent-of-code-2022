use crate::utils::read_lines;

pub(crate) fn day_6_1() {
    println!("Start of packet detected at: {}", search_uniq_string(4));
}

pub(crate) fn day_6_2() {
    println!("Start of message detected at: {}", search_uniq_string(14));
}

fn search_uniq_string(packet_length: usize) -> usize {
    let mut start_of_packet = 0;
    if let Ok(lines) = read_lines("./data/input_6.txt") {
        let lines = lines.map(|x| x.unwrap()).collect::<Vec<String>>();
        let line = lines.first().unwrap();

        for index in 0..line.len() - packet_length {
            let slice = &line[index..index + packet_length].chars().collect::<Vec<char>>();
            let mut filtered = slice.clone();
            filtered.sort();
            filtered.dedup();

            if filtered.len() == slice.len() {
                start_of_packet = index + packet_length;
                break;
            }
        }
    }

    return start_of_packet;
}
