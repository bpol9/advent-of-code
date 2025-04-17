use std::fs;
use std::process;
use std::error::Error;

fn main() {
    let file_input = read_file("D:/advent-of-code/2024/09/nine-a/input.txt").unwrap_or_else(|err| {
        println!("Error reading input: {err}");
        process::exit(1);
    });

    let input_map: Vec<u8> = file_input.chars().filter(|x| x.is_numeric()).map(|x| x.to_digit(10).unwrap() as u8).collect();
    let mut alt_map: Vec<Option<u64>> = get_alt_map(&input_map);
    defrag(&mut alt_map);
    println!("{}", compute_checksum(&alt_map));

}

fn get_alt_map(map: &[u8]) -> Vec<Option<u64>> {
    let mut file_id: u64 = 0;
    let mut alt_map: Vec<Option<u64>> = Vec::new();
    let mut o: Option<u64>;
    for i in 0..map.len() {
        if i % 2 == 0 {
            o = Some(file_id);
            file_id += 1;
        } else {
            o = None;
        }
        for _j in 0..map[i] {
            alt_map.push(o);
        }
    }

    return alt_map;
}

fn defrag(disk: &mut [Option<u64>]) {
    let mut j: usize = disk.len() - 1;
    let mut i: usize = 0;

    loop {
        while disk[j].is_none() {
            j -= 1;
        }
        while disk[i].is_some() {
            i += 1;
        }
        if j <= i {
            break;
        }

        let tmp = disk[i];
        disk[i] = disk[j];
        disk[j] = tmp;
    }
}

fn compute_checksum(disk: &[Option<u64>]) -> u64 {
    let mut sum: u64 = 0;
    for i in 0..disk.len() {
        if let Some(file_id) = disk[i] {
            sum += i as u64 * file_id;
        } else {
            break;
        }
    }

    return sum;
}

fn read_file(file_path: &str) -> Result<String, Box<dyn Error>> {
    let contents = fs::read_to_string(file_path)?;
    Ok(contents)
}
