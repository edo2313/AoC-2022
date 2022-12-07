use std::fs;

fn part_one() {
    let input = fs::read_to_string("input.txt")
        .expect("Should have been able to read the file")
        .replace("\r", "");

    let mut points = 0;

    for line in input.split('\n') {
        
    }
}

fn part_two() {
    let input = fs::read_to_string("input.txt")
        .expect("Should have been able to read the file")
        .replace("\r", "");
}

fn main() {
    part_one();
    part_two();
}
