use std::fs;
use std::process;
use std::error::Error;
use std::collections::HashSet;
use std::cmp;

#[derive(Copy, Clone, Debug)]
struct Way {
    times_a: usize,
    times_b: usize
}

#[derive(Copy, Clone)]
struct Button {
    x: usize,
    y: usize
}

#[derive(Copy, Clone)]
struct Location {
    x: usize,
    y: usize
}

#[derive(Copy, Clone)]
struct Configuration {
    a: Button,
    b: Button,
    prize_loc: Location
}

impl Way {
    fn cost(&self) -> usize {
        return self.times_a * 3 + self.times_b;
    }
}

fn get_ways(conf: Configuration) -> Vec<Way> {
    let mut ways: Vec<Way> = Vec::new();
    let times_a_max = cmp::min(conf.prize_loc.x / conf.a.x, conf.prize_loc.y / conf.a.y);
    for i in 1..=times_a_max {
        let remainder_x = conf.prize_loc.x - i*conf.a.x;
        let remainder_y = conf.prize_loc.y - i*conf.a.y;
        if remainder_x % conf.b.x == 0 && remainder_y % conf.b.y == 0 && remainder_x / conf.b.x == remainder_y / conf.b.y {
            ways.push(Way {times_a: i, times_b: remainder_x / conf.b.x});
        }
    }

    //let times_b_max = cmp::min(prize.x / b.x, prize.y / b.y);
    //for i in 1..=times_b_max {
    //    let remainder_x = prize.x - i*b.x;
    //    let remainder_y = prize.y - i*b.y;
    //    if remainder_x % a.x == 0 && remainder_y % a.y == 0 && remainder_x / a.x == remainder_y / a.y {
    //        ways.push(Way {times_a: remainder_x / a.x, times_b: i});
    //    }
    //}

    ways
}

fn read_file(file_path: &str) -> Result<String, Box<dyn Error>> {
    let contents = fs::read_to_string(file_path)?;
    Ok(contents)
}

fn parse_configuration(conf: &str) -> Configuration {
    //println!("parse_configuration on {}", conf);
    let lines: Vec<&str> = conf.split("\n").collect();
    let a_x = lines[0].chars().skip(lines[0].find('+').unwrap() + 1)
        .take(lines[0].find(',').unwrap() - lines[0].find('+').unwrap() - 1)
        .collect::<String>().parse::<usize>().unwrap();
    let a_y = lines[0].chars().skip(lines[0].rfind('+').unwrap() + 1)
        .take(lines[0].len() - lines[0].rfind('+').unwrap() - 1)
        .collect::<String>().parse::<usize>().unwrap();
    let b_x = lines[1].chars().skip(lines[1].find('+').unwrap() + 1)
        .take(lines[1].find(',').unwrap() - lines[1].find('+').unwrap() - 1)
        .collect::<String>().parse::<usize>().unwrap();
    let b_y = lines[1].chars().skip(lines[1].rfind('+').unwrap() + 1)
        .take(lines[1].len() - lines[1].rfind('+').unwrap() - 1)
        .collect::<String>().parse::<usize>().unwrap();
    let prize_x = lines[2].chars().skip(lines[2].find('=').unwrap() + 1)
        .take(lines[2].find(',').unwrap() - lines[2].find('=').unwrap() - 1)
        .collect::<String>().parse::<usize>().unwrap();
    let prize_y = lines[2].chars().skip(lines[2].rfind('=').unwrap() + 1)
        .take(lines[2].len() - lines[2].rfind('=').unwrap() - 1)
        .collect::<String>().parse::<usize>().unwrap();

    Configuration { a: Button {x: a_x, y: a_y}, b: Button {x: b_x, y: b_y}, prize_loc: Location {x: prize_x, y: prize_y}}
}



fn main() {
    let file_input = read_file("D:/advent-of-code/2024/13/thirteen-a/input.txt").unwrap_or_else(|err| {
        println!("Error reading input: {err}");
        process::exit(1);
    });

    let mut sum: usize = 0;
    for conf in file_input.split("\n\n").collect::<Vec<_>>() {
        let mut cheapest = usize::MAX;
        for way in get_ways(parse_configuration(conf)) {
            let cost = way.cost();
            if cost < cheapest {
                cheapest = cost;
            }
        }
        if cheapest < usize::MAX {
            sum += cheapest;
        }
    }

    println!("{}", sum);
}

#[test]
fn test_parse_configuration() {
    let conf_string = String::from("Button a: X+62, Y+48\nButton b: X+21, Y+74\nPrize: X=3946, Y=6404");
    let conf = parse_configuration(&conf_string);
    assert_eq!(conf.a.x, 62);
    assert_eq!(conf.a.y, 48);
    assert_eq!(conf.b.x, 21);
    assert_eq!(conf.b.y, 74);
    assert_eq!(conf.prize_loc.x, 3946);
    assert_eq!(conf.prize_loc.y, 6404);
}
