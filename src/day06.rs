pub mod day06 {
    use std::collections::HashSet;
    use crate::day06::day06::Direction::{Down, Left, Right, Up};

    #[derive(Debug, Hash, Eq, PartialEq)]
    struct Point(i32, i32);

    enum Direction {
        Up,
        Right,
        Down,
        Left,
    }

    pub fn solve_part_1(input: &str) -> i32 {
        let field_size: Point = get_field_size(input);

        let starting_point: Point = get_starting_point(input);
        let obstructions: Vec<Point> = get_obstructions(input);

        let mut visited_fields: HashSet<Point> = HashSet::new();
        let mut current_direction = Up;
        let mut current_position: Point = starting_point;

        while is_in_field(&current_position, &field_size) {
            visited_fields.insert(Point(current_position.0, current_position.1));
            let mut destination: Point = Point(current_position.0, current_position.1);

            match current_direction {
                Up => {
                    destination.1 -= 1;
                }
                Right => {
                    destination.0 += 1;
                }
                Down => {
                    destination.1 += 1;
                }
                Left => {
                    destination.0 -= 1;
                }
            }

            if obstructions.contains(&destination) {
                current_direction = get_next_direction(current_direction);
                continue;
            }

            current_position = destination;
        }

        visited_fields.len() as i32
    }

    fn get_starting_point(input: &str) -> Point {
        const STARTING_CHAR: char = '^';
        let mut starting_point: Point = Point(0, 0);

        for line in input.lines() {
            if !line.contains(STARTING_CHAR) {
                continue;
            }

            let x = line.chars().position(|c| c == STARTING_CHAR).unwrap();
            let y = input.lines().position(|l| l == line).unwrap();
            starting_point = Point(x as i32, y as i32);
        }

        starting_point
    }

    fn get_obstructions(input: &str) -> Vec<Point> {
        const OBSTRUCTION_CHAR: char = '#';

        let mut obstructions: Vec<Point> = Vec::new();

        let mut y: i32 = 0;
        for line in input.lines() {
            if !line.contains(OBSTRUCTION_CHAR) {
                y += 1;
                continue;
            }

            let mut x: i32 = 0;
            for char in line.chars() {
                if char == OBSTRUCTION_CHAR {
                    obstructions.push(Point(x, y));
                }
                x += 1;
            }
            y += 1;
        }

        obstructions
    }

    fn get_field_size(input: &str) -> Point {
        let y: i32 = (input.lines().count() - 1) as i32;
        let x: i32 = (input.lines().next().unwrap().chars().count() - 1) as i32;

        Point(x, y)
    }

    fn get_next_direction(direction: Direction) -> Direction {
        match direction {
            Up => Right,
            Right => Down,
            Down => Left,
            Left => Up,
        }
    }

    fn is_in_field(position: &Point, field_size: &Point) -> bool {
        position.0 >= 0
            && position.1 >= 0
            && position.0 <= field_size.0
            && position.1 <= field_size.1
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        const SAMPLE_INPUT: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

        #[test]
        fn test_get_starting_point() {
            let expected: Point = Point(4, 6);

            let actual: Point = get_starting_point(SAMPLE_INPUT);

            assert_eq!(expected, actual);
        }

        #[test]
        fn test_get_obstructions() {
            let expected: Vec<Point> = vec![
                Point(4, 0),
                Point(9, 1),
                Point(2, 3),
                Point(7, 4),
                Point(1, 6),
                Point(8, 7),
                Point(0, 8),
                Point(6, 9),
            ];

            let actual = get_obstructions(SAMPLE_INPUT);

            assert_eq!(expected, actual);
        }

        #[test]
        fn test_get_field_size() {
            let expected: Point = Point(9, 9);

            let actual: Point = get_field_size(SAMPLE_INPUT);

            assert_eq!(expected, actual);
        }

        #[test]
        fn test_solve_part_1() {
            let expected: i32 = 41;

            let actual: i32 = solve_part_1(SAMPLE_INPUT);

            assert_eq!(expected, actual);
        }
    }
}
