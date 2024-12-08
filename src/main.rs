use std::fs;
mod day04;

fn main() {
    let input = fs::read_to_string("inputs/day04.txt").expect("No file found");

    print!("Solution: {}", day04::day04::solve_part_1(&input));
}
