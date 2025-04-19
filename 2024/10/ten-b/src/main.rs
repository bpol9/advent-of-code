use std::fs;
use std::process;
use std::error::Error;

#[derive(Eq, PartialEq, Debug, Hash)]
struct Point {
    i: usize,
    j: usize
}

fn main() {
    let file_input = read_file("D:/advent-of-code/2024/10/ten-a/input.txt").unwrap_or_else(|err| {
        println!("Error reading input: {err}");
        process::exit(1);
    });

    let mut map: Vec<Vec<u8>> = Vec::new();
    let mut zero_pos: Vec<Point> = Vec::new();
    let mut i: usize = 0;
    let mut j: usize;
    for line in file_input.lines() {
        let mut row = vec![0; line.len()];
        j = 0;
        for c in line.chars() {
            if c == '0' {
                zero_pos.push(Point {i, j});
            }
            row[j] = c.to_digit(10).unwrap() as u8;
            j += 1;
        }
        map.push(row);
        i += 1;
    }
     
    let mut reachable_nines: Vec<Point> = Vec::new();
    let mut sum: usize = 0;
    for p in zero_pos {
        find_reachable_nines(p, &map, &mut reachable_nines);
        sum += reachable_nines.len();
        reachable_nines.clear();
    }

    println!("{}", sum);
}

fn get_next_moves(curr: Point, map: &Vec<Vec<u8>>) -> Vec<Point> {
    let max_j = map[0].len() - 1;
    let max_i = map.len() - 1;
    let mut next_points: Vec<Point> = Vec::new();
    let next_height: u8 = map[curr.i][curr.j] + 1;
    if curr.j > 0 && map[curr.i][curr.j - 1] == next_height {
        next_points.push(Point {i: curr.i, j: curr.j - 1});
    }
    if curr.j < max_j && map[curr.i][curr.j + 1] == next_height {
        next_points.push(Point {i: curr.i, j: curr.j + 1});
    }
    if curr.i > 0 && map[curr.i - 1][curr.j] == next_height {
        next_points.push(Point {i: curr.i - 1, j: curr.j});
    }
    if curr.i < max_i && map[curr.i + 1][curr.j] == next_height {
        next_points.push(Point {i: curr.i + 1, j: curr.j});
    }

    next_points
}

fn find_reachable_nines(start: Point, map: &Vec<Vec<u8>>, nines: &mut Vec<Point>) {
    let next_points = get_next_moves(start, map);
    for next in next_points {
        if map[next.i][next.j] == 9 {
            nines.push(next);
        } else {
            find_reachable_nines(next, map, nines);
        }
    }
}

#[test]
fn test_reachable_nines() {
    let map: Vec<Vec<u8>> = vec![
        vec![1,0,0,0,9,0,0],
        vec![2,0,0,0,8,0,0],
        vec![3,0,0,0,7,0,0],
        vec![4,5,6,7,6,5,4],
        vec![0,0,0,8,0,0,3],
        vec![0,0,0,9,0,0,2],
        vec![0,0,0,0,0,0,1]
    ];
    let mut start: Point = Point {i: 0, j: 1};
    let mut nines: Vec<Point> = Vec::new();
    find_reachable_nines(start, &map, &mut nines);
    assert_eq!(nines.len(), 1);
    assert!(nines.contains(&Point{i: 5, j: 3}));
    start = Point {i: 6, j: 5};
    nines.clear();
    find_reachable_nines(start, &map, &mut nines);
    assert_eq!(nines.len(), 2);
    assert!(nines.contains(&Point{i: 5, j: 3}));
    assert!(nines.contains(&Point{i: 0, j: 4}));
}

fn read_file(file_path: &str) -> Result<String, Box<dyn Error>> {
    let contents = fs::read_to_string(file_path)?;
    Ok(contents)
}
