use std::collections::LinkedList;
use std::fs;

fn part_one() {
    let input = fs::read_to_string("input.txt")
        .expect("Should have been able to read the file")
        .replace("\r", "");
    let mut input_iterator = input.split('\n');

    let mut top_stack = [' '; 9];

    let mut stacks = vec![LinkedList::<char>::new(); 9];

    // Stacks parsing
    while let Some(line) = input_iterator.next() {
        if line.is_empty() {
            break;
        }
        let mut i = 1;
        let line_length = line.len();
        while i < line_length {
            let _char = line.chars().nth(i).unwrap();
            if _char != ' ' && _char.is_alphabetic() {
                stacks[(i - 1) / 4].push_front(_char);
            }
            i += 4;
        }
    }

    // Moves
    while let Some(line) = input_iterator.next() {
        if line.is_empty() {
            break;
        }

        let move_string: Vec<i32> = line
            .split(" ")
            .map(|x| x.parse::<i32>().or::<i32>(Ok(-1)).unwrap())
            .collect();
        (0..move_string[1]).for_each(|_| {
            let moved = stacks[move_string[3] as usize - 1].pop_back().unwrap();
            stacks[move_string[5] as usize - 1].push_back(moved);
        });
    }

    for i in 0..9 {
        top_stack[i] = stacks[i].pop_back().unwrap();
    }
    println!("Part 1: {}", top_stack.iter().collect::<String>());
}

fn part_two() {
    let input = fs::read_to_string("input.txt")
        .expect("Should have been able to read the file")
        .replace("\r", "");
    let mut input_iterator = input.split('\n');

    let mut top_stack = [' '; 9];

    let mut stacks = vec![LinkedList::<char>::new(); 9];

    // Stacks parsing
    while let Some(line) = input_iterator.next() {
        if line.is_empty() {
            break;
        }
        let mut i = 1;
        let line_length = line.len();
        while i < line_length {
            let _char = line.chars().nth(i).unwrap();
            if _char != ' ' && _char.is_alphabetic() {
                stacks[(i - 1) / 4].push_front(_char);
            }
            i += 4;
        }
    }

    // Moves
    while let Some(line) = input_iterator.next() {
        if line.is_empty() {
            break;
        }

        let move_string: Vec<i32> = line
            .split(" ")
            .map(|x| x.parse::<i32>().or::<i32>(Ok(-1)).unwrap())
            .collect();
        let mut moved = LinkedList::<char>::new();
        (0..move_string[1]).for_each(|_| {
            moved.push_front(stacks[move_string[3] as usize - 1].pop_back().unwrap());
        });
        for ele in moved {
            stacks[move_string[5] as usize - 1].push_back(ele);
        }
    }

    for i in 0..9 {
        top_stack[i] = stacks[i].pop_back().unwrap();
    }
    println!("Part 1: {}", top_stack.iter().collect::<String>());
}

fn main() {
    part_one();
    part_two();
}
