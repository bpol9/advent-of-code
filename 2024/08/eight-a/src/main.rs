use std::fs;
use std::process;
use std::error::Error;
use std::collections::HashMap;

fn main() {
    let file_input = read_file("D:/advent-of-code/2024/08/eight-a/input.txt").unwrap_or_else(|err| {
        println!("Error reading input: {err}");
        process::exit(1);
    });

    let mut antena_locations: HashMap<char, Vec<(usize, usize)>> = HashMap::new();

    let mut row: usize = 0;
    let mut col: usize = 0;
    let mut max_number_of_antennas: usize = 0;
    let mut max_col: usize = 0;
    let max_row: usize;
    for line in file_input.lines() {
        if max_col == 0 {
            max_col = line.len() - 1;
        }
        for c in line.chars() {
            if c != '.' {
                match antena_locations.get_mut(&c) {
                    Some(v) => {
                        v.push((row,col));
                        if v.len() > max_number_of_antennas {
                            max_number_of_antennas = v.len();
                        }
                    },
                    None => {
                        antena_locations.insert(c, vec![(row,col)]);
                        if 1 > max_number_of_antennas {
                            max_number_of_antennas = 1;
                        }
                    }
                }
            }
            col += 1;
        }
        col = 0;
        row += 1;
    }
    max_row = row - 1;

    let combinations_of_two: HashMap<usize, Vec<(usize, usize)>> = combinations_of_two(max_number_of_antennas - 1);
    let mut antinode_points: HashMap<(usize, usize), bool> = HashMap::new();

    for (_key, value) in antena_locations {
        let number_of_points = value.len();
        let mut index_pairs: Vec<(usize, usize)> = Vec::new();
        for i in 0..number_of_points {
            index_pairs.extend(combinations_of_two.get(&i).unwrap().clone());
        }

        for p in index_pairs {
            let (antinode_1, antinode_2) = get_antinodes(value[p.0], value[p.1], max_row, max_col);
            if let Some(antinode) = antinode_1 {
                antinode_points.insert(antinode, true);
            }
            if let Some(antinode) = antinode_2 {
                antinode_points.insert(antinode, true);
            }
        }
    }

    println!("{}", antinode_points.len());

}

fn combinations_of_two(n: usize) -> HashMap<usize, Vec<(usize, usize)>> {
    let mut result: HashMap<usize, Vec<(usize, usize)>> = HashMap::new();
    for i in 0..=n {
        let mut v: Vec<(usize, usize)> = vec![];
        for j in 0..i {
            v.push((i,j));
        }
        result.insert(i,v);
    }

    return result;
}

fn get_antinodes((i1,j1): (usize, usize), (i2,j2): (usize, usize), i_max: usize, j_max: usize) -> (Option<(usize, usize)>, Option<(usize, usize)>) {
    let i_max_signed = i_max as i32;
    let j_max_signed = j_max as i32;
    let i3 = i1 as i32 - (i2 as i32 - i1 as i32);
    let j3 = j1 as i32 - (j2 as i32 - j1 as i32);

    let i4 = i2 as i32 - (i1 as i32 - i2 as i32);
    let j4 = j2 as i32 - (j1 as i32 - j2 as i32);

    let first_option = if i3 >= 0 && i3 <= i_max_signed && j3 >=0 && j3 <= j_max_signed {
        Some((i3 as usize, j3 as usize))
    } else {
        None
    };

    let second_option = if i4 >= 0 && i4 <= i_max_signed && j4 >=0 && j4 <= j_max_signed {
        Some((i4 as usize, j4 as usize))
    } else {
        None
    };

    return (first_option, second_option);
}

fn read_file(file_path: &str) -> Result<String, Box<dyn Error>> {
    let contents = fs::read_to_string(file_path)?;
    Ok(contents)
}
