use std::fs;

fn part_one() {
    let input = fs::read_to_string("input.txt").expect("Should have been able to read the file").replace("\r", "");

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
    let input = fs::read_to_string("input.txt").expect("Should have been able to read the file").replace("\r", "");

    let mut max = [0, 0, 0];
    let mut sum = 0;
    for line in input.split('\n') {
        if !line.is_empty() {
            sum += line.parse::<i32>().unwrap();
        } else {
            if sum > max[0] {
                if sum > max[1] {
                    if sum > max[2] {
                        max[0] = max[1];
                        max[1] = max[2];
                        max[2] = sum;
                    } else {
                        max[0] = max[1];
                        max[1] = sum;
                    }
                } else {
                    max[0] = sum;
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
