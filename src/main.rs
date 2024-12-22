use std::fs;
mod day06;

fn main() {
    let input = fs::read_to_string("inputs/day06.txt").expect("No file found");

    print!("Solution: {}", day06::day06::solve_part_1(&input));
}
