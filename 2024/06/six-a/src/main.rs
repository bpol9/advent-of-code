use std::fs;
use std::process;
use std::error::Error;
use std::collections::HashMap;

enum Direction {
    North,
    East,
    South,
    West,
}

fn main() {
    let mut input: Vec<Vec<char>> = Vec::new();
    let file_input = read_file("D:/advent-of-code/2024/06/six-a/input.txt").unwrap_or_else(|err| {
        println!("Error reading input: {err}");
        process::exit(1);
    });

    let mut i: usize = 0;
    let mut start_i: usize = 0;
    let mut start_j: usize = 0;
    let mut start_found = false;
    for line in file_input.lines() {
        let chars: Vec<char> = line.chars().collect();
        if !start_found {
            for j in 0..chars.len() {
                if chars[j] == '^' {
                    (start_i, start_j) = (i,j);
                    start_found = true;
                }
            }
        }
        input.push(chars);
        i = i + 1;
    }

    if !start_found {
        println!("Start position not found in input");
        process::exit(1);
    }

    let visited_points = get_visited_points(&input, start_i, start_j);
    println!("{}", visited_points.len());
}

fn is_out_of_bounds(i: i32, j: i32, max_i: i32, max_j: i32) -> bool {
    return i < 0 || i > max_i || j < 0 || j > max_j;
}

fn is_obstacle(point: char) -> bool {
    return point == '#';
}

fn get_visited_points(map: &Vec<Vec<char>>, i: usize, j: usize) -> HashMap<(usize, usize), bool> {
    let mut inc_i: i32;
    let mut inc_j: i32;
    let max_i: i32 = map.len() as i32 - 1;
    let max_j: i32 = map[0].len() as i32 - 1;
    let mut ii: i32 = i as i32;
    let mut jj: i32 = j as i32;
    let mut visited_points: HashMap<(usize, usize), bool> = HashMap::new();
    visited_points.insert((i,j), true);
    let mut dir: Direction = Direction::North;
    (inc_i, inc_j) = get_inc_i_j(&dir);

    loop {
        ii += inc_i;
        jj += inc_j;

        if is_out_of_bounds(ii, jj, max_i, max_j) {
            return visited_points;
        }

        if is_obstacle(map[ii as usize][jj as usize]) {
            ii -= inc_i;
            jj -= inc_j;
            dir = get_next_direction(&dir);
            (inc_i, inc_j) = get_inc_i_j(&dir);
        } else {
            visited_points.insert((ii as usize, jj as usize), true);
        }
    }
}

fn get_inc_i_j(direction: &Direction) -> (i32, i32) {
    match direction {
        Direction::North => (-1, 0),
        Direction::West => (0, -1),
        Direction::South => (1, 0),
        Direction::East => (0, 1)
    }
}

fn get_next_direction(dir: &Direction) -> Direction {
    match dir {
        Direction::North => Direction::East,
        Direction::West => Direction::North,
        Direction::South => Direction::West,
        Direction::East => Direction::South
    }
}


fn read_file(file_path: &str) -> Result<String, Box<dyn Error>> {
    let contents = fs::read_to_string(file_path)?;
    Ok(contents)
}
