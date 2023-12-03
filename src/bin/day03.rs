use aoc2023::{read_input, InputType};
use timed::timed;

const DAY: u8 = 3;

fn main() {
    let input = read_input(DAY, InputType::Real).unwrap();

    println!("Part1 v1: {}", part1_v1(&input));
    println!("Part1 v2: {}", part1_v2(&input));
    println!("Part2: {}", part2(&input));
}

#[timed]
fn part1_v1(input: &str) -> u32 {
    let lines = input.lines().collect::<Vec<&str>>();
    let mut result = 0;
    for (line_index, line) in lines.iter().enumerate() {
        let mut found_symbol = false;
        let mut number = 0;
        for (char_index, character) in line.chars().enumerate() {
            if '0' <= character && character <= '9' {
                number = (number * 10) + character.to_digit(10).unwrap();
                for y_offset in -1..=1 as i32 {
                    let y = (line_index as i32) + y_offset;
                    if y < 0 || lines.len() as i32 - 1 < y {
                        continue;
                    }
                    for x_offset in -1..=1 as i32 {
                        let x = (char_index as i32) + x_offset;
                        if x < 0 || line.len() as i32 - 1 < x {
                            continue;
                        }
                        let potential_symbol =
                            lines[y as usize].chars().collect::<Vec<char>>()[x as usize];
                        if potential_symbol != '.'
                            && (potential_symbol < '0' || potential_symbol > '9')
                        {
                            found_symbol = true;
                        }
                    }
                }
                continue;
            } else if number != 0 && found_symbol {
                result += number;
            }
            number = 0;
            found_symbol = false;
        }
        if number != 0 && found_symbol {
            result += number;
        }
    }
    result
}

fn check_sign_near(
    lines: &Vec<&str>,
    current_line_index: usize,
    number_start_index: usize,
    num_len: usize,
) -> bool {
    let start_line_index: usize;
    let end_line_index: usize;
    let start_char_index: usize;
    let end_char_index: usize;

    if number_start_index == 0 {
        start_char_index = number_start_index;
    } else {
        start_char_index = number_start_index - 1;
    }
    if number_start_index + num_len == lines[0].len() {
        end_char_index = number_start_index + num_len - 1;
    } else {
        end_char_index = number_start_index + num_len;
    }
    if current_line_index == 0 {
        start_line_index = current_line_index;
    } else {
        start_line_index = current_line_index - 1;
    }
    if current_line_index == lines.len() - 1 {
        end_line_index = current_line_index;
    } else {
        end_line_index = current_line_index + 1;
    }

    let lines = lines[start_line_index..=end_line_index].to_vec();
    for line in lines.clone() {
        let characters =
            line.chars().collect::<Vec<char>>()[start_char_index..=end_char_index].to_vec();
        for character in characters {
            if character != '.' && (character < '0' || '9' < character) {
                return true;
            }
        }
    }
    false
}
#[timed]
fn part1_v2(input: &str) -> u32 {
    let lines = input.lines().collect::<Vec<&str>>();
    let mut result = 0;
    for (line_index, line) in lines.iter().enumerate() {
        let mut number = 0;
        let mut number_len = 0;
        let mut number_start_index = 0;
        for (char_index, character) in line.chars().enumerate() {
            if '0' <= character && character <= '9' {
                if number == 0 {
                    number_start_index = char_index;
                }
                number = (number * 10) + character.to_digit(10).unwrap();
                number_len += 1;
                continue;
            } else if number != 0
                && check_sign_near(&lines, line_index, number_start_index, number_len)
            {
                result += number;
            }
            number = 0;
            number_len = 0;
        }
        if number != 0 && check_sign_near(&lines, line_index, number_start_index, number_len) {
            result += number;
        }
    }
    result
}

#[timed]
fn part2(input: &str) -> u32 {
    let lines = input.lines().collect::<Vec<&str>>();
    let mut result = 0;
    for (line_index, line) in lines.iter().enumerate() {
        for (char_index, character) in line.chars().enumerate() {
            if character == '*' {
                let mut numbers = vec![];
                for y_offset in -1..=1 as i32 {
                    let y = (line_index as i32) + y_offset;
                    if y < 0 || lines.len() as i32 - 1 < y {
                        continue;
                    }
                    let mut selected_line = lines[y as usize].chars().collect::<Vec<char>>();
                    for x_offset in -1..=1 as i32 {
                        let x = (char_index as i32) + x_offset;
                        if x < 0 || line.len() as i32 - 1 < x {
                            continue;
                        }
                        let potential_number = selected_line[x as usize];
                        if '0' <= potential_number && potential_number <= '9' {
                            let mut number_start_index = x;
                            let mut number_end_index = x;
                            while number_start_index > 0
                                && '0' <= selected_line[number_start_index as usize - 1]
                                && selected_line[number_start_index as usize - 1] <= '9'
                            {
                                number_start_index -= 1
                            }
                            while number_end_index < selected_line.len() as i32
                                && '0' <= selected_line[number_end_index as usize]
                                && selected_line[number_end_index as usize] <= '9'
                            {
                                number_end_index += 1
                            }
                            let number_string = (&selected_line
                                [number_start_index as usize..number_end_index as usize])
                                .into_iter()
                                .collect::<String>();
                            selected_line = selected_line
                                .iter()
                                .collect::<String>()
                                .replace(&number_string, &".".repeat(number_string.len()))
                                .chars()
                                .collect::<Vec<char>>();
                            let number = number_string.parse::<u32>().unwrap();
                            numbers.push(number);
                        }
                    }
                }
                if numbers.len() == 2 {
                    result += numbers.iter().fold(1, |acc, x| acc * x);
                }
            }
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_input(input_type: InputType) -> String {
        read_input(DAY, input_type).unwrap()
    }

    #[test]
    fn part1_v1_test() {
        let expected = 4361;
        let result = part1_v1(&get_test_input(InputType::Test));
        assert_eq!(result, expected);
    }
    #[test]
    fn part1_v2_test() {
        let expected = 4361;
        let result = part1_v2(&get_test_input(InputType::Test));
        assert_eq!(result, expected);
    }
    #[test]
    fn part1_v1_test_kk() {
        let expected = 520135;
        let result = part1_v1(&get_test_input(InputType::Other("KK")));
        assert_eq!(result, expected);
    }
    #[test]
    fn part1_v2_test_kk() {
        let expected = 520135;
        let result = part1_v2(&get_test_input(InputType::Other("KK")));
        assert_eq!(result, expected);
    }
    #[test]
    fn part1_v1_test_wj() {
        let expected = 556057;
        let result = part1_v1(&get_test_input(InputType::Other("WJ")));
        assert_eq!(result, expected);
    }
    #[test]
    fn part1_v2_test_wj() {
        let expected = 556057;
        let result = part1_v2(&get_test_input(InputType::Other("WJ")));
        assert_eq!(result, expected);
    }
    #[test]
    fn part2_test() {
        let expected = 467835;
        let result = part2(&get_test_input(InputType::Test));
        assert_eq!(result, expected);
    }
    #[test]
    fn part2_test_kk() {
        let expected = 72514855;
        let result = part2(&get_test_input(InputType::Other("KK")));
        assert_eq!(result, expected);
    }
    #[test]
    fn part2_test_wj() {
        let expected = 82824352;
        let result = part2(&get_test_input(InputType::Other("WJ")));
        assert_eq!(result, expected);
    }
}
