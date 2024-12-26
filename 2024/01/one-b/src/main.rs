use std::fs;
use std::process;
use std::error::Error;
use std::collections::HashMap;

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

    let mut map: HashMap<u32, (u32,u32)> = HashMap::new();

    for i in 0..first_list.len() {
        match map.get(&first_list[i]) {
            Some((o1, o2)) => map.insert(first_list[i], (o1+1, *o2)),
            None => map.insert(first_list[i], (1, 0))
        };

        match map.get(&second_list[i]) {
            Some((o1, o2)) => map.insert(second_list[i], (*o1, o2+1)),
            None => map.insert(second_list[i], (0, 1))
        };
    }

    let mut sum = 0;
    for num in first_list {
        match map.get(&num) {
            Some((o1, o2)) => sum = sum + num*o1*o2,
            None => {
                println!("{} not found in map.", num);
                process::exit(1);
            }
        };
    }

    println!("{}", sum);

}

fn read_file(file_path: &str) -> Result<String, Box<dyn Error>> {
    let contents = fs::read_to_string(file_path)?;
    Ok(contents)
}
