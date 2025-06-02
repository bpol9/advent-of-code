use std::fs;
use std::process;
use std::error::Error;
use std::collections::HashMap;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Direction {
    South,
    West,
    North,
    East
}

#[derive(Clone, Copy, Eq, PartialEq)]
struct Move {
    p: Position,
    c: char
}

#[derive(Hash, Debug, Clone, Copy, Eq, PartialEq)]
struct Position {
    i: usize,
    j: usize
}

impl Position {
    fn new(i: usize, j: usize) -> Self {
        Position {i, j}
    }

    fn one_up(&self) -> Self {
        return Position {i: self.i - 1, j: self.j};
    }

    fn one_down(&self) -> Self {
        return Position {i: self.i + 1, j: self.j};
    }

    fn one_left(&self) -> Self {
        return Position {i: self.i, j: self.j - 1};
    }

    fn one_right(&self) -> Self {
        return Position {i: self.i, j: self.j + 1};
    }
}

impl Move {
    fn new(p: Position, c: char) -> Self {
        Move {p, c}
    }
}

fn main() {
    let file_input = read_file("D:/advent-of-code/2024/15/fifteen-a/input.txt").unwrap_or_else(|err| {
        println!("Error reading input: {err}");
        process::exit(1);
    });

    let parts: Vec<&str> = file_input.split("\n\n").collect();
    let mut map: Vec<Vec<char>> = Vec::new();
    let mut robot_pos_opt: Option<Position> = None;
    let mut i = 0;
    for line in parts[0].lines() {
        let l = extend_line(line.chars().collect::<Vec<char>>());
        if let Some(j) = l.iter().position(|&c| c == '@') {
            robot_pos_opt = Some(Position::new(i,j));
        }
        map.push(l);
        i += 1;
    }

    if robot_pos_opt.is_none() {
        println!("Robot initial position not found!");
        process::exit(1);
    }

    let mut robot_pos = robot_pos_opt.unwrap();

    let movements = parts[1].replace("\n", "");
    for dir in movements.chars().map(|c| get_dir_from_char(c).unwrap()) {
        match dir {
            Direction::North => robot_pos = move_up(&mut map, robot_pos),
            Direction::South => robot_pos = move_down(&mut map, robot_pos),
            Direction::East  => robot_pos = move_right(&mut map, robot_pos),
            Direction::West  => robot_pos = move_left(&mut map, robot_pos),
        }
    }

    let mut sum: u64 = 0;
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            if map[i][j] == '[' {
                sum += get_gps_coordinate(i,j);
            }
        }
    }

    println!("{}", sum);
}

fn extend_line(l: Vec<char>) -> Vec<char> {
    let mut result = Vec::new();
    for c in l {
        match c {
            '#' => result.extend(['#', '#']),
            'O' => result.extend(['[', ']']),
            '.' => result.extend(['.', '.']),
            '@' => result.extend(['@', '.']),
             _  => println!("Invalid input char {}", c),
        }
    }
    result
}

fn add_move(moves: &mut HashMap<Position, char>, pos: Position, c: char) {
    if c == '.' {
        moves.entry(pos).or_insert(c);
    } else {
        moves.insert(pos, c);
    }
}

fn move_up(map: &mut Vec<Vec<char>>, pos: Position) -> Position {
    let mut moves: HashMap<Position, char> = HashMap::new();
    moves.insert(pos, '.');
    let mut to_be_examined: Vec<Position> = vec![pos];
    while to_be_examined.len() > 0 {
        let p = to_be_examined.pop().unwrap();
        let next = get_next_up(map, p);
        if next.is_none() { // hit an obstacle, can't move up
            return pos;
        }
        to_be_examined.append(&mut next.unwrap());
        add_move(&mut moves, p.one_up(), get_char(map, p));
        add_move(&mut moves, p, '.');
    }

    for (pos, c) in moves {
        make_move(map, Move::new(pos, c));
    }

    return pos.one_up();
}

fn move_down(map: &mut Vec<Vec<char>>, pos: Position) -> Position {
    let mut moves: HashMap<Position, char> = HashMap::new();
    moves.insert(pos, '.');
    let mut to_be_examined: Vec<Position> = vec![pos];
    while to_be_examined.len() > 0 {
        let p = to_be_examined.pop().unwrap();
        let next = get_next_down(map, p);
        if next.is_none() {
            return pos;
        }
        to_be_examined.append(&mut next.unwrap());
        add_move(&mut moves, p.one_down(), get_char(map, p));
        add_move(&mut moves, p, '.');
    }

    for (pos, c) in moves {
        make_move(map, Move::new(pos, c));
    }

    return pos.one_down();
}

fn move_left(map: &mut Vec<Vec<char>>, robot_pos: Position) -> Position {
    if get_char(map, robot_pos) != '@' {
        return robot_pos;
    }

    let mut p = robot_pos.one_left();
    loop {
        let c = get_char(map, p);
        if c == '.' {
            let mut left_bracket = true;
            for j in (p.j)..(robot_pos.j-1) {
                if left_bracket {
                    map[p.i][j] = '[';
                    left_bracket = false;
                } else {
                    map[p.i][j] = ']';
                    left_bracket = true;
                }
            }
            map[robot_pos.i][robot_pos.j-1] = '@';
            map[robot_pos.i][robot_pos.j] = '.';
            return robot_pos.one_left();
        } else if c == '#' {
            return robot_pos;
        } else {
            p = p.one_left();
        }
    }
}

fn move_right(map: &mut Vec<Vec<char>>, robot_pos: Position) -> Position {
    if get_char(map, robot_pos) != '@' {
        return robot_pos;
    }

    let mut p = robot_pos.one_right();
    loop {
        let c = get_char(map, p);
        if c == '.' {
            let mut left_bracket = true;
            for j in (robot_pos.j+2)..=(p.j) {
                if left_bracket {
                    map[p.i][j] = '[';
                    left_bracket = false;
                } else {
                    map[p.i][j] = ']';
                    left_bracket = true;
                }
            }
            map[robot_pos.i][robot_pos.j+1] = '@';
            map[robot_pos.i][robot_pos.j] = '.';
            return robot_pos.one_right();
        } else if c == '#' {
            return robot_pos;
        } else {
            p = p.one_right();
        }
    }
}

fn make_move(map: &mut Vec<Vec<char>>, m: Move) {
    map[m.p.i][m.p.j] = m.c;
}

fn get_char(map: &Vec<Vec<char>>, p: Position) -> char {
    return map[p.i][p.j];
}

fn print_map(map: &Vec<Vec<char>>) {
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            print!("{}", map[i][j]);
        }
        println!("");
    }
}

fn get_next_up(map: &Vec<Vec<char>>, pos: Position) -> Option<Vec<Position>> {
    let i = pos.i;
    let j = pos.j;
    if i == 0 {
        return None;
    }

    let c = map[i-1][j];
    if c == ']' {
        return Some(vec![Position::new(i-1,j-1), Position::new(i-1,j)]);
    } else if c == '[' {
        return Some(vec![Position::new(i-1,j), Position::new(i-1,j+1)]);
    } else if c == '#' {
        return None;
    } else { // c == '.'
        return Some(vec![]);
    }
}

fn get_next_down(map: &Vec<Vec<char>>, pos: Position) -> Option<Vec<Position>> {
    let i = pos.i;
    let j = pos.j;
    if i == map.len()-1 {
        return None;
    }

    let c = map[i+1][j];
    if c == ']' {
        return Some(vec![Position::new(i+1,j-1), Position::new(i+1,j)]);
    } else if c == '[' {
        return Some(vec![Position::new(i+1,j), Position::new(i+1,j+1)]);
    } else if c == '#' {
        return None;
    } else {
        return Some(vec![]);
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

fn read_file(file_path: &str) -> Result<String, Box<dyn Error>> {
    let contents = fs::read_to_string(file_path)?;
    Ok(contents)
}
