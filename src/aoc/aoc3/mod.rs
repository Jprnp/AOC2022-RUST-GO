use std::fs;

pub mod pt2;

pub fn rucksack_reorganization() {
    let mut sum: usize = 0;
    if let Ok(file) = fs::read_to_string("src/aoc/aoc3/input") {
        file.split('\n').for_each(|line| {
            if line.trim().len() > 0 {
                sum += find_common_item_value(line);
            }
        });
    } else {
        panic!("File not found")
    }

    println!("AOC3 - PT1: {}", sum);
}

fn find_common_item_value(items: &str) -> usize {
    let middle_idx = items.len() / 2;
    for i in 0..middle_idx {
        let x = items.chars().nth(i).unwrap();
        for j in middle_idx..items.len() {
            let y = items.chars().nth(j).unwrap();
            if x == y {
                return item_value(x);
            }
        }
    }
    return 0;
}

fn item_value(item: char) -> usize {
    let ascii_val = item as u32;
    if item.is_uppercase() {
        return (ascii_val - 38) as usize;
    }
    return (ascii_val - 96) as usize;
}
