use std::fs;

fn read_file() -> String {
    fs::read_to_string("input").expect("Error reading the file")
}

fn main() {
    let input = read_file();
    let lines: Vec<&str> = input.split('\n').collect();
    let mut previous_value: u64 = lines[0].parse().unwrap();
    let mut increment_count = 0;
    for line in lines {
        if !line.is_empty() {
            let current: u64 = line.parse().unwrap();
            if current > previous_value {
                increment_count = increment_count + 1;
            }
            previous_value = current;
        }
    }
    println!("First solution: {}", increment_count);
}
