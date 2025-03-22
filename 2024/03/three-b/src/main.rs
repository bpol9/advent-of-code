use regex::Regex;
use std::fs;
use std::process;
use std::error::Error;

fn main() {
    let input = read_file("D:/advent-of-code/2024/03/three-a/input.txt").unwrap_or_else(|err| {
        println!("Error reading input: {err}");
        process::exit(1);
    });

    let v: Vec<&str> = input.split("don't()").collect();
    let mut sum: u32 = get_sum(v[0]);
    for i in 1..v.len() {
        let text = v[i];
        let vv: Vec<&str> = text.split("do()").collect();
        if vv.len() == 1 {
            continue;
        }
        for j in 1..vv.len() {
            sum = sum + get_sum(vv[j]);
        }
    }
    println!("{}", sum);
}

fn read_file(file_path: &str) -> Result<String, Box<dyn Error>> {
    let contents = fs::read_to_string(file_path)?;
    Ok(contents)
}

fn get_sum(input: &str) -> u32 {
    let r = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();
    let mut sum: u32 = 0;
    for (_, [first, second]) in r.captures_iter(&input).map(|c| c.extract()) {
        let first_num = &first.parse::<u32>().unwrap();
        let second_num = &second.parse::<u32>().unwrap();
        sum = sum + first_num * second_num;
    }
    return sum;
}
