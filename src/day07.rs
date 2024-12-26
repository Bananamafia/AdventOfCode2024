pub mod day07 {
    use std::collections::{HashSet, VecDeque};

    pub fn solve_part_1(input: &str) -> i64 {
        let equations: Vec<(i64, Vec<i64>)> = get_equations(input);

        let mut solution: i64 = 0;

        for equation in equations {
            if has_correct_equations(equation.0, equation.1) {
                solution += equation.0;
            }
        }

        solution
    }

    pub fn solve_part_2(input: &str) -> i64 {
        let equations: Vec<(i64, Vec<i64>)> = get_equations(input);

        let mut solution: i64 = 0;

        for equation in equations {
            let mut subset: VecDeque<i64> = VecDeque::from(equation.1);
            let mut possible_solutions =
                get_possible_solutions_part2(HashSet::from_iter((vec![subset.pop_front().unwrap()]).iter().cloned()), Option::None);
            for value in subset {
                possible_solutions = get_possible_solutions_part2(possible_solutions, Option::from(value));
            }

            if possible_solutions.contains(&equation.0){
                solution += equation.0;
            }
        }

        solution
    }

    fn get_possible_solutions_part2(values: HashSet<i64>, combiner: Option<i64>) -> HashSet<i64> {
        if combiner.is_none() {
            return values;
        }

        let mut possible_values: HashSet<i64> = HashSet::new();

        for value in values {
            possible_values.insert(value + combiner.unwrap());
            possible_values.insert(value * combiner.unwrap());
            possible_values.insert(format!("{}{}", value, combiner.unwrap()).parse::<i64>().unwrap());
        }

        possible_values
    }

    pub fn has_correct_equations(expected: i64, values: Vec<i64>) -> bool {
        let operation_count = values.len() - 1;
        let mut binary_operation = (1 << operation_count) - 1;

        while binary_operation >= 0 {
            let mut values: VecDeque<i64> = VecDeque::from(values.clone());
            let mut calculation: i64 = values.pop_front().unwrap();

            for char in format!("{:0n$b}", binary_operation, n = operation_count).chars() {
                if char == '1' {
                    calculation *= values.pop_front().unwrap();
                } else {
                    calculation += values.pop_front().unwrap();
                }
            }

            binary_operation -= 1;

            if calculation == expected {
                return true;
            }
        }

        false
    }

    fn get_equations(input: &str) -> Vec<(i64, Vec<i64>)> {
        let mut equations: Vec<(i64, Vec<i64>)> = Vec::new();

        for line in input.lines() {
            let split_line: Vec<&str> = line.split(':').collect();

            equations.push((
                split_line[0].parse().unwrap(),
                split_line[1]
                    .split_whitespace()
                    .map(|v| v.parse().unwrap())
                    .collect(),
            ))
        }

        equations
    }

    #[cfg(test)]
    mod tests {

        use super::*;

        const SAMPLE_INPUT: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

        #[test]
        fn test_get_equation() {
            let expected: Vec<(i64, Vec<i64>)> = vec![
                (190, vec![10, 19]),
                (3267, vec![81, 40, 27]),
                (83, vec![17, 5]),
                (156, vec![15, 6]),
                (7290, vec![6, 8, 6, 15]),
                (161011, vec![16, 10, 13]),
                (192, vec![17, 8, 14]),
                (21037, vec![9, 7, 18, 13]),
                (292, vec![11, 6, 16, 20]),
            ];

            let actual: Vec<(i64, Vec<i64>)> = get_equations(SAMPLE_INPUT);

            assert_eq!(expected, actual);
        }

        #[test]
        fn test_solve_part_1() {
            let expected: i64 = 3749;

            let actual: i64 = solve_part_1(SAMPLE_INPUT);

            assert_eq!(expected, actual);
        }

        #[test]
        fn test_solve_part2() {
            let expected: i64 = 11387;

            let actual: i64 = solve_part_2(SAMPLE_INPUT);

            assert_eq!(expected, actual);
        }
    }
}
