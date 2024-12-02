pub mod day01 {
    pub fn solve_part_1(input: &str) -> i32 {
        let mut values: (Vec<i32>, Vec<i32>) = get_values_from(input);

        let mut left_values = values.0;
        let mut right_values = values.1;
        left_values.sort();
        right_values.sort();

        let mut solution: i32 = 0;
        while !left_values.is_empty() {
            solution += (right_values.pop().expect("No value remaining.")
                - left_values.pop().expect("No value remaining.")).abs();
        }

        return solution;
    }

    fn get_values_from(input: &str) -> (Vec<i32>, Vec<i32>) {
        let mut left: Vec<i32> = Vec::new();
        let mut right: Vec<i32> = Vec::new();

        for line in input.lines() {
            let parts: Vec<&str> = line.split(' ').collect();

            let left_element: i32 = parts
                .first()
                .expect("No first element found.")
                .parse()
                .expect("First value is not of type int.");
            let right_element: i32 = parts
                .last()
                .expect("No last element found.")
                .parse()
                .expect("Second value is not of type int.");

            left.push(left_element);
            right.push(right_element);
        }

        return (left, right);
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        const SAMPLE_INPUT: &str = "3   4
4   3
2   5
1   3
3   9
3   3";

        #[test]
        fn part1() {
            let expected: i32 = 11;

            let actual: i32 = solve_part_1(SAMPLE_INPUT);

            assert_eq!(actual, expected);
        }

        #[test]
        fn get_values() {
            let expected_left = vec![3, 4, 2, 1, 3, 3];
            let expected_right = vec![4, 3, 5, 3, 9, 3];

            let expected: (Vec<i32>, Vec<i32>) = (expected_left, expected_right);

            let actual: (Vec<i32>, Vec<i32>) = get_values_from(SAMPLE_INPUT);

            assert_eq!(actual, expected);
        }
    }
}
