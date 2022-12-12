use std::fs;

fn part_one() {
    let input = fs::read_to_string("input.txt")
        .expect("Should have been able to read the file")
        .replace("\r", "");

    let mut priorities_sum = 0;
    for line in input.split('\n') {
        if line.is_empty() {
            continue;
        }

        let (first_part, second_part) = line.split_at(line.len() / 2);
        let common_char = find_common(first_part, second_part);
        if common_char.is_some() {
            priorities_sum += char_to_priority(common_char.unwrap());
        }
    }

    fn find_common(first_string: &str, second_string: &str) -> Option<char> {
        for letter1 in first_string.chars() {
            for letter2 in second_string.chars() {
                if letter1 == letter2 {
                    return Some(letter1);
                }
            }
        }
        return None;
    }

    println!("Part 1: {}", priorities_sum);
}

fn part_two() {
    let binding = fs::read_to_string("input.txt")
        .expect("Should have been able to read the file")
        .replace("\r", "");
    let input: Vec<&str> = binding.split('\n').collect();
    
    let mut priorities_sum = 0;

    for group in input.chunks(3){
        if group.len() < 3 {
            continue;
        }
        let common_char = find_common(group[0], group[1], group[2]);
        if common_char.is_some() {
            priorities_sum += char_to_priority(common_char.unwrap());
        }
    }
    
    println!("Part 2: {}", priorities_sum);

    fn find_common(first_string: &str, second_string: &str, third_string: &str) -> Option<char> {
        for letter1 in first_string.chars() {
            for letter2 in second_string.chars() {
                for letter3 in third_string.chars() {
                    if letter1 == letter2 && letter2 == letter3 {
                        return Some(letter1);
                    }
                }
            }
        }
        return None;
    }
}

fn main() {
    part_one();
    part_two();
}

fn char_to_priority(_char: char) -> i32 {
    let mut priority = _char as i32 - 96;
    if priority < 0 {
        priority = priority + 58;
    }
    return priority;
}