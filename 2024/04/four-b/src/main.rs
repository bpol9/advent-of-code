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
            if !(input[i][j] == 'A') {
                continue;
            }

            if check_first_diagonal(&input, i, j) && check_second_diagonal(&input, i, j) {
                sum = sum + 1;
            }
        }
    }

    println!("{}", sum);
}

fn check_first_diagonal(input: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
    let x_max = input.len() as i32 - 1;
    let y_max = input[0].len() as i32 - 1;
    let xx = x as i32;
    let yy = y as i32;
    if xx - 1 < 0 || xx + 1 > x_max || yy - 1 < 0 || yy + 1 > y_max {
        return false;
    }

    return (input[(xx-1) as usize][(yy-1) as usize] == 'M' && input[(xx+1) as usize][(yy+1) as usize] == 'S')
        || (input[(xx-1) as usize][(yy-1) as usize] == 'S' && input[(xx+1) as usize][(yy+1) as usize] == 'M');
}

fn check_second_diagonal(input: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
    let x_max = input.len() as i32 - 1;
    let y_max = input[0].len() as i32 - 1;
    let xx = x as i32;
    let yy = y as i32;
    if xx - 1 < 0 || xx + 1 > x_max || yy - 1 < 0 || yy + 1 > y_max {
        return false;
    }

    return (input[(xx-1) as usize][(yy+1) as usize] == 'M' && input[(xx+1) as usize][(yy-1) as usize] == 'S')
        || (input[(xx-1) as usize][(yy+1) as usize] == 'S' && input[(xx+1) as usize][(yy-1) as usize] == 'M');
}

fn read_file(file_path: &str) -> Result<String, Box<dyn Error>> {
    let contents = fs::read_to_string(file_path)?;
    Ok(contents)
}
