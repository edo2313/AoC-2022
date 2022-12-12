use std::fs;

fn part_one() {
    let input = fs::read_to_string("input.txt")
        .expect("Should have been able to read the file")
        .replace("\r", "");

    let mut points = 0;
    for line in input.split('\n') {
        if line.is_empty() {
            continue;
        }
        let mut line_chars = line.chars();
        let player1 = line_chars.next().unwrap();
        line_chars.next(); //Skip empty char
        let player2 = line_chars.next().unwrap();
        let diff = player2 as i32 - player1 as i32 - 23;
        points += player2 as i32 - 87;
        if diff == 0 {
            points += 3;
        } else if diff == 1 || diff == -2 {
            points += 6;
        }
    }
    println!("Punti totali: {}", points);
}

fn part_two() {
    let input = fs::read_to_string("input.txt")
        .expect("Should have been able to read the file")
        .replace("\r", "");

    let mut points = 0;
    for line in input.split('\n') {
        if line.is_empty() {
            continue;
        }
        let mut line_chars = line.chars();
        let player1 = line_chars.next().unwrap();
        line_chars.next(); //Skip empty char
        let outcome = line_chars.next().unwrap();
        let mut player2 = 0;
        if outcome == 'X' {
            //Lose
            match player1 {
                'A' => player2 = 'Z' as i32,
                'B' => player2 = 'X' as i32,
                'C' => player2 = 'Y' as i32,
                _ => (),
            }
        } else if outcome == 'Y' {
            //Draw
            player2 = player1 as i32 + 23;
            points += 3;
        } else {
            //Win
            match player1 {
                'A' => player2 = 'Y' as i32,
                'B' => player2 = 'Z' as i32,
                'C' => player2 = 'X' as i32,
                _ => (),
            }
            points += 6;
        }
        println!(
            "P1: {} - OUTCOME: {} - P2: {}",
            player1,
            outcome,
            format!("{:?}", player2 as u8 as char)
        );
        points += player2 - 87;
    }
    println!("Punti totali: {}", points);
}

fn main() {
    part_one();
    part_two();
}
