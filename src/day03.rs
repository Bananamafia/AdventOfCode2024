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
        let mul_values: Vec<(i32, i32)> = extract_precise_mul_values(input);

        let mut solution: i32 = 0;

        for mul_value_pair in mul_values {
            solution += mul_value_pair.0 * mul_value_pair.1;
        }

        solution
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

    fn extract_precise_mul_values(input: &str) -> Vec<(i32, i32)> {
        let split_by_dont = input.split("don't").collect::<Vec<&str>>();

        let mut mul_values: Vec<(i32, i32)> = Vec::new();
        for split_section in split_by_dont {
            let sub_sections: Vec<&str> = split_section.split("do").collect::<Vec<&str>>();

            if mul_values.is_empty() {
                let mut temp_mul_values = extract_mul_values(sub_sections[0]);
                mul_values.append(temp_mul_values.as_mut());
            }

            if sub_sections.len() >= 2 {
                for i in 1..sub_sections.len() {
                    let mut temp_mul_values = extract_mul_values(sub_sections[i]);
                    mul_values.append(temp_mul_values.as_mut());
                }
            }
        }

        mul_values
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        const SAMPLE_INPUT_1: &str =
            "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

        const SAMPLE_INPUT_2: &str =
            "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

        #[test]
        fn solve_part_1_test() {
            let expected: i32 = 161;

            let actual: i32 = solve_part_1(SAMPLE_INPUT_1);

            assert_eq!(expected, actual);
        }

        #[test]
        fn solve_part_2_test() {
            let expected: i32 = 48;

            let actual: i32 = solve_part_2(SAMPLE_INPUT_2);

            assert_eq!(expected, actual);
        }

        #[test]
        fn extract_mul_values_test() {
            let expected: Vec<(i32, i32)> = vec![(2, 4), (5, 5), (11, 8), (8, 5)];

            let actual: Vec<(i32, i32)> = extract_mul_values(SAMPLE_INPUT_1);

            assert_eq!(expected, actual);
        }

        #[test]
        fn extract_precise_mul_values_test() {
            let expected: Vec<(i32, i32)> = vec![(2, 4), (8, 5)];

            let actual: Vec<(i32, i32)> = extract_precise_mul_values(SAMPLE_INPUT_2);

            assert_eq!(expected, actual);
        }
    }
}
