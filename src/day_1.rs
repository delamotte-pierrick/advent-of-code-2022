use crate::utils::read_lines;

pub(crate) fn day_1_1() {
    let mut most_calories = 0;

    if let Ok(lines) = read_lines("./data/input_1.txt") {
        let mut current_calories = 0;
        for line in lines {
            if let Ok(value) = line {
                if value.is_empty() {
                    if current_calories > most_calories {
                        most_calories = current_calories;
                    }

                    current_calories = 0;
                    continue;
                }
                current_calories += value.parse::<i32>().unwrap();
            }
        }
    }
    println!("Most calories: {}", most_calories);
}

pub(crate) fn day_1_2() {
    let mut calories_list = Vec::new();

    if let Ok(lines) = read_lines("./data/input_1.txt") {
        let mut current_calories = 0;
        for line in lines {
            if let Ok(value) = line {
                if value.is_empty() {
                    calories_list.push(current_calories);
                    current_calories = 0;
                    continue;
                }
                current_calories += value.parse::<i32>().unwrap();
            }
        }
    }

    calories_list.sort_by(|a, b| b.cmp(a));

    let most_calories = &calories_list[0..3].iter().sum::<i32>();
    println!("Most calories: {}", most_calories);
}