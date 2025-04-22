use std::fs;
use std::process;
use std::error::Error;
use std::collections::HashMap;

fn main() {
    let file_input = read_file("D:/advent-of-code/2024/11/eleven-a/input.txt").unwrap_or_else(|err| {
        println!("Error reading input: {err}");
        process::exit(1);
    });

    let stones: Vec<u64> = file_input.trim().split(" ").map(|n| n.parse::<u64>().unwrap()).collect();
    let mut stones_num: u64 = 0;
    let mut computations: HashMap<u64, Vec<(u8, u64)>> = HashMap::new();
    for s in stones {
        stones_num += number_of_stones_after_blinks(s, 75, &mut computations);
    }
    println!("{}", stones_num);
}

fn blink(num: u64) -> (u64, Option<u64>) {
    let digits_num = get_number_of_digits(num);
    if num == 0 {
        return (1, None);
    } else if digits_num % 2 == 0 {
        let divider = 10u64.pow(digits_num / 2);
        let left_value: u64 = num / divider;
        let right_value: u64 = num % divider;
        return (left_value, Some(right_value));
    } else {
        return (num * 2024, None);
    }
}

fn number_of_stones_after_blinks(initial_stone: u64, times: u8, computations: &mut HashMap<u64, Vec<(u8, u64)>>) -> u64 {
    if times == 0 {
        return 1;
    }

    if let Some(num) = get_computed_number_of_stones(initial_stone, times, computations) {
        //println!("Computation found");
        return num;
    }

    let (left, right) = blink(initial_stone);
    let mut stones_num = number_of_stones_after_blinks(left, times - 1, computations);
    if let Some(r) = right {
        stones_num += number_of_stones_after_blinks(r, times - 1, computations);
    }
    match computations.get_mut(&initial_stone) {
        None => {
            computations.insert(initial_stone, vec![(times, stones_num)]);
        },
        Some(v) => {
            v.push((times, stones_num));
        }
    }
    return stones_num;
}

fn get_computed_number_of_stones(initial_stone: u64, blinks_number: u8, map: &HashMap<u64, Vec<(u8, u64)>>) -> Option<u64> {
    match map.get(&initial_stone) {
        None => { return None; },
        Some(v) => {
            for (blinks, stones_num) in v {
                if *blinks == blinks_number {
                    return Some(*stones_num);
                }
            }
            return None;
        }
    }
}
            
fn get_number_of_digits(n: u64) -> u32 {
    let mut digits_num = 1;
    let mut q = n / 10;
    while q != 0 {
        digits_num += 1;
        q = q / 10;
    }

    digits_num
}

fn read_file(file_path: &str) -> Result<String, Box<dyn Error>> {
    let contents = fs::read_to_string(file_path)?;
    Ok(contents)
}

#[test]
fn test_get_number_of_digits() {
    assert_eq!(5, get_number_of_digits(12345));
    assert_eq!(3, get_number_of_digits(347));
}
