pub mod day04 {
    use std::collections::hash_map::Values;

    pub fn solve_part_1(input: &str) -> i32 {
        let values = get_values(input);

        let mut solution: i32 = 0;

        for y in 0..values.len() {
            for x in 0..values[y].len() {
                if values[y][x] != 'X' {
                    continue;
                }

                if check_horizontal(x, y, &values) {
                    solution += 1;
                }

                if check_horizontal_reverse(x, y, &values) {
                    solution += 1;
                }

                if check_vertical(x, y, &values) {
                    solution += 1;
                }

                if check_vertical_reverse(x, y, &values) {
                    solution += 1;
                }

                if check_diagonal_left_to_right_top_to_bottom(x, y, &values) {
                    solution += 1;
                }

                if check_diagonal_left_to_right_bottom_to_top(x, y, &values) {
                    solution += 1;
                }

                if check_diagonal_right_to_left_top_to_bottom(x, y, &values) {
                    solution += 1;
                }

                if check_diagonal_right_to_left_bottom_to_top(x, y, &values) {
                    solution += 1;
                }
            }
        }

        solution
    }

    pub fn solve_part_2(input: &str) -> i32 {
        let values: Vec<Vec<char>> = get_values(input);

        let mut solution: i32 = 0;

        for y in 1..values.len() - 1 {
            for x in 1..values[y].len() - 1 {
                let current_char = values[y][x];

                if current_char != 'A' {
                    continue;
                }

                let neighbours = get_neighbours(x, y, &values);

                if !has_more_than_one_of_m_and_s(&neighbours) {
                    continue;
                }

                if has_more_than_one_of_m_and_s_flat(&get_corners_flat(&neighbours)) && diagonal_are_not_the_same(&neighbours) {
                    solution += 1;
                }

                // if has_more_than_one_of_m_and_s_flat(&get_borders_flat(&neighbours)) {
                //     solution += 1;
                // }
            }
        }

        solution
    }

    fn get_values(input: &str) -> Vec<Vec<char>> {
        input.lines().map(|l| l.chars().collect()).collect()
    }

    fn is_next_char_matching(current: &char, next: &char) -> bool {
        match current {
            'X' => next == &'M',
            'M' => next == &'A',
            'A' => next == &'S',
            'S' => panic!("reached last char"),
            _ => panic!("not supported char"),
        }
    }

    fn check_horizontal(x: usize, y: usize, values: &Vec<Vec<char>>) -> bool {
        let mut current_char = values[y][x];

        if current_char != 'X' {
            return false;
        }

        if values[y].len() <= x + 3 {
            return false;
        }

        for i in 1..4 {
            let next_char = values[y][x + i];
            if !is_next_char_matching(&current_char, &next_char) {
                return false;
            }
            current_char = next_char;
        }

        true
    }

    fn check_horizontal_reverse(x: usize, y: usize, values: &Vec<Vec<char>>) -> bool {
        let mut current_char = values[y][x];

        if current_char != 'X' {
            return false;
        }

        if x < 3 {
            return false;
        }

        for i in 1..4 {
            let next_char = values[y][x - i];
            if !is_next_char_matching(&current_char, &next_char) {
                return false;
            }
            current_char = next_char;
        }

        true
    }

    fn check_vertical(x: usize, y: usize, values: &Vec<Vec<char>>) -> bool {
        let mut current_char = values[y][x];

        if current_char != 'X' {
            return false;
        }

        if values.len() <= y + 3 {
            return false;
        }

        for i in 1..4 {
            let next_char = values[y + i][x];
            if !is_next_char_matching(&current_char, &next_char) {
                return false;
            }
            current_char = next_char;
        }

        true
    }

    fn check_vertical_reverse(x: usize, y: usize, values: &Vec<Vec<char>>) -> bool {
        let mut current_char = values[y][x];

        if current_char != 'X' {
            return false;
        }

        if y < 3 {
            return false;
        }

        for i in 1..4 {
            let next_char = values[y - i][x];
            if !is_next_char_matching(&current_char, &next_char) {
                return false;
            }
            current_char = next_char;
        }

        true
    }

    fn check_diagonal_left_to_right_top_to_bottom(
        x: usize,
        y: usize,
        values: &Vec<Vec<char>>,
    ) -> bool {
        let mut current_char = values[y][x];

        if current_char != 'X' {
            return false;
        }

        if values[y].len() <= x + 3 || values.len() <= y + 3 {
            return false;
        }

        for i in 1..4 {
            let next_char = values[y + i][x + i];
            if !is_next_char_matching(&current_char, &next_char) {
                return false;
            }
            current_char = next_char;
        }

        true
    }

    fn check_diagonal_left_to_right_bottom_to_top(
        x: usize,
        y: usize,
        values: &Vec<Vec<char>>,
    ) -> bool {
        let mut current_char = values[y][x];

        if current_char != 'X' {
            return false;
        }

        if x < 3 || y < 3 {
            return false;
        }

        for i in 1..4 {
            let next_char = values[y - i][x - i];
            if !is_next_char_matching(&current_char, &next_char) {
                return false;
            }
            current_char = next_char;
        }

        true
    }

    fn check_diagonal_right_to_left_top_to_bottom(
        x: usize,
        y: usize,
        values: &Vec<Vec<char>>,
    ) -> bool {
        let mut current_char = values[y][x];

        if current_char != 'X' {
            return false;
        }

        if x < 3 || values.len() <= y + 3 {
            return false;
        }

        for i in 1..4 {
            let next_char = values[y + i][x - i];
            if !is_next_char_matching(&current_char, &next_char) {
                return false;
            }
            current_char = next_char;
        }

        true
    }

    fn check_diagonal_right_to_left_bottom_to_top(
        x: usize,
        y: usize,
        values: &Vec<Vec<char>>,
    ) -> bool {
        let mut current_char = values[y][x];

        if current_char != 'X' {
            return false;
        }

        if values[y].len() <= x + 3 || y < 3 {
            return false;
        }

        for i in 1..4 {
            let next_char = values[y - i][x + i];
            if !is_next_char_matching(&current_char, &next_char) {
                return false;
            }
            current_char = next_char;
        }

        true
    }

    fn get_neighbours(x: usize, y: usize, values: &Vec<Vec<char>>) -> Vec<Vec<char>> {
        if x < 1 || y < 1 || y > values.len() - 2 || x > values[0].len() - 2 {
            panic!("Value is on boarder. Couldn't get all neighbours");
        }

        vec![
            vec![values[y - 1][x - 1], values[y - 1][x], values[y - 1][x + 1]],
            vec![values[y][x - 1], values[y][x], values[y][x + 1]],
            vec![values[y + 1][x - 1], values[y + 1][x], values[y + 1][x + 1]],
        ]
    }

    fn get_corners_flat(values: &Vec<Vec<char>>) -> Vec<char> {
        vec![values[0][0], values[0][2], values[2][0], values[2][2]]
    }

    fn get_borders_flat(values: &Vec<Vec<char>>) -> Vec<char> {
        vec![values[0][1], values[1][2], values[2][1], values[1][0]]
    }

    fn has_more_than_one_of_m_and_s(values: &Vec<Vec<char>>) -> bool {
        let flat: Vec<char> = values.clone().into_iter().flat_map(|v: Vec<char>| v).collect::<Vec<char>>();
        has_more_than_one_of_m_and_s_flat(&flat)
    }

    fn has_more_than_one_of_m_and_s_flat(values: &Vec<char>) -> bool {
        values.iter().filter(|&v| v == &'M').count() > 1
            && values.iter().filter(|&v| v == &'S').count() > 1
    }

    fn diagonal_are_not_the_same(values: &Vec<Vec<char>>) -> bool {
        values[0][0] != values[2][2] && values[0][2] != values[2][0]
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        const SAMPLE_INPUT: &str = "..........
MMMMMMMMMM
.AAAAAAAA.
SSSSSSSSSS
..........
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

        #[test]
        fn solve_part_1_test() {
            let expected: i32 = 18;

            let actual: i32 = solve_part_1(SAMPLE_INPUT);

            assert_eq!(expected, actual);
        }

        #[test]
        fn solve_part_2_test() {
            let expected: i32 = 17;

            let actual: i32 = solve_part_2(SAMPLE_INPUT);

            assert_eq!(expected, actual);
        }
    }
}
