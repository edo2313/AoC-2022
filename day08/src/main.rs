use std::fs;

fn part_one() {
    let input = fs::read_to_string("input.txt")
        .expect("Should have been able to read the file")
        .replace("\r", "");

    let mut visible_trees = 0;
    let rows = input.split('\n').count() - 1;
    let cols = input.split('\n').next().unwrap().len();
    let mut matrix = vec![vec![0; cols]; rows];
    for (y, line) in input.split('\n').enumerate() {
        if line.is_empty() {
            continue;
        }
        for (x, tree) in line.chars().map(|x| x as u32 - '0' as u32).enumerate() {
            matrix[y][x] = tree;
        }
    }
    for y in 0..rows {
        for x in 0..cols {
            if x == 0 || y == 0 {
                visible_trees += 1;
                continue;
            }

            let tree = matrix[y][x];

            let mut visible_up = true;
            for i in 0..y {
                if matrix[i][x] >= tree {
                    visible_up = false;
                    break;
                }
            }
            let mut visible_down = true;
            for i in (y + 1)..rows {
                if matrix[i][x] >= tree {
                    visible_down = false;
                    break;
                }
            }
            let mut visible_left = true;
            for i in 0..x {
                if matrix[y][i] >= tree {
                    visible_left = false;
                    break;
                }
            }
            let mut visible_right = true;
            for i in (x + 1)..cols {
                if matrix[y][i] >= tree {
                    visible_right = false;
                    break;
                }
            }
            visible_trees += (visible_up || visible_down || visible_left || visible_right) as i32;
        }
    }

    println!("Part 1: {}", visible_trees);
}

fn part_two() {
    let input = fs::read_to_string("input.txt")
        .expect("Should have been able to read the file")
        .replace("\r", "");

    let mut scenic_score = 0;
    let rows = input.split('\n').count() - 1;
    let cols = input.split('\n').next().unwrap().len();
    let mut matrix = vec![vec![0; cols]; rows];
    for (y, line) in input.split('\n').enumerate() {
        if line.is_empty() {
            continue;
        }
        for (x, tree) in line.chars().map(|x| x as u32 - '0' as u32).enumerate() {
            matrix[y][x] = tree;
        }
    }
    for y in 1..rows-1 {
        for x in 1..cols-1 {
            let tree = matrix[y][x];

            let mut visible_up = 0;
            for i in 0..y {
                visible_up += 1;
                if matrix[y-(i+1)][x] >= tree {
                    break;
                }
            }
            let mut visible_down = 0;
            for i in (y + 1)..rows {
                visible_down += 1;
                if matrix[i][x] >= tree {
                    break;
                }
            }
            let mut visible_left = 0;
            for i in 0..x {
                visible_left += 1;
                if matrix[y][x-(i+1)] >= tree {
                    break;
                }
            }
            let mut visible_right = 0;
            for i in (x + 1)..cols {
                visible_right += 1;
                if matrix[y][i] >= tree {
                    break;
                }
            }
            let temp_score = visible_up * visible_down * visible_left * visible_right;
            // println!(
            //     "x {} y {} val {} vis_u {} vis_d {} vis_l {} vis_r {} score {}",
            //     x, y, matrix[y][x], visible_up, visible_down, visible_left, visible_right, temp_score
            // );
            if temp_score > scenic_score {
                scenic_score = temp_score;
            }
        }
    }

    println!("Part 2: {}", scenic_score);
}

fn main() {
    part_one();
    part_two();
}
