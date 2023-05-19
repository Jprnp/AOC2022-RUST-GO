use std::fs;
use crate::aoc::aoc3::item_value;

pub fn rucksack_reorganization2() {
    let mut sum: usize = 0;
    if let Ok(file) = fs::read_to_string("src/aoc/aoc3/input") {
        let lines: Vec<&str> = file.split('\n').collect();
        for chunk in lines.chunks(3) {
            if let Some(dup) = find_badge_in_chunk(chunk) {
                sum += item_value(dup);
            }
        }
    } else {
        panic!("File not found")
    }

    println!("AOC3 - PT2: {}", sum);
}

fn find_badge_in_chunk(chunk: &[&str]) -> Option<char> {
    for i in 0..chunk[0].len() {
        let elf1_itm = chunk[0].chars().nth(i).unwrap();
        for u in 0..chunk[1].len() {
            let elf2_itm = chunk[1].chars().nth(u).unwrap();
            if elf1_itm == elf2_itm {
                for j in 0..chunk[2].len() {
                    if elf2_itm == chunk[2].chars().nth(j).unwrap() {
                        return Some(elf2_itm)
                    }
                }
            }
        }
    }
    return None;
}
