pub mod day05 {
    use std::collections::{HashMap, HashSet};
    use std::ops::Index;
    use std::str::Split;

    pub fn get_ordering_rules(input: &str) -> Vec<(i32, i32)> {
        let mut rules: Vec<(i32, i32)> = Vec::new();

        for line in input.lines() {
            let values: Vec<i32> = line.split('|').map(|v| v.parse().unwrap()).collect();
            rules.push((values[0], values[1]));
        }

        rules
    }

    pub fn get_clean_ordering_rules_ordered_by_last(
        rules: &Vec<(i32, i32)>,
    ) -> HashMap<i32, Vec<i32>> {
        let mut cleaned_rules: HashMap<i32, Vec<i32>> = HashMap::new();

        for rule in rules {
            cleaned_rules
                .entry(rule.1)
                .or_insert(Vec::new())
                .push(rule.0);
        }

        cleaned_rules
    }

    pub fn get_clean_ordering_rules_ordered_by_first(
        rules: &Vec<(i32, i32)>,
    ) -> HashMap<i32, Vec<i32>> {
        let mut cleaned_rules: HashMap<i32, Vec<i32>> = HashMap::new();

        for rule in rules {
            cleaned_rules
                .entry(rule.0)
                .or_insert(Vec::new())
                .push(rule.1);
        }

        cleaned_rules
    }

    pub fn get_update_numbers(input: &str) -> Vec<Vec<i32>> {
        let mut numbers: Vec<Vec<i32>> = Vec::new();

        for line in input.lines() {
            let values: Vec<i32> = line.split(',').map(|v| v.parse().unwrap()).collect();
            numbers.push(values);
        }

        numbers
    }

    pub fn is_update_in_order(update: &Vec<i32>, rules: &HashMap<i32, Vec<i32>>) -> bool {
        let mut order_breaking_numbers: HashSet<i32> = HashSet::new();

        for value in update {
            if order_breaking_numbers.contains(&value) {
                return false;
            }

            if rules.contains_key(&value) {
                let must_come_before: &Vec<i32> = rules.get(&value).unwrap();
                order_breaking_numbers.extend(must_come_before);
            }
        }

        true
    }

    pub fn solve_part_1(input: &str) -> i32 {
        let input_parts: Vec<&str> = input.split("\n\n").collect();
        let rules: Vec<(i32, i32)> = get_ordering_rules(input_parts[0]);
        let cleaned_rules: HashMap<i32, Vec<i32>> = get_clean_ordering_rules_ordered_by_last(&rules);
        let update_numbers: Vec<Vec<i32>> = get_update_numbers(input_parts[1]);

        let mut solution: i32 = 0;

        for update in update_numbers {
            if is_update_in_order(&update, &cleaned_rules) {
                let middle_index = update.len() / 2;
                solution += update[middle_index]
            }
        }

        solution
    }

    fn get_ordered_update(update: &Vec<i32>, rules: &HashMap<i32, Vec<i32>>) -> Vec<i32> {
        let mut ordered_update: Vec<i32> = Vec::new();

        for value in update.into_iter().rev() {

            if rules.contains_key(value){
                let rule = rules.get(value).unwrap();
                let first_index = ordered_update.iter().position(|v| rule.contains(v));

                ordered_update.insert(first_index.unwrap_or(ordered_update.len()), *value);
            }
            else {
                ordered_update.push(*value);
            }
        }

        ordered_update
    }

    pub fn solve_part_2(input: &str) -> i32 {
        let input_parts: Vec<&str> = input.split("\n\n").collect();
        let rules: Vec<(i32, i32)> = get_ordering_rules(input_parts[0]);
        let cleaned_rules_ordered_by_last: HashMap<i32, Vec<i32>> =
            get_clean_ordering_rules_ordered_by_last(&rules);
        let cleaned_rules_ordered_by_first: HashMap<i32, Vec<i32>> =
            get_clean_ordering_rules_ordered_by_first(&rules);

        let update_numbers: Vec<Vec<i32>> = get_update_numbers(input_parts[1]);

        let mut solution: i32 = 0;

        for update in update_numbers {
            if !is_update_in_order(&update, &cleaned_rules_ordered_by_last) {
                let ordered_update: Vec<i32> =
                    get_ordered_update(&update, &cleaned_rules_ordered_by_first);
                let middle_index = ordered_update.len() / 2;
                solution += ordered_update[middle_index]
            }
        }

        solution
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_get_ordering_rules() {
            let input: &str = "97|13
97|61
97|47
75|29";

            let expected: Vec<(i32, i32)> = vec![(97, 13), (97, 61), (97, 47), (75, 29)];

            let actual: Vec<(i32, i32)> = get_ordering_rules(input);

            assert_eq!(expected, actual);
        }

        #[test]
        fn test_get_cleaned_ordering_rules() {
            let input: &str = "97|13
22|13
97|47
75|29";

            let expected: HashMap<i32, Vec<i32>> =
                HashMap::from([(13, vec![97, 22]), (47, vec![97]), (29, vec![75])]);

            let actual: HashMap<i32, Vec<i32>> =
                get_clean_ordering_rules_ordered_by_last(&get_ordering_rules(input));

            assert_eq!(expected, actual);
        }

        #[test]
        fn test_get_update_numbers() {
            let input: &str = "75,47,61,53,29
97,61,53,29,13
75,29,13";

            let expected: Vec<Vec<i32>> = vec![
                vec![75, 47, 61, 53, 29],
                vec![97, 61, 53, 29, 13],
                vec![75, 29, 13],
            ];

            let actual: Vec<Vec<i32>> = get_update_numbers(input);

            assert_eq!(actual, expected);
        }

        #[test]
        fn test_ordered_update() {
            let rules: HashMap<i32, Vec<i32>> = HashMap::from([(3, vec![1, 2])]);
            let update: Vec<i32> = vec![1, 2, 3];

            let expected: bool = true;

            let actual: bool = is_update_in_order(&update, &rules);

            assert_eq!(expected, actual);
        }

        #[test]
        fn test_unordered_update() {
            let rules: HashMap<i32, Vec<i32>> = HashMap::from([(3, vec![1, 2])]);
            let update: Vec<i32> = vec![3, 2, 1];

            let expected: bool = false;

            let actual: bool = is_update_in_order(&update, &rules);

            assert_eq!(expected, actual);
        }

        #[test]
        fn test_get_ordered_update() {
            let rules: HashMap<i32, Vec<i32>> = HashMap::from([(1, vec![2, 3]), (2, vec![3])]);
            let update: Vec<i32> = vec![3, 2, 1];

            let expected: Vec<i32> = vec![1, 2, 3];

            let actual: Vec<i32> = get_ordered_update(&update, &rules);

            assert_eq!(expected, actual);
        }

        const SAMPLE_INPUT: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

        #[test]
        fn test_solve_part_1() {
            let expected: i32 = 143;

            let actual: i32 = solve_part_1(SAMPLE_INPUT);

            assert_eq!(expected, actual);
        }

        #[test]
        fn test_solve_part_2() {
            let expected: i32 = 123;

            let actual: i32 = solve_part_2(SAMPLE_INPUT);

            assert_eq!(expected, actual);
        }
    }
}
