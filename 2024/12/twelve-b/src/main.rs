use std::fs;
use std::process;
use std::error::Error;
use std::collections::HashSet;

#[derive(Eq, PartialEq,Copy, Clone,Hash)]
enum Orientation {
    Horizontal,
    Vertical
}

#[derive(Eq, PartialEq,Copy, Clone,Hash)]
enum Direction {
    North,
    East,
    South,
    West
}

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
struct Point {
    i: usize,
    j: usize
}

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
struct GridPoint {
    i: usize,
    j: usize
}

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
struct Edge {
    orientation: Orientation,
    direction: Direction, // points to the foreign region
    start: GridPoint,
    end: GridPoint
}

impl GridPoint {
    fn new(i: usize, j: usize) -> Self {
        GridPoint{i,j}
    }
}

impl std::fmt::Display for Edge {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({},{}) -> ({},{})", self.start.i, self.start.j, self.end.i, self.end.j)
    }
}

impl Edge {
    fn new(start: GridPoint, end: GridPoint, direction: Direction) -> Result<Self, String> {
        if start.i == end.i && start.j < end.j {
            let orientation = Orientation::Horizontal;
            Ok(Self {orientation, direction, start, end})
        } else if start.i < end.i && start.j == end.j {
            let orientation = Orientation::Vertical;
            Ok(Self {orientation, direction, start, end})
        } else {
            Err("Edge is not vertical or horizontal".to_string())
        }

    }
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
            let point = Point {i, j};
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
        let mut minor_edges = get_minor_edges_of_region(&r, &map);
        let merged_edges = merge_minor_edges_of_region(&mut minor_edges, &map);
        sum += r.len() * merged_edges.len();
    }

    println!("{}", sum);
}

fn get_outer_edges_of_point(point: &Point, map: &Vec<Vec<char>>) -> Vec<Edge> {
    let mut result: Vec<Edge> = Vec::new();
    let my_char = map[point.i][point.j];
    let north_char = get_char(point.i as i32 - 1, point.j as i32, map);
    let east_char = get_char(point.i as i32, point.j as i32 + 1, map);
    let south_char = get_char(point.i as i32 + 1, point.j as i32, map);
    let west_char = get_char(point.i as i32, point.j as i32 - 1, map);
    let north_edge = Edge::new(GridPoint::new(point.i, point.j), GridPoint::new(point.i, point.j+1), Direction::North).unwrap();
    let east_edge = Edge::new(GridPoint::new(point.i, point.j+1), GridPoint::new(point.i+1, point.j+1), Direction::East).unwrap();
    let south_edge = Edge::new(GridPoint::new(point.i+1, point.j), GridPoint::new(point.i+1, point.j+1), Direction::South).unwrap();
    let west_edge = Edge::new(GridPoint::new(point.i, point.j), GridPoint::new(point.i+1, point.j), Direction::West).unwrap();
    if let Some(n) = north_char {
        if n != my_char {
            result.push(north_edge);
        }
    } else {
        result.push(north_edge);
    }
    if let Some(e) = east_char {
        if e != my_char {
            result.push(east_edge);
        }
    } else {
        result.push(east_edge);
    }
    if let Some(s) = south_char {
        if s != my_char {
            result.push(south_edge);
        }
    } else {
        result.push(south_edge);
    }
    if let Some(w) = west_char {
        if w != my_char {
            result.push(west_edge);
        }
    } else {
        result.push(west_edge);
    }

    result
}

fn get_char(i: i32, j: i32, map: &Vec<Vec<char>>) -> Option<char> {
    if is_in_range(&(i,j), map) {
        Some(map[i as usize][j as usize])
    } else {
        None
    }
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
        if !visited.contains(&n) && map[n.i][n.j] == map[start.i][start.j] {
            visited.insert(n);
            get_region(n, map, visited);
        }
    }
}

fn get_neighbors(p: &Point, map: &Vec<Vec<char>>) -> Vec<Point> {
    let initial = vec![
        (p.i as i32 - 1, p.j as i32),
        (p.i as i32 + 1, p.j as i32),
        (p.i as i32, p.j as i32 - 1),
        (p.i as i32, p.j as i32 + 1)
    ];

    initial.into_iter().filter(|c| is_in_range(c, map)).map(|c| Point {i: c.0 as usize, j: c.1 as usize}).collect()
}

fn is_in_range(p: &(i32, i32), map: &Vec<Vec<char>>) -> bool {
    return p.0 >= 0 && p.0 < map.len() as i32 && p.1 >= 0 && p.1 < map[0].len() as i32;
}


fn read_file(file_path: &str) -> Result<String, Box<dyn Error>> {
    let contents = fs::read_to_string(file_path)?;
    Ok(contents)
}

fn extend_edge_east(e: &mut Edge, edge_set: &mut HashSet<Edge>, max_j: usize) {
    //println!("Extending east for {}", e);

    let (i,mut j) = (e.end.i, e.end.j);
    let direction = e.direction;
    while j < max_j {
        let next_edge = Edge {
            orientation : Orientation::Horizontal,
            start: GridPoint::new(i,j),
            end: GridPoint::new(i,j+1),
            direction: direction
        };
        if edge_set.contains(&next_edge) {
            e.end.j += 1;
            edge_set.remove(&next_edge);
            j += 1;
        }
        else {
            break;
        }
    }

    //println!("Result: {}", e);
}

fn extend_edge_west(e: &mut Edge, edge_set: &mut HashSet<Edge>) {
    //println!("Extending west for {}", e);
    let (i,mut j) = (e.start.i, e.start.j);
    let direction = e.direction;
    while j > 0 {
        let next_edge = Edge {
            orientation : Orientation::Horizontal,
			start: GridPoint::new(i,j-1),
			end: GridPoint::new(i,j),
            direction: direction
        };
        if edge_set.contains(&next_edge) {
            e.start.j -= 1;
            edge_set.remove(&next_edge);
            j -= 1;
        }
        else {
            break;
        }
    }
    //println!("Result: {}", e);
}

fn extend_edge_north(e: &mut Edge, edge_set: &mut HashSet<Edge>) {
    let (mut i,j) = (e.start.i, e.start.j);
    let direction = e.direction;
    while i > 0 {
        let next_edge = Edge {
            orientation : Orientation::Vertical,
			start: GridPoint::new(i-1,j),
			end: GridPoint::new(i,j),
            direction: direction
        };
        if edge_set.contains(&next_edge) {
            e.start.i -= 1;
            edge_set.remove(&next_edge);
            i -= 1;
        }
        else {
            break;
        }
    }
}

fn extend_edge_south(e: &mut Edge, edge_set: &mut HashSet<Edge>, max_i: usize) {
    let (mut i,j) = (e.end.i, e.end.j);
    let direction = e.direction;
    while i < max_i {
        let next_edge = Edge {
            orientation : Orientation::Vertical,
			start: GridPoint::new(i,j),
			end: GridPoint::new(i+1,j),
            direction: direction
        };
        if edge_set.contains(&next_edge) {
            e.end.i += 1;
            edge_set.remove(&next_edge);
            i += 1;
        }
        else {
            break;
        }
    }
}

fn merge_minor_edges_of_region(region_edges: &mut HashSet<Edge>, map: &Vec<Vec<char>>) -> Vec<Edge> {
    let mut result: Vec<Edge> = Vec::new();
    let max_i = map.len();
    let max_j = map[0].len();
    while let Some(mut e) = region_edges.iter().next().cloned() {
        region_edges.remove(&e);
        if e.orientation == Orientation::Horizontal {
            extend_edge_east(&mut e, region_edges, max_j);
            extend_edge_west(&mut e, region_edges);
        } else if e.orientation == Orientation::Vertical {
            extend_edge_south(&mut e, region_edges, max_i);
            extend_edge_north(&mut e, region_edges);
        }
        result.push(e);
    }
    result
}

fn get_minor_edges_of_region(region: &HashSet<Point>, map: &Vec<Vec<char>>) -> HashSet<Edge> {
    let mut minor_edges: HashSet<Edge> = HashSet::new();
    for point in region {
        for e in get_outer_edges_of_point(point, map) {
            minor_edges.insert(e);
        }
    }
    minor_edges
}

#[test]
fn test_get_minor_edges_of_region() {
    let map = vec![
        vec!['A', 'A', 'A', 'A'],
        vec!['B', 'B', 'C', 'D'],
        vec!['B', 'B', 'C', 'C'],
        vec!['E', 'E', 'E', 'C']
    ];
    let mut region: HashSet<Point> = HashSet::new();
    region.insert(Point{i:0, j:0});
    region.insert(Point{i:0, j:1});
    region.insert(Point{i:0, j:2});
    region.insert(Point{i:0, j:3});

    let edges = get_minor_edges_of_region(&region, &map);
    assert_eq!(edges.len(), 10);
    assert!(edges.contains(&Edge::new(GridPoint::new(0,0), GridPoint::new(0,1)).unwrap()));
}

//#[test]
//fn test_get_region() {
//    let map = vec![
//        vec!['A', 'A', 'A', 'A'],
//        vec!['B', 'B', 'C', 'D'],
//        vec!['B', 'B', 'C', 'C'],
//        vec!['E', 'E', 'E', 'C']
//    ];
//
//    let mut region: HashSet<Point> = HashSet::new();
//    let start: Point = Point {i: 0, j: 0};
//    region.insert(start);
//    get_region(start, &map, &mut region);   
//    assert_eq!(4, region.len());
//    assert(region.contains(&Point{i: 0, j: 0}));
//    assert(region.contains(&Point{i: 0, j: 1}));
//    assert(region.contains(&Point{i: 0, j: 2}));
//    assert(region.contains(&Point{i: 0, j: 3}));
//}

#[test]
fn test_merge_minor_edges_of_region() {
    let map = vec![
        vec!['A', 'A', 'A', 'A'],
        vec!['B', 'B', 'C', 'D'],
        vec!['B', 'B', 'C', 'C'],
        vec!['E', 'E', 'E', 'C']
    ];

    let mut region_edges: HashSet<Edge> = HashSet::new();
    region_edges.insert(Edge::new(GridPoint::new(0, 0), GridPoint::new(0,1)).unwrap());
    region_edges.insert(Edge::new(GridPoint::new(0, 1), GridPoint::new(0,2)).unwrap());
    region_edges.insert(Edge::new(GridPoint::new(0, 2), GridPoint::new(0,3)).unwrap());
    region_edges.insert(Edge::new(GridPoint::new(0, 3), GridPoint::new(0,4)).unwrap());
    region_edges.insert(Edge::new(GridPoint::new(1, 0), GridPoint::new(1,1)).unwrap());
    region_edges.insert(Edge::new(GridPoint::new(1, 1), GridPoint::new(1,2)).unwrap());
    region_edges.insert(Edge::new(GridPoint::new(1, 2), GridPoint::new(1,3)).unwrap());
    region_edges.insert(Edge::new(GridPoint::new(1,3), GridPoint::new(1,4)).unwrap());
    region_edges.insert(Edge::new(GridPoint::new(0, 0), GridPoint::new(1,0)).unwrap());
    region_edges.insert(Edge::new(GridPoint::new(0,4), GridPoint::new(1,4)).unwrap());

    let result = merge_minor_edges_of_region(&mut region_edges, &map);
    assert_eq!(result.len(), 4);
    assert_eq!(region_edges.len(), 0);
    assert!(result.contains(&(Edge::new(GridPoint::new(0,0), GridPoint::new(0,4)).unwrap())));
    assert!(result.contains(&(Edge::new(GridPoint::new(1,0), GridPoint::new(1,4)).unwrap())));
    assert!(result.contains(&(Edge::new(GridPoint::new(0,0), GridPoint::new(1,0)).unwrap())));
    assert!(result.contains(&(Edge::new(GridPoint::new(0,4), GridPoint::new(1,4)).unwrap())));
}
