use std::fs;
mod day01;

fn main() {
    let input = fs::read_to_string("inputs/day01.txt").expect("No file found");

    print!("Solution: {}", day01::day01::solve_part_2(&input));
}
