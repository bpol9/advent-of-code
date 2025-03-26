use std::fs;
use std::process;
use std::error::Error;

fn main() {
    let mut input: Vec<Vec<char>> = Vec::new();
    let file_input = read_file("D:/advent-of-code/2024/04/four-a/input.txt").unwrap_or_else(|err| {
        println!("Error reading input: {err}");
        process::exit(1);
    });
    
    for line in file_input.lines() {
        input.push(line.chars().collect());
    }

    let mut sum: u32 = 0;
    for i in 0..input.len() {
        for j in 0..input[i].len() {
            if !(input[i][j] == 'X') {
                continue;
            }
            if check_x(&input, i, j, -1, 0) {
                sum = sum + 1;
            }
            if check_x(&input, i, j, -1, 1) {
                sum = sum + 1;
            }
            if check_x(&input, i, j, 0, 1) {
                sum = sum + 1;
            }
            if check_x(&input, i, j, 1, 1) {
                sum = sum + 1;
            }
            if check_x(&input, i, j, 1, 0) {
                sum = sum + 1;
            }
            if check_x(&input, i, j, 1, -1) {
                sum = sum + 1;
            }
            if check_x(&input, i, j, 0, -1) {
                sum = sum + 1;
            }
            if check_x(&input, i, j, -1, -1) {
                sum = sum + 1;
            }
        }
    }

    println!("{}", sum);
                 
}

fn read_file(file_path: &str) -> Result<String, Box<dyn Error>> {
    let contents = fs::read_to_string(file_path)?;
    Ok(contents)
}

fn check_x(input: &Vec<Vec<char>>, x: usize, y: usize, inc_x: i32, inc_y: i32) -> bool {
    let x_max: i32 = input.len() as i32 - 1;
    let y_max: i32 = input[0].len() as i32 - 1;
    let mut xx: i32 = x as i32;
    let mut yy: i32 = y as i32;

    let mut next_char: char = 'M';
    loop {
        xx = xx as i32 + inc_x;
        yy = yy as i32 + inc_y;
        if xx > x_max || xx < 0 || yy > y_max || yy < 0 {
            return false;
        }

        if !(input[xx as usize][yy as usize] == next_char) {
            return false;
        }

        if next_char == 'S' {
            return true;
        }

        next_char = get_next_char(next_char);
    }

}

fn get_next_char(curr: char) -> char {
    if curr == 'M' {
        return 'A';
    } else { // curr should be 'A'
        return 'S';
    }
}

