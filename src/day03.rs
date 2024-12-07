pub mod day03 {
    use regex::Regex;

    pub fn solve_part_1(input: &str) -> i32 {
        let mul_values: Vec<(i32, i32)> = extract_mul_values(input);

        let mut solution: i32 = 0;

        for mul_value_pair in mul_values {
            solution += mul_value_pair.0 * mul_value_pair.1;
        }

        solution
    }

    pub fn solve_part_2(input: &str) -> i32 {
        unimplemented!()
    }

    fn extract_mul_values(input: &str) -> Vec<(i32, i32)> {
        const REGEX_STRING: &str = "mul\\((\\d*),(\\d*)\\)";
        let regex: Regex = Regex::new(REGEX_STRING).unwrap();

        let mul_values: Vec<(i32, i32)> = regex
            .captures_iter(input)
            .map(|values| {
                let (_, [left, right]) = values.extract();
                (left.parse::<i32>().unwrap(), right.parse::<i32>().unwrap())
            })
            .collect();

        mul_values
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        const SAMPLE_INPUT: &str =
            "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

        #[test]
        fn solve_part_1_test() {
            let expected: i32 = 161;

            let actual: i32 = solve_part_1(SAMPLE_INPUT);

            assert_eq!(expected, actual);
        }

        #[test]
        fn solve_part_2_test() {}

        #[test]
        fn extract_mul_values_test() {
            let expected: Vec<(i32, i32)> = vec![(2, 4), (5, 5), (11, 8), (8, 5)];

            let actual: Vec<(i32, i32)> = extract_mul_values(SAMPLE_INPUT);

            assert_eq!(expected, actual);
        }
    }
}
