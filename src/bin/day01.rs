use aoc2023::{read_input, InputType};
use timed::timed;

const DAY: u8 = 1;

fn main() {
    let input = read_input(DAY, InputType::Real).unwrap();

    println!("Part1: {}", part1(&input));
    println!("Part2: {}", part2(&input));
}

#[timed]
fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|x| {
            let tmp = x
                .chars()
                .filter(|&x| '1' <= x && x <= '9')
                .collect::<Vec<char>>();
            format!("{}{}", tmp.first().unwrap(), tmp.last().unwrap())
                .parse::<u32>()
                .unwrap()
        })
        .sum()
}

#[timed]
fn part2(input: &str) -> usize {
    input
        .lines()
        .map(|x| {
            let mut tmp = vec![0; x.len()];

            let characters = [
                "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", " ", "1",
                "2", "3", "4", "5", "6", "7", "8", "9",
            ];
            for (char_index, character) in characters.iter().enumerate() {
                if *character == " " {
                    continue;
                }
                let first_index = x.find(character);
                let last_index = x.rfind(character);
                match (first_index, last_index) {
                    (Some(i1), Some(i2)) => {
                        tmp[i1] = (char_index + 1) % 10;
                        tmp[i2] = (char_index + 1) % 10;
                    }
                    _ => {}
                }
            }

            let res = tmp.into_iter().filter(|&x| x > 0).collect::<Vec<usize>>();
            res.first().unwrap() * 10 + res.last().unwrap()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_input(input_type: InputType) -> String {
        read_input(DAY, input_type).unwrap()
    }

    #[test]
    fn part1_test() {
        let expected = 142;
        let result = part1(&get_test_input(InputType::Test));
        assert_eq!(result, expected);
    }
    #[test]
    fn part2_test1() {
        let expected = 142;
        let result = part2(&get_test_input(InputType::Test));
        assert_eq!(result, expected);
    }
    #[test]
    fn part2_test2() {
        let expected = 281;
        let result = part2(&get_test_input(InputType::Other("test2")));
        assert_eq!(result, expected);
    }
    #[test]
    fn part2_test3() {
        let expected = 54728;
        let result = part2(&get_test_input(InputType::Other("DP")));
        assert_eq!(result, expected);
    }
    #[test]
    fn part2_test4() {
        let expected = 54265;
        let result = part2(&get_test_input(InputType::Other("WJ")));
        assert_eq!(result, expected);
    }
}
