use std::fs;

pub fn aoc1() {
    let mut calories: Vec<usize> = vec![];
    let mut current: usize = 0;

    if let Ok(file) = fs::read_to_string("src/aoc1/inputz") {
        file.split('\n').for_each(|line| {
            match line.parse::<usize>() {
                Ok(value) => {
                    current += value;
                }
                Err(_) => {
                    calories.push(current);
                    current = 0;
                }
            }
        });
    } else {
        panic!("File not found")
    }

    calories.push(current);
    calories.sort_by(|a, b| b.cmp(a));
    let result: usize = calories.iter().take(3).sum();
    println!("{}", result);
}
