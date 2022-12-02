use crate::utils::read_lines;

pub(crate) fn day_2_1() {
    let mut my_score = 0;

    // Number of point you gonna win for a game
    // Ex: Rock (rules[0]) vs Rock (rules[x][0]) = draw = 3
    let rules = [
        [3, 6, 0], // Rock
        [0, 3, 6], // Paper
        [6, 0, 3], // Scissors
    ];
    // Value you hand will score
    let expected_score = [1, 2, 3];

    calculate_score(&rules, &expected_score, &mut my_score);
    println!("My score is {}", my_score);
}

pub(crate) fn day_2_2() {
    let mut my_score = 0;

    // Value of your hand to loose, draw or win a game
    // Ex: You want draw (rules[x][0]) with Rock (rules[0]) = 1 (Rock)
    let rules = [
        [3, 1, 2], // Rock
        [1, 2, 3], // Paper
        [2, 3, 1], // Scissors
    ];
    // Expected score if you want to loose, draw or win a game
    let expected_score = [0, 3, 6];

    calculate_score(&rules, &expected_score, &mut my_score);
    println!("My score is {}", my_score);
}

fn calculate_score(rules: &[[i32; 3]; 3], expected_score: &[i32; 3], my_score: &mut i32) {
    if let Ok(lines) = read_lines("./data/input_2.txt") {
        for line in lines {
            if let Ok(values) = line {
                let play = values.split(" ").map(|x| x.chars().collect::<Vec<char>>()[0]).collect::<Vec<char>>();
                let player_1 = play[0] as usize - 65;
                let player_2 = play[1] as usize - 88;
                let hand_score = rules[player_1][player_2];

                *my_score += hand_score + expected_score[player_2];
            }
        }
    }
}
