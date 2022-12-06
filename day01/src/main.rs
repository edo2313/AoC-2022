use std::fs;

fn part_one() {
    let input = fs::read_to_string("input.txt").expect("Should have been able to read the file");

    let mut max = 0;
    let mut sum = 0;
    for line in input.split('\n') {
        if !line.is_empty() {
            sum += line.parse::<i32>().unwrap();
        } else {
            if sum > max {
                max = sum;
            }
            sum = 0;
        }
    }
    println!("Maximum is {}", max);
}

fn part_two() {
    let input = fs::read_to_string("input.txt").expect("Should have been able to read the file");

    let mut max = [0, 0, 0];
    let mut sum = 0;
    for line in input.split('\n') {
        if !line.is_empty() {
            sum += line.parse::<i32>().unwrap();
        } else {
            for i in 0..3 {
                if sum > max[i] {
                    max[i] = sum;
                    break;
                }
            }
            sum = 0;
        }
    }
    println!("3 maximums are {}, {} and {}", max[0], max[1], max[2]);
    println!("Their sum is {}", max[0]+max[1]+max[2])
}

fn main() {
    part_one();
    part_two();
}
