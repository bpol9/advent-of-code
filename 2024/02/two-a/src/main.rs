use std::fs;
use std::process;
use std::error::Error;

fn main() {
    let input = read_file("D:/advent-of-code/2024/02/two-a/input.txt").unwrap_or_else(|err| {
        println!("Error reading input: {err}");
        process::exit(1);
    });

    let mut safe: u32 = 0;
    for line in input.lines() {
        let levels: Vec<_> = line.split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect();
        let low: i32;
        let high: i32;
        let diff: i32 = levels[0] - levels[1];
        if diff >= 1 && diff <= 3 {
            low = 1;
            high = 3;
        } else if diff >= -3 && diff <= -1 {
            low = -3;
            high = -1;
        } else {
            continue;
        }

        let mut is_safe: bool = true;
        for i in 0..levels.len()-1 {
            let diff = levels[i] - levels[i+1];
            if !(diff >= low && diff <= high) {
                is_safe = false;
                break;
            }
        }

        if is_safe {
            safe = safe + 1;
        }

    }

    println!("{}", safe);
}


fn read_file(file_path: &str) -> Result<String, Box<dyn Error>> {
    let contents = fs::read_to_string(file_path)?;
    Ok(contents)
}
