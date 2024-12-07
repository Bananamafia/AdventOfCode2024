use std::fs;
mod day03;

fn main() {
    let input = fs::read_to_string("inputs/day03.txt").expect("No file found");

    print!("Solution: {}", day03::day03::solve_part_2(&input));
}
