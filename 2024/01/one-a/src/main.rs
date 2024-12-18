use std::fs;
use std::process;
use std::error::Error;

fn main() {
 let input = read_file("D:/advent-of-code/2024/01/one-a/input.txt").unwrap_or_else(|err| {
     println!("Error reading input: {err}");
     process::exit(1);
 });

 let mut first_list = Vec::new();
 let mut second_list = Vec::new();
 for line in input.lines() {
     let ids: Vec<_> = line.split_whitespace().map(|x| x.parse::<u32>().unwrap()).collect();
     first_list.push(ids[0]);
     second_list.push(ids[1]);
 }

 first_list.sort();
 second_list.sort();
 let pairs: Vec<(&u32, &u32)> = first_list.iter().zip(second_list.iter()).collect();

 let mut sum: u32 = 0;
 for (a,b) in pairs {
     sum += a.abs_diff(*b);
 }

 println!("Result = {}", sum);
}

fn read_file(file_path: &str) -> Result<String, Box<dyn Error>> {
    let contents = fs::read_to_string(file_path)?;
    Ok(contents)
}

