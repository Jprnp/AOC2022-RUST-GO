use std::fs;

fn main() {
    let mut calories: Vec<usize> = vec![];
    let mut current: usize = 0;

    if let Ok(file) = fs::read_to_string("input") {
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
    }
    calories.push(current);
    calories.sort_by(|a, b| b.cmp(a));
    println!("{}", calories[0] + calories[1] + calories[2]);
}
