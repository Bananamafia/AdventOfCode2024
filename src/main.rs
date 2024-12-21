use std::fs;
mod day05;

fn main() {
    let input = fs::read_to_string("inputs/day05.txt").expect("No file found");

    print!("Solution: {}", day05::day05::solve_part_1(&input));
}
