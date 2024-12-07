use std::fs;
mod day02;

fn main() {
    let input = fs::read_to_string("inputs/day02.txt").expect("No file found");

    print!("Solution: {}", day02::day02::solve_part_2(&input));
}
