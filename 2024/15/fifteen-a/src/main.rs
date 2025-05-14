use std::fs;
use std::process;
use std::error::Error;

#[derive(Copy, Clone, Eq, PartialEq)]
enum Direction {
    South,
    West,
    North,
    East
}

fn main() {
    let file_input = read_file("D:/advent-of-code/2024/15/fifteen-a/input.txt").unwrap_or_else(|err| {
        println!("Error reading input: {err}");
        process::exit(1);
    });

    let parts: Vec<&str> = file_input.split("\n\n").collect();
    let mut map: Vec<Vec<char>> = Vec::new();
    let mut robot_pos: Option<(usize, usize)> = None;
    let mut i = 0;
    for line in parts[0].lines() {
        let l = line.chars().collect::<Vec<char>>();
        if let Some(j) = l.iter().position(|&c| c == '@') {
            robot_pos = Some((i,j));
        }
        map.push(l);
        i += 1;
    }

    if robot_pos.is_none() {
        println!("Robot initial position not found!");
        process::exit(1);
    }

    let (mut robot_i, mut robot_j) = robot_pos.unwrap();

    let movements = parts[1].replace("\n", "");
    for dir in movements.chars().map(|c| get_dir_from_char(c).unwrap()) {
        (robot_i, robot_j) = movement(&mut map, robot_i, robot_j, dir);
        //print_map(&map);
    }

    let mut sum: u64 = 0;
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            if map[i][j] == 'O' {
                sum += get_gps_coordinate(i,j);
            }
        }
    }

    println!("{}", sum);

}

fn read_file(file_path: &str) -> Result<String, Box<dyn Error>> {
    let contents = fs::read_to_string(file_path)?;
    Ok(contents)
}

fn print_map(map: &Vec<Vec<char>>) {
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            print!("{}", map[i][j]);
        }
        println!("");
    }
}

fn movement(map: &mut Vec<Vec<char>>, i: usize, j: usize, dir: Direction) -> (usize, usize) {
    let next = get_next(map, i, j, dir);
    if next.is_none() {
        return (i,j);
    }

    let (robot_next_i, robot_next_j) = next.unwrap();
    let mut c = map[robot_next_i][robot_next_j];
    if c == '#' {
        return (i,j);
    }
    if c == '.' {
        map[i][j] = '.';
        map[robot_next_i][robot_next_j] = '@';
        return (robot_next_i, robot_next_j);
    }

    let (mut next_i, mut next_j) = (robot_next_i, robot_next_j);
    loop {
       (next_i, next_j) = get_next(map, next_i, next_j, dir).unwrap();
       c = map[next_i][next_j];
       if c == '.' {
           map[next_i][next_j] = 'O';
           map[robot_next_i][robot_next_j] = '@';
           map[i][j] = '.';
           return (robot_next_i, robot_next_j);
       } else if c == '#' {
           return (i,j);
       }
    }
}

fn get_next(map: &Vec<Vec<char>>, i: usize, j: usize, dir: Direction) -> Option<(usize, usize)> {
    if dir == Direction::West {
        if j == 0 {
            return None;
        } else {
            return Some((i, j-1));
        }
    } else if dir == Direction::East {
        if j == map[0].len() - 1 {
            return None;
        } else {
            return Some((i,j+1));
        }
    } else if dir == Direction::North {
        if i == 0 {
            return None;
        } else {
            return Some((i-1,j));
        }
    } else {
        if i == map.len() - 1 {
            return None;
        } else {
            return Some((i+1,j));
        }
    }
}

fn get_dir_from_char(c: char) -> Option<Direction> {
    return match c {
        '>' => Some(Direction::East),
        'v' => Some(Direction::South),
        '<' => Some(Direction::West),
        '^' => Some(Direction::North),
        _   => None
    };
}

fn get_gps_coordinate(i: usize, j: usize) -> u64 {
    return (i as u64)*100 + j as u64;
}
