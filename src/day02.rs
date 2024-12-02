pub mod day02 {
    pub fn solve_part_1(input: &str) -> i32 {
        let levels = get_values(input);

        let mut solution = 0;

        for level in levels {
            if is_row(&level) && are_steps_smaller_3(&level){
                solution += 1;
            }
        }

        solution
    }

    pub fn solve_part_2(input: &str) -> i32 {
        unimplemented!();
    }

    fn get_values(input: &str) -> Vec<Vec<i32>> {
        let mut output: Vec<Vec<i32>> = Vec::new();

        for line in input.lines() {
            let split_line: Vec<&str> = line.split_whitespace().collect::<Vec<&str>>();
            let values_of_line: Vec<i32> = split_line
                .iter()
                .map(|v| v.parse::<i32>().unwrap())
                .collect();
            output.push(values_of_line);
        }

        output
    }

    fn is_row(levels: &Vec<i32>) -> bool {
        is_increasing(&levels) || is_decreasing(&levels)
    }

    fn is_increasing(levels: &Vec<i32>) -> bool {
        for i in 0..levels.len() - 1 {
            if levels[i] <= levels[i + 1] {
                return false;
            }
        }

        true
    }

    fn is_decreasing(levels: &Vec<i32>) -> bool {
        for i in 0..levels.len() - 1 {
            if levels[i] >= levels[i + 1] {
                return false;
            }
        }

        true
    }

    fn are_steps_smaller_3(levels: &Vec<i32>) -> bool {
        for i in 0..levels.len() - 1 {
            if (levels[i] - levels[i + 1]).abs() > 3 {
                return false;
            }
        }

        true
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        const SAMPLE_INPUT: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

        #[test]
        fn get_values_test() {
            let expected: Vec<Vec<i32>> = vec![
                vec![7, 6, 4, 2, 1],
                vec![1, 2, 7, 8, 9],
                vec![9, 7, 6, 2, 1],
                vec![1, 3, 2, 4, 5],
                vec![8, 6, 4, 4, 1],
                vec![1, 3, 6, 7, 9],
            ];

            let actual: Vec<Vec<i32>> = get_values(SAMPLE_INPUT);

            assert_eq!(actual, expected);
        }

        #[test]
        fn solve_part_1_test() {
            let expected: i32 = 2;

            let actual: i32 = solve_part_1(SAMPLE_INPUT);

            assert_eq!(actual, expected);
        }
    }
}
