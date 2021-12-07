use std::fs;

fn read_file() -> Vec<u32> {
    let input = fs::read_to_string("input").unwrap();
    input.lines().map(|x| x.parse().unwrap()).collect()
}

fn first_solution(numbers: &[u32]) -> u64 {
    numbers.windows(2).filter(|x| x[1] > x[0]).count() as u64
}

fn second_solution(numbers: &[u32]) -> u64 {
    numbers.windows(4).filter(|x| x[3] > x[0]).count() as u64
}

fn main() {
    let numbers = read_file();
    let first_solution = first_solution(&numbers);
    let second_solution = second_solution(&numbers);
    assert_eq!(first_solution, 1462);
    assert_eq!(second_solution, 1497);
    println!("First solution: {}", first_solution);
    println!("Second solution: {}", second_solution);
}
