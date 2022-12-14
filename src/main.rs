#![feature(is_some_and)]

// mod day_X;
mod day_1;
mod utils;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;
mod day_7;
mod day_8;
mod day_9;
mod day_10;
mod day_11;
mod day_12;
mod day_13;
mod day_14;
mod day_15;

fn main() {
    let now = std::time::Instant::now();
    let mut times = Vec::new();

    // day_1::day_1_1();
    // day_1::day_1_2();
    // day_2::day_2_1();
    // day_2::day_2_2();
    // day_3::day_3_1();
    // day_3::day_3_2();
    // day_4::day_4_1();
    // day_4::day_4_2();
    // day_5::day_5_1();
    // day_5::day_5_2();
    // day_6::day_6_1();
    // day_6::day_6_2();
    // day_7::day_7_1();
    // day_7::day_7_2();
    // day_8::day_8_1();
    // day_8::day_8_2();
    // day_9::day_9_1();
    // day_9::day_9_2();
    // day_10::day_10_1();
    // day_10::day_10_2();
    // day_11::day_11_1();
    // day_11::day_11_2(); // This day takes a while to run
    // day_12::day_12_1(); // This day takes a while to run
    // day_12::day_12_2(); // This day takes a while to run
    // day_13::day_13_1(); // This day takes a while to run
    // day_13::day_13_2(); // This day takes a while to run
    // day_14::day_14_1();
    // day_14::day_14_2();
    day_15::day_15_1(); // This day takes a while to run
    day_15::day_15_2(); // This day takes a while to run

    times.push(now.elapsed());

    println!("Took {:.2?} to be ran", times.first().unwrap());
}
