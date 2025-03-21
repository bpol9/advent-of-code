use std::fs;
use std::process;
use std::error::Error;

fn main() {
    let input = read_file("D:/advent-of-code/2024/02/two-a/input.txt").unwrap_or_else(|err| {
        println!("Error reading input: {err}");
        process::exit(1);
    });

    let mut safe: u32 = 0;
    let mut cnt: u32 = 0;
    for line in input.lines() {
        cnt = cnt + 1;
        let mut levels: Vec<_> = line.split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect();
        let mut has_spent_deletion: bool = false;
        let mut low: i32 = -1; // bogus(?) initialization for compile pass
        let mut high: i32 = -1; // bogus(?) initialization for compile pass
        let mut triplet_safe: bool = false;

        if !are_compatible(levels[0], levels[1]) {
            has_spent_deletion = true;
            while true {
                (triplet_safe, low, high) = is_triplet_compatible(levels[0], levels[2], levels[3]);
                if triplet_safe {
                    levels.remove(1);
                    break;
                }

                (triplet_safe, low, high) = is_triplet_compatible(levels[1], levels[2], levels[3]);
                if triplet_safe {
                    levels.remove(0);
                }

                break;
                
            }

            if !triplet_safe { // removing either levels[0] or levels[1] doesn't make the sequence
                               // safe
                continue;
            }


            //has_spent_deletion = true;
            //(triplet_safe, low, high) = is_triplet_compatible(levels[0], levels[2], levels[3]);
            //if triplet_safe {
            //    levels.remove(1);
            //}
            //else {
            //    (triplet_safe, low, high) = is_triplet_compatible(levels[1], levels[2], levels[3]);
            //    levels.remove(0);
            //}

            //if !triplet_safe { // removing either of levels[0], levels[1] doesn't make the sequence
            //                   // safe.
            //    continue;
            //}

        } else {
            (low, high) = if levels[0] > levels[1] {
                (1,3)
            } else {
                (-3, -1)
            }
        }

        if !is_diff_within(&[levels[1], levels[2]], low, high) {
            has_spent_deletion = true;
            while true {
                (triplet_safe, low, high) = is_triplet_compatible(levels[1], levels[2], levels[3]);
                if triplet_safe {
                    levels.remove(0);
                    break;
                }
                (triplet_safe, low, high) = is_triplet_compatible(levels[0], levels[2], levels[3]);
                if triplet_safe {
                    levels.remove(1);
                    break;
                }
                (triplet_safe, low, high) = is_triplet_compatible(levels[0], levels[1], levels[3]);
                levels.remove(2);
                break;
            }
            if !triplet_safe {
                continue;
            }
        }
        
        let mut is_safe: bool = true;
        let mut j: usize = levels.len();
        for i in 2..levels.len()-1 {
            if !is_diff_within(&[levels[i], levels[i+1]], low, high) {
                if has_spent_deletion {
                    is_safe = false;
                    break;
                }
                else if i == levels.len() - 2 {
                    is_safe = true;
                    break;
                }
                else if is_diff_within(&[levels[i], levels[i+2]], low, high) {
                    levels.remove(i+1);
                    has_spent_deletion = true;
                    j = i+1;
                    break;
                }
                else if is_diff_within(&[levels[i-1], levels[i+1], levels[i+2]], low, high) {
                    levels.remove(i);
                    has_spent_deletion = true;
                    j = i + 1;
                    break;
                }
                else {
                    is_safe = false;
                    break;
                }
            }
        }

        for i in j..levels.len()-1 {
            if !is_diff_within(&[levels[i], levels[i+1]], low, high) {
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

fn are_compatible(first: i32, second: i32) -> bool {
    let diff = first - second;
    return (diff >= 1 && diff <= 3) || (diff >= -3 && diff <= -1);
}

fn is_triplet_compatible(first: i32, second: i32, third: i32) -> (bool, i32, i32) {
    let diff1: i32 = first - second;
    let diff2: i32 = second - third;
    if diff1 >= 1 && diff1 <= 3 {
        return (diff2 >= 1 && diff2 <=3, 1, 3);
    } else if diff1 >= -3 && diff1 <= -1 {
        return (diff2 >= -3 && diff2 <= -1, -3, -1);
    } else {
        return (false, 0, 0);
    }
}

//fn is_diff_within(first: i32, second: i32, low: i32, high: i32) -> bool {
//    let diff = first - second;
//    return diff >= low && diff <= high;
//}

fn is_diff_within(arr: &[i32], low: i32, high: i32) -> bool {
    let diff: i32;
    for i in 0..arr.len()-1 {
        let diff = arr[i] - arr[i+1];
        if !(diff >= low && diff <= high) {
            return false;
        }
    }
    return true;
}
