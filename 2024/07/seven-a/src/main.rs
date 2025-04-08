use std::fs;
use std::process;
use std::error::Error;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Operator {
    Plus,
    Times,
}

fn main() {
    let file_input = read_file("D:/advent-of-code/2024/07/seven-a/input.txt").unwrap_or_else(|err| {
        println!("Error reading input: {err}");
        process::exit(1);
    });

    let mut results: Vec<u64> = Vec::new();
    let mut sequences: Vec<Vec<u64>> = Vec::new();

    let mut max_len: usize = 0;
    let mut cnt: usize = 0;
    for line in file_input.lines() {
        let parts = line.split(": ").collect::<Vec<&str>>();
        results.push(parts[0].parse::<u64>().unwrap());
        sequences.push(parts[1].split(" ").map(|n| n.parse::<u64>().unwrap()).collect::<Vec<u64>>());
        if sequences[cnt].len() > max_len {
            max_len = sequences[cnt].len();
        }
        cnt += 1;
    }

    let combinations = get_combinations(max_len-1);
    let mut sum: u64 = 0;
    for i in 0..results.len() {
        let len = sequences[i].len();
        let combinations_num = 2u32.pow((len-1) as u32) as usize;
        let range_start = combinations.len() - combinations_num;

        for j in range_start..combinations.len() {
            if are_equal(results[i], &sequences[i], &(combinations[j][(max_len - len)..])) {
                sum += results[i];
                break;
            }
        }
    }

    println!("{}", sum);
}

fn are_equal(v: u64, vs: &Vec<u64>, operators: &[Operator]) -> bool {
    let mut acc: u64 = vs[0];
    for i in 0..operators.len() {
        match operators[i] {
            Operator::Plus => acc += vs[i+1],
            Operator::Times => acc *= vs[i+1]
        }
    }

    return acc == v;
}

fn get_combinations(len: usize) -> Vec<Vec<Operator>> {
    if len == 1 {
        return vec![vec![Operator::Plus], vec![Operator::Times]];
    }

    let mut rec: Vec<Vec<Operator>> = get_combinations(len - 1);
    rec.extend(rec.clone());
    let half_len: usize = rec.len()/2;
    for i in 0..half_len {
        rec[i].insert(0, Operator::Plus);
        rec[i+half_len].insert(0, Operator::Times);
    }

    return rec;
}

fn read_file(file_path: &str) -> Result<String, Box<dyn Error>> {
    let contents = fs::read_to_string(file_path)?;
    Ok(contents)
}
