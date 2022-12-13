use std::fs;

fn part_one() {
    let input = fs::read_to_string("input.txt")
        .expect("Should have been able to read the file")
        .replace("\r", "");

    let mut last_chars = [' '; 4];
    let mut packet_start = 0;

    for _char in input.chars() {
        if packet_start > 4 && check_unique(&last_chars) {
            break;
        }
        last_chars[packet_start % 4] = _char;
        packet_start += 1;
    }

    println!("Part 1: {}", packet_start);
}

fn part_two() {
    let input = fs::read_to_string("input.txt")
        .expect("Should have been able to read the file")
        .replace("\r", "");

    let mut last_chars = [' '; 14];
    let mut packet_start = 0;

    for _char in input.chars() {
        if packet_start > 14 && check_unique(&last_chars) {
            break;
        }
        last_chars[packet_start % 14] = _char;
        packet_start += 1;
    }

    println!("Part 1: {}", packet_start);
}

fn main() {
    part_one();
    part_two();
}

fn check_unique(arr: &[char]) -> bool {
    for i in 0..arr.len() {
        for j in i+1..arr.len() {
            if arr[i] == arr[j] {
                return false;
            }
        }
    }
    return true;
}
