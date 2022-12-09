use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub(crate) fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub(crate) fn create_matrices<T: Clone>(x: i32, y: i32, value: T) -> Vec<Vec<T>>{
    let mut matrix = Vec::new();
    for _ in 0..y {
        let mut row = Vec::new();
        for _ in 0..x {
            row.push(value.clone());
        }
        matrix.push(row);
    }
    return matrix;
}