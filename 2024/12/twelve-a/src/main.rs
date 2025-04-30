use std::fs;
use std::process;
use std::error::Error;
use std::collections::HashSet;

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
struct Point {
    i: u32,
    j: u32
}

fn main() {
    let file_input = read_file("D:/advent-of-code/2024/12/twelve-a/input.txt").unwrap_or_else(|err| {
        println!("Error reading input: {err}");
        process::exit(1);
    });

    let mut map: Vec<Vec<char>> = Vec::new();
    for line in file_input.lines() {
        map.push(line.trim().chars().collect());
    }

    let mut regions: Vec<HashSet<Point>> = Vec::new();

    for i in 0..map.len() {
        for j in 0..map[0].len() {
            let point = Point {i: i as u32, j: j as u32};
            if !is_in_region(point, &regions) {
                let mut region: HashSet<Point> = HashSet::new();
                region.insert(point);
                get_region(point, &map, &mut region);
                regions.push(region);
            }
        }
    }

    let mut sum: usize = 0;
    for r in regions {
        sum += r.len() * get_perimeter_of_region(&r, &map);
    }

    println!("{}", sum);
}

fn is_in_region(point: Point, regions: &Vec<HashSet<Point>>) -> bool {
    for r in regions {
        if r.contains(&point) {
            return true;
        }
    }
    return false;
}

fn get_region(start: Point, map: &Vec<Vec<char>>, visited: &mut HashSet<Point>) {
    for n in get_neighbors(&start, map) {
        if !visited.contains(&n) && map[n.i as usize][n.j as usize] == map[start.i as usize][start.j as usize] {
            visited.insert(n);
            get_region(n, map, visited);
        }
    }
}

fn get_perimeter_of_region(region: &HashSet<Point>, map: &Vec<Vec<char>>) -> usize {
    let mut perimeter: usize = 0;
    for point in region {
        let n = get_neighbors(point, map);
        perimeter += 4 - n.len();
        for p in n {
            if map[p.i as usize][p.j as usize] != map[point.i as usize][point.j as usize] {
                perimeter += 1;
            }
        }
    }

    perimeter
}

fn get_neighbors(p: &Point, map: &Vec<Vec<char>>) -> Vec<Point> {
    let initial = vec![
        (p.i as i32 - 1, p.j as i32),
        (p.i as i32 + 1, p.j as i32),
        (p.i as i32, p.j as i32 - 1),
        (p.i as i32, p.j as i32 + 1)
    ];

    initial.into_iter().filter(|c| is_in_range(c, map)).map(|c| Point {i: c.0 as u32, j: c.1 as u32}).collect()
}

fn is_in_range(p: &(i32, i32), map: &Vec<Vec<char>>) -> bool {
    return p.0 >= 0 && p.0 < map.len() as i32 && p.1 >= 0 && p.1 < map[0].len() as i32;
}

#[test]
fn test_get_region() {
    let map = vec![
        vec!['A', 'A', 'A', 'A'],
        vec!['B', 'B', 'C', 'D'],
        vec!['B', 'B', 'C', 'C'],
        vec!['E', 'E', 'E', 'C']
    ];

    let mut region: HashSet<Point> = HashSet::new();
    let start: Point = Point {i: 0, j: 0};
    region.insert(start);
    get_region(start, &map, &mut region);   
    assert_eq!(4, region.len());
    assert(region.contains(&Point{i: 0, j: 0}));
    assert(region.contains(&Point{i: 0, j: 1}));
    assert(region.contains(&Point{i: 0, j: 2}));
    assert(region.contains(&Point{i: 0, j: 3}));
}

fn read_file(file_path: &str) -> Result<String, Box<dyn Error>> {
    let contents = fs::read_to_string(file_path)?;
    Ok(contents)
}
