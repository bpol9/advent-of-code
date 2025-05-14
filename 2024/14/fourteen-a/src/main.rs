use std::fs;
use std::process;
use std::error::Error;

struct Robot {
    v: Velocity,
    p: Position
}

struct Velocity {
    x: i32,
    y: i32
}

struct Position {
    x: i32,
    y: i32
}

struct Floor {
    w: i32,
    h: i32
}

impl Robot {
    fn new(p_x: i32, p_y: i32, v_x: i32, v_y: i32) -> Self {
        Robot {p: Position {x: p_x, y: p_y}, v: Velocity {x: v_x, y: v_y}}
    }

    fn move_me(&mut self, mut seconds: u32, f: Floor) {
        //loop {
        //    if seconds == 0 {
        //        break;
        //    }
        //    self.p.x = math_mod(self.p.x + self.v.x, f.w);
        //    self.p.y = math_mod(self.p.y + self.v.y, f.h);
        //    seconds -= 1;
        //}

        self.p.x = math_mod(self.p.x + (seconds as i32)*self.v.x, f.w);
        self.p.y = math_mod(self.p.y + (seconds as i32)*self.v.y, f.h);
    }
}


fn main() {
    let file_input = read_file("D:/advent-of-code/2024/14/fourteen-a/input.txt").unwrap_or_else(|err| {
        println!("Error reading input: {err}");
        process::exit(1);
    });

    let mut rs: Vec<Robot> = Vec::new();
    for line in file_input.lines() {
        let p_x = line.chars().skip(line.find('=').unwrap() + 1)
            .take(line.find(',').unwrap() - line.find('=').unwrap() - 1)
            .collect::<String>().parse::<i32>().unwrap();
        let p_y = line.chars().skip(line.find(',').unwrap() + 1)
            .take(line.find(' ').unwrap() - line.find(',').unwrap() - 1)
            .collect::<String>().parse::<i32>().unwrap();
        let v_x = line.chars().skip(line.rfind('=').unwrap() + 1)
            .take(line.rfind(',').unwrap() - line.rfind('=').unwrap() - 1)
            .collect::<String>().parse::<i32>().unwrap();
        let v_y = line.chars().skip(line.rfind(',').unwrap() + 1)
            .take(line.len() - line.rfind(',').unwrap() - 1)
            .collect::<String>().parse::<i32>().unwrap();

        rs.push(Robot::new(p_x, p_y, v_x, v_y));
    }

    let mut floor = vec![vec![0; 101]; 103];
    for mut r in rs {
        r.move_me(100, Floor {w: 101, h: 103});
        floor[r.p.y as usize][r.p.x as usize] += 1;
    }

    let i_max = floor.len()/2;
    let j_max = floor[0].len()/2;
    let i_step = floor.len()/2 + 1;
    let j_step = floor[0].len()/2 + 1;
    let mut q1: u64 = 0;
    let mut q2: u64 = 0;
    let mut q3: u64 = 0;
    let mut q4: u64 = 0;
    for i in 0..i_max {
        for j in 0..j_max {
            q1 += floor[i][j];
            q2 += floor[i][j+j_step];
            q3 += floor[i+i_step][j];
            q4 += floor[i+i_step][j+j_step];
        }
    }

    println!("{}", q1*q2*q3*q4);
            

    //for i in 0..floor.len() {
    //    for j in 0..floor[i].len() {
    //        if floor[i][j] == 0 {
    //            print!(".");
    //        } else {
    //            print!("{}", floor[i][j]);
    //        }
    //    }
    //    println!("");
    //}
}

fn read_file(file_path: &str) -> Result<String, Box<dyn Error>> {
    let contents = fs::read_to_string(file_path)?;
    Ok(contents)
}

fn math_mod(a: i32, b: i32) -> i32 {
    return ((a % b) + b) % b;
}
