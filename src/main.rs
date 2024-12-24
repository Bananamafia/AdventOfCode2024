use std::fs;
mod day07;

fn main() {
    let input = fs::read_to_string("inputs/day07.txt").expect("No file found");

    print!("Solution: {}", day07::day07::solve_part_1(&input));
}
