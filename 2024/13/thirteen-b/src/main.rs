use std::fs;
use std::process;
use std::error::Error;
use std::collections::HashSet;

#[derive(Copy, Clone, Debug)]
struct Way {
    k: u64,
    l: u64
}

#[derive(Copy, Clone)]
struct Button {
    x: u64,
    y: u64
}

#[derive(Copy, Clone)]
struct Location {
    x: u64,
    y: u64
}

#[derive(Copy, Clone)]
struct Configuration {
    a: Button,
    b: Button,
    p: Location
}

impl Way {
    fn new(k: u64, l: u64) -> Self {
        Way {k, l}
    }

    fn cost(&self) -> u64 {
        return self.k * 3 + self.l;
    }
}

impl Configuration {
    fn is_satisfied_by(&self, k: u64, l: u64) -> bool {
        return k*self.a.x + l*self.b.x == self.p.x && k*self.a.y + l*self.b.y == self.p.y;
    }
}

fn main() {
    let file_input = read_file("D:/advent-of-code/2024/13/thirteen-a/input.txt").unwrap_or_else(|err| {
        println!("Error reading input: {err}");
        process::exit(1);
    });

    let mut sum: u64 = 0;
    for conf in file_input.split("\n\n").collect::<Vec<_>>() {
        let mut cheapest = u64::MAX;
        for way in get_ways_for_big_numbers(parse_configuration(conf)) {
            let cost = way.cost();
            if cost < cheapest {
                cheapest = cost;
            }
        }
        if cheapest < u64::MAX {
            sum += cheapest;
        }
    }

    println!("{}", sum);
}

// The logic is based on the following (assuming N >= M)
// k*A_x + l*B_x = N (1)
// k*A_y + l*B_y = M (2)
// Subtract by parts -> k(A_x - A_y) + l(B_x - B_y) = N - M -> k*C + l*D = E (3)
// (a) if E = 0, C = 0, D = 0:
//      then A_x = A_y, B_x = B_y, N = M and equations (1) and (2) are the same
//      find solutions (k,l) of (1) as in problem 13a but it will be slow because of big N.
// (b) if both C,D are positive:
//      Get the (k,l) solutions of (3) as in problem 13a. It will be fast because of small E.
//      check each (k,l) solution against (1) and (2)
// (c) if one of C,D is negative:
//      Get the (k,l) solutions of equation (3) in closed form -> k = ko + i*(lcm(C,D)/C),
//      l = lo + i*(lcm(C,D)/D), i=0,1,2,... (ko,lo) is the smallest (k,l) for which (3) is satisfied
//      put these solutions in (1) and (2) and find i.
fn get_ways_for_big_numbers(conf: Configuration) -> Vec<Way> {
    let mut ways = Vec::new();
    let c: i64;
    let d: i64;
    let e: i64;
    if conf.p.x > conf.p.y {
        c = conf.a.x as i64 - conf.a.y as i64;
        d = conf.b.x as i64 - conf.b.y as i64;
        e = conf.p.x as i64 - conf.p.y as i64;
    } else {
        c = conf.a.y as i64 - conf.a.x as i64;
        d = conf.b.y as i64 - conf.b.x as i64;
        e = conf.p.y as i64 - conf.p.x as i64;
    }

    if e == 0 && c == 0 && d == 0 { // case (a)
        ways = get_ways(conf.a.x, conf.b.x, conf.p.x);
    } else if c > 0 && d > 0 { // case (b)
        ways = get_ways(c as u64, d as u64, e as u64).into_iter().filter(|w| conf.is_satisfied_by(w.k, w.l)).collect();
    } else if c > 0 && d < 0 { // case (c)
        let cc = c as u64;
        let dd = -d as u64;
        let ee = e as u64;
        if let Some((ko, lo)) = find_ko_lo(cc, dd, ee) {
            if let Some((k,l)) = get_k_l(ko, lo, conf) {
                ways.push(Way::new(k,l));
            }
        }
    } else if c < 0 && d > 0 { // case (c)
        let cc = -c as u64;
        let dd = d as u64;
        let ee = e as u64;
        if let Some((lo, ko)) = find_ko_lo(dd, cc, ee) {
            if let Some((k,l)) = get_k_l(ko, lo, conf) {
                ways.push(Way::new(k,l));
            }
        }
    } else if c == 0 && d > 0 && e % d == 0 { // k*c + l*d = e => l = e / d
        let l = e as u64 / d as u64;
        if (conf.p.x - l*conf.b.x) % conf.a.x == 0 { // compute k from equation (1)
            let k = (conf.p.x - l*conf.b.x) / conf.a.x;
            if conf.is_satisfied_by(k, l) {
                ways.push(Way::new(k,l));
            }
        }
    } else if c > 0 && d == 0 && e % c == 0 {
        let k = e as u64 / c as u64;
        if (conf.p.x - k*conf.a.x) % conf.b.x == 0 {
            let l = (conf.p.x - k*conf.a.x) / conf.b.x;
            if conf.is_satisfied_by(k, l) {
                ways.push(Way::new(k,l));
            }
        }
    }

    return ways;
}

fn get_k_l(ko: u64, lo: u64, conf: Configuration) -> Option<(u64,u64)> {
    let n = conf.p.x as i64;
    let a_x = conf.a.x;
    let b_x = conf.b.x;
    let c = (conf.a.x as i64 - conf.a.y as i64).abs() as u64;
    let d = (conf.b.x as i64 - conf.b.y as i64).abs() as u64;

    let lcm = lcm(c, d);
    let num: i64 = n - (ko as i64)*(a_x as i64) - (lo as i64)*(b_x as i64);
    if num < 0 {
        return None;
    }
    let den = a_x*lcm/c + b_x*lcm/d;
    if num as u64 % den == 0 {
        let i = num as u64 / den;
        let k = ko + i*(lcm/c);
        let l = lo + i*(lcm/d);
        if conf.is_satisfied_by(k,l) {
            return Some((k,l));
        } else {
            return None;
        }
    } else {
        return None;
    }
}

fn lcm(a: u64, b: u64) -> u64 {
    return (a/gcd(a,b))*b;
}

// find (k,l) pairs for which k*a + l*b = n
fn get_ways(a: u64, b: u64, n: u64) -> Vec<Way> {
    let mut ways = Vec::new();
    let max_times_a = n / a;
    for i in 0..=max_times_a {
        let remainder = n - i*a;
        if remainder % b == 0 {
            ways.push(Way::new(i, remainder / b));
        }
    }
    ways
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

// find smallest (k,l) that satisfies this -> a*k - b*l = n
fn find_ko_lo(a: u64, b: u64, n: u64) -> Option<(u64, u64)> {
   if n % a == 0 {
        return Some((n/a, 0));
   }

   let aa = a as i64;
   let bb = b as i64;
   let nn = n as i64;
   let min_k = nn / aa + 1;
   let mut visited = HashSet::new();
   let mut sum = min_k * aa;
   let mut l = 0;
   let mut k = min_k as u64;
   loop {
        if visited.contains(&sum) {
            return None;
        } else {
            visited.insert(sum);
        }

        if sum > nn {
            sum -= bb;
            l += 1;
        } else if sum < nn {
            sum += aa;
            k += 1;
        } else {
            return Some((k,l));
        }
   }
}

fn read_file(file_path: &str) -> Result<String, Box<dyn Error>> {
    let contents = fs::read_to_string(file_path)?;
    Ok(contents)
}

fn parse_configuration(conf: &str) -> Configuration {
    let lines: Vec<&str> = conf.split("\n").collect();
    let a_x = lines[0].chars().skip(lines[0].find('+').unwrap() + 1)
        .take(lines[0].find(',').unwrap() - lines[0].find('+').unwrap() - 1)
        .collect::<String>().parse::<u64>().unwrap();
    let a_y = lines[0].chars().skip(lines[0].rfind('+').unwrap() + 1)
        .take(lines[0].len() - lines[0].rfind('+').unwrap() - 1)
        .collect::<String>().parse::<u64>().unwrap();
    let b_x = lines[1].chars().skip(lines[1].find('+').unwrap() + 1)
        .take(lines[1].find(',').unwrap() - lines[1].find('+').unwrap() - 1)
        .collect::<String>().parse::<u64>().unwrap();
    let b_y = lines[1].chars().skip(lines[1].rfind('+').unwrap() + 1)
        .take(lines[1].len() - lines[1].rfind('+').unwrap() - 1)
        .collect::<String>().parse::<u64>().unwrap();
    let prize_x = lines[2].chars().skip(lines[2].find('=').unwrap() + 1)
        .take(lines[2].find(',').unwrap() - lines[2].find('=').unwrap() - 1)
        .collect::<String>().parse::<u64>().unwrap() + 10000000000000;
    let prize_y = lines[2].chars().skip(lines[2].rfind('=').unwrap() + 1)
        .take(lines[2].len() - lines[2].rfind('=').unwrap() - 1)
        .collect::<String>().parse::<u64>().unwrap() + 10000000000000;

    Configuration { a: Button {x: a_x, y: a_y}, b: Button {x: b_x, y: b_y}, p: Location {x: prize_x, y: prize_y}}
}

#[test]
fn test_get_k_l() {
    let result = find_ko_lo(10, 5, 5);
    assert!(result.is_some());
    assert_eq!(result.unwrap().0, 1);
    assert_eq!(result.unwrap().1, 1);

}
