use crate::utils::read_lines;

pub(crate) fn day_10_1() {
    if let Ok(mut lines) = read_lines("./data/input_10.txt") {
        let mut program_run = true;
        let mut current_cycle = 1;
        let mut current_value = 1;
        let mut current_instruction_cycles = 0;
        let mut instruction_value = 0;
        let mut signal_strength = 0;
        while program_run {
            if current_instruction_cycles == 0 {
                current_value += instruction_value;

                if let Some(instruction) = lines.next() {
                    let instruction = instruction.unwrap();
                    if instruction.starts_with("addx") {
                        instruction_value = instruction[5..].parse::<i32>().unwrap();
                        current_instruction_cycles = 2;
                    } else if instruction.starts_with("noop") {
                        instruction_value = 0;
                        current_instruction_cycles = 1;
                    }
                } else {
                    program_run = false;
                }
            }

            //Spying on the current instruction
            if [20, 60, 100, 140, 180, 220].contains(&current_cycle) {
                signal_strength += current_value * current_cycle;
            }

            current_instruction_cycles -= 1;
            current_cycle += 1;
        }
        println!("Signal strength value is: {}", signal_strength);
    }
}

pub(crate) fn day_10_2() {
    println!("The crt print :");

    if let Ok(mut lines) = read_lines("./data/input_10.txt") {
        let mut program_run = true;
        let mut current_cycle = 1;
        let mut current_value = 1;
        let mut current_instruction_cycles = 0;
        let mut instruction_value = 0;
        let mut crt: String = "".to_string();
        let mut offset = 0;
        while program_run {
            if current_instruction_cycles == 0 {
                current_value += instruction_value;

                if let Some(instruction) = lines.next() {
                    let instruction = instruction.unwrap();
                    if instruction.starts_with("addx") {
                        instruction_value = instruction[5..].parse::<i32>().unwrap();
                        current_instruction_cycles = 2;
                    } else if instruction.starts_with("noop") {
                        instruction_value = 0;
                        current_instruction_cycles = 1;
                    }
                } else {
                    program_run = false;
                }
            }

            //Compute CRT
            let current_sprite = (current_value + offset..current_value + offset + 3).collect::<Vec<i32>>();
            if current_sprite.contains(&current_cycle) {
                crt += "#";
            } else {
                crt += ".";
            }

            if current_cycle % 40 == 0 {
                println!("{}", crt.replace(".", " ").replace("#", "█"));
                offset = current_cycle;
                crt = "".to_string();
            }

            current_instruction_cycles -= 1;
            current_cycle += 1;
        }
    }
}
