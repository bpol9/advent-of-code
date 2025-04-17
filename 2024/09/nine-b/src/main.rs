use std::fs;
use std::process;
use std::error::Error;

#[derive(Eq, PartialEq)]
struct FreeRange {
    start: usize,
    blocks: usize
}

#[derive(Eq, PartialEq)]
struct File {
    start: usize,
    blocks: usize,
    id: u64
}

fn main() {
    let file_input = read_file("D:/advent-of-code/2024/09/nine-a/input.txt").unwrap_or_else(|err| {
        println!("Error reading input: {err}");
        process::exit(1);
    });

    let input_map: Vec<u8> = file_input.chars().filter(|x| x.is_numeric()).map(|x| x.to_digit(10).unwrap() as u8).collect();
    let alt_map: Vec<Option<u64>> = get_alt_map(&input_map);
    let (mut free_ranges, mut files) = get_free_ranges_and_files(&alt_map);
    defrag(&mut free_ranges, &mut files);
    println!("{}", compute_checksum(&files));

}

fn compute_checksum(files: &[File]) -> u64 {
    let mut sum: u64 = 0;
    for f in files {
        for i in f.start..f.start+f.blocks {
            sum += i as u64 * f.id;
        }
    }
    return sum;
}

fn defrag(ranges: &mut [FreeRange], files: &mut [File]) {
    for f in files {
        let mut i: i64 = ranges.len() as i64 - 1;
        loop {
            if i < 0 {
                break;
            }
            if ranges[i as usize].start > f.start {
                break;
            } else if ranges[i as usize].blocks < f.blocks {
                i -= 1;
            } else {
                put_file_to_free_range(f, &mut ranges[i as usize]);
                break;
            }
        }
    }

}


fn put_file_to_free_range(file: &mut File, range: &mut FreeRange) {
    file.start = range.start;
    range.start += file.blocks;
    range.blocks -= file.blocks;
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

fn read_file(file_path: &str) -> Result<String, Box<dyn Error>> {
    let contents = fs::read_to_string(file_path)?;
    Ok(contents)
}

fn get_free_ranges_and_files(disk: &[Option<u64>]) -> (Vec<FreeRange>, Vec<File>) {
    let mut free_ranges: Vec<FreeRange> = Vec::new();
    let mut files: Vec<File> = Vec::new();
    let mut i = disk.len() as i64 - 1;
    loop {
        let file_end = i;
        while i >= 0 && disk[i as usize].is_some() && disk[i as usize] == disk[file_end as usize] {
            i -= 1;
        }
        if file_end > i {
            let file_start = i + 1;
            files.push(File {
                start: file_start as usize,
                blocks: (file_end - file_start + 1) as usize,
                id: disk[file_end as usize].unwrap()
            });
        }

        if i < 0 {
            break;
        }
        if disk[i as usize].is_some() {
            continue;
        }

        let block_end = i;
        while i >= 0 && disk[i as usize].is_none() {
            i -= 1;
        }
        let block_start = i + 1;

        free_ranges.push(FreeRange {start: block_start as usize, blocks: (block_end - block_start + 1) as usize});
    }

    return (free_ranges, files);
}

#[test]
fn test_get_free_ranges() {
    let disk: Vec<Option<u64>> = vec![None, Some(0), Some(0), None, None, None, Some(1), Some(1), Some(1), None, None, None, Some(2)];
    let (ranges, files) = get_free_ranges_and_files(&disk);
    assert_eq!(ranges.len(), 3);
    assert_eq!(ranges[0].start, 9);
    assert_eq!(ranges[0].blocks, 3);
    assert_eq!(ranges[1].start, 3);
    assert_eq!(ranges[1].blocks, 3);
    assert_eq!(ranges[2].start, 0);
    assert_eq!(ranges[2].blocks, 1);
    assert_eq!(files.len(), 3);
    assert_eq!(files[0].start, 12);
    assert_eq!(files[0].blocks, 1);
    assert_eq!(files[0].id, 2)
}
