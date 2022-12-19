use std::fs;

fn part_one() {
    let input = fs::read_to_string("input.txt")
        .expect("Should have been able to read the file")
        .replace("\r", "");
    let mut input_iterator = input.split('\n');

    let mut strength_sum = 0;
    let mut index = 1;
    let mut x = 1;
    let mut adding = false;
    let mut temp = 0;

    loop {
        if adding {
            x += temp;
            adding = false;
        } else {
            if let Some(line) = input_iterator.next() {
                if line.starts_with("addx ") {
                    temp = line.replace("addx ", "").parse::<i32>().unwrap();
                    adding = true;
                }
            } else {
                break;
            }
        }

        index += 1;

        if (index - 20) % 40 == 0 {
            strength_sum += x * index;
        }
    }

    println!("Part 1: {}", strength_sum);
}

fn part_two() {
    let input = fs::read_to_string("input.txt")
        .expect("Should have been able to read the file")
        .replace("\r", "");
    let mut input_iterator = input.split('\n');

    let mut index = 0;
    let mut x = 1;
    let mut adding = false;
    let mut temp = 0;

    loop {
        if adding {
            x += temp;
            adding = false;
        } else {
            if let Some(line) = input_iterator.next() {
                if line.starts_with("addx ") {
                    temp = line.replace("addx ", "").parse::<i32>().unwrap();
                    adding = true;
                }
            } else {
                break;
            }
        }

        index += 1;

        if index % 40 == x - 1 || index % 40 == x || index % 40 == x + 1 {
            print!("#");
        } else {
            print!(".");
        }
        if index % 40 == 0 {
            print!("\n");
        }
    }
}

fn main() {
    part_one();
    part_two();
}
