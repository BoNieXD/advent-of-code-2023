use aoc2023::{read_input, InputType};
use regex::Regex;
use timed::timed;

const DAY: u8 = 4;

fn main() {
    let input = read_input(DAY, InputType::Real).unwrap();

    println!("Part1: {}", part1(&input));
    println!("Part2: {}", part2(&input));
}

#[timed]
fn part1(input: &str) -> u32 {
    let regex_numbers = Regex::new(r"\d+").unwrap();
    let regex_line = Regex::new(r"((?:\d+\s*)+)\s+\|\s+((?:\d+\s*)+)\s").unwrap();
    regex_line
        .captures_iter(input)
        .map(|c| c.extract())
        .map(|(_, [winning, choosen])| {
            let winning = winning.split(' ').collect::<Vec<_>>();
            regex_numbers
                .find_iter(choosen)
                .map(|x| x.as_str())
                .filter(|x| winning.contains(x))
                .enumerate()
                .fold(0, |_, (index, _)| 1 << index)
        })
        .sum()
}

#[timed]
fn part2(input: &str) -> usize {
    let regex_numbers = Regex::new(r"\d+").unwrap();
    let regex_line = Regex::new(r"((?:\d+\s*)+)\s+\|\s+((?:\d+\s*)+)\s").unwrap();
    let parsed = regex_line.captures_iter(input).map(|c| c.extract());
    let mut coppied_cards = vec![1; input.lines().count()];
    for (index, (_, [winning, choosen])) in parsed.enumerate() {
        let winning = winning.split(' ').collect::<Vec<_>>();
        let matches = regex_numbers
            .find_iter(choosen)
            .map(|x| x.as_str())
            .filter(|x| winning.contains(x))
            .count();
        for i in index + 1..=index + matches {
            coppied_cards[i] += coppied_cards[index];
        }
    }
    coppied_cards.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_input(input_type: InputType) -> String {
        read_input(DAY, input_type).unwrap()
    }

    #[test]
    fn part1_test() {
        let expected = 13;
        let result = part1(&get_test_input(InputType::Test));
        assert_eq!(result, expected);
    }
    #[test]
    fn part2_test() {
        let expected = 30;
        let result = part2(&get_test_input(InputType::Test));
        assert_eq!(result, expected);
    }
}
