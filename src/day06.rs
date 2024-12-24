pub mod day06 {
    use crate::day06::day06::Direction::{Down, Left, Right, Up};
    use std::collections::{HashSet, VecDeque};

    #[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
    struct Point(i32, i32);

    #[derive(Copy, Clone, Eq, PartialEq)]
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
            let destination: Point = get_destination(&current_position, &current_direction);

            if obstructions.contains(&destination) {
                current_direction = get_next_direction(current_direction);
                continue;
            }

            current_position = destination;
        }

        visited_fields.len() as i32
    }

    pub fn solve_part_2(input: &str) -> i32 {
        let field_size: Point = get_field_size(input);

        let starting_point: Point = get_starting_point(input);
        let obstructions: Vec<Point> = get_obstructions(input);

        let mut current_direction = Up;
        let mut current_position: Point = starting_point;

        let mut loop_obstructions: HashSet<Point> = HashSet::new();
        let mut checked_loop_obstructions: HashSet<Point> = HashSet::new();

        while is_in_field(&current_position, &field_size) {
            let destination: Point = get_destination(&current_position, &current_direction);

            if !checked_loop_obstructions.contains(&destination) && does_create_loop(
                &current_position,
                &current_direction,
                &obstructions,
                &field_size,
            ) {
                loop_obstructions.insert(destination);
            }

            checked_loop_obstructions.insert(destination);


            if obstructions.contains(&destination) {
                current_direction = get_next_direction(current_direction);
                continue;
            }

            current_position = destination;
        }

        loop_obstructions.len() as i32
    }

    fn does_create_loop(
        current_position: &Point,
        current_direction: &Direction,
        obstructions: &Vec<Point>,
        field_size: &Point,
    ) -> bool {
        let setup_obstruction = get_destination(current_position, current_direction);

        let mut direction: Direction = current_direction.clone();
        let mut position: Point = current_position.clone();

        let mut obstructions_met: Vec<(Point, Direction)> = Vec::new();

        while is_in_field(&position, field_size) {
            let destination: Point = get_destination(&position, &direction);

            if obstructions.contains(&destination) || destination == setup_obstruction {
                if obstructions_met.contains(&(destination, direction)) {
                    return true;
                }

                obstructions_met.push((destination, direction));
                direction = get_next_direction(direction);
                continue;
            }

            position = destination;
        }

        false
    }

    fn get_destination(current_position: &Point, direction: &Direction) -> Point {
        let mut destination: Point = Point(current_position.0, current_position.1);

        match direction {
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

        destination
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

        #[test]
        fn test_solve_part_2() {
            let expected: i32 = 6;

            let actual: i32 = solve_part_2(SAMPLE_INPUT);

            assert_eq!(expected, actual);
        }
    }
}
