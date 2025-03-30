use std::collections::HashMap;
use std::fs;
use std::process;
use std::error::Error;

fn main() {
    let file_input = read_file("D:/advent-of-code/2024/05/five-a/input.txt").unwrap_or_else(|err| {
        println!("Error reading input: {err}");
        process::exit(1);
    });
    let mut rules: HashMap<u32, Vec<u32>> = HashMap::new();
    let mut updates: Vec<Vec<u32>> = Vec::new();
    
    let mut reading_rules: bool = true;
    for line in file_input.lines() {

        if line.is_empty() {
            reading_rules = false;
            continue;
        }

        if reading_rules {
            let parts: Vec<u32> = line.split("|").map(|x| x.parse::<u32>().unwrap()).collect();
            match rules.get_mut(&parts[0]) {
                Some(v) => v.push(parts[1]),
                None => {rules.insert(parts[0], vec![parts[1]]); ()}
            };
        }
        else {
            updates.push(line.split(",").map(|x| x.parse::<u32>().unwrap()).collect());
        }
    }

    let mut sum = 0;
    let mut right_order: bool;
    for i in 0..updates.len() {
        right_order = true;
        for j in 0..updates[i].len() {
            let page = updates[i][j];
            if let Some(v) = rules.get(&page) {
                for k in 0..j {
                    if v.contains(&updates[i][k]) {
                        right_order = false;
                        break;
                    }
                }
            }
            if !right_order {
                break;
            }
        }

        if right_order {
            sum = sum + updates[i][updates[i].len() / 2];
        }
    }


    println!("{}", sum);

}

fn read_file(file_path: &str) -> Result<String, Box<dyn Error>> {
    let contents = fs::read_to_string(file_path)?;
    Ok(contents)
}
