pub mod day04 {

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
        unimplemented!();
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

    fn check_diagonal_left_to_right_top_to_bottom(x: usize, y: usize, values: &Vec<Vec<char>>) -> bool {
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

    fn check_diagonal_left_to_right_bottom_to_top(x: usize, y: usize, values: &Vec<Vec<char>>) -> bool {
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

    fn check_diagonal_right_to_left_top_to_bottom(x: usize, y: usize, values: &Vec<Vec<char>>) -> bool {
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

    fn check_diagonal_right_to_left_bottom_to_top(x: usize, y: usize, values: &Vec<Vec<char>>) -> bool {
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

    #[cfg(test)]
    mod tests {
        use super::*;

        const SAMPLE_INPUT: &str = "MMMSXXMASM
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
        fn solve_part_2_test() {}
    }
}
