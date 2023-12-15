use std::collections::HashMap;

use aoc2023::{read_input, InputType};
use timed::timed;

const DAY: u8 = 12;

fn main() {
    let input = read_input(DAY, InputType::Real).unwrap();

    println!("Part1: {}", part1(&input));
    println!("Part2: {}", part2(&input));
}

fn get_possible_solutions(
    symbols: &Vec<char>,
    numbers: &Vec<usize>,
    mem: &mut HashMap<(usize, usize, usize), usize>,
    current_symbol: usize,
    current_number: usize,
    count: usize,
) -> usize {
    let key = (current_symbol, current_number, count);
    if mem.contains_key(&key) {
        return *mem.get(&key).unwrap();
    }
    if current_symbol == symbols.len() {
        return (current_number == numbers.len()
            || (current_number == numbers.len() - 1 && count == numbers[current_number]))
            as usize;
    }
    if current_number == numbers.len() {
        return (symbols[current_symbol..].iter().all(|&x| x != '#')) as usize;
    }
    let mut result = 0;
    if symbols[current_symbol] == '.' || symbols[current_symbol] == '?' {
        if count == 0 || count == numbers[current_number] {
            result += get_possible_solutions(
                symbols,
                numbers,
                mem,
                current_symbol + 1,
                current_number + (count != 0) as usize,
                0,
            )
        }
    }
    if symbols[current_symbol] == '#' || symbols[current_symbol] == '?' {
        if count < numbers[current_number] {
            result += get_possible_solutions(
                symbols,
                numbers,
                mem,
                current_symbol + 1,
                current_number,
                count + 1,
            )
        }
    }
    mem.insert(key, result);
    result
}

#[timed]
fn part1(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let line = line.split_once(' ').unwrap();
            let symbols = line.0.chars().collect::<Vec<_>>();
            let numbers = line
                .1
                .split(',')
                .flat_map(|x| x.parse::<usize>())
                .collect::<Vec<_>>();
            get_possible_solutions(&symbols, &numbers, &mut HashMap::new(), 0, 0, 0)
        })
        .sum()
}

#[timed]
fn part2(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let line = line.split_once(' ').unwrap();
            let mut symbols = format!("{}?", line.0).repeat(5).chars().collect::<Vec<_>>();
            symbols.pop();
            let numbers = format!("{},", line.1)
                .repeat(5)
                .split(',')
                .filter_map(|x| x.parse::<usize>().ok())
                .collect::<Vec<_>>();
            get_possible_solutions(&symbols, &numbers, &mut HashMap::new(), 0, 0, 0)
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
        let expected = 21;
        let result = part1(&get_test_input(InputType::Test));
        assert_eq!(result, expected);
    }
    #[test]
    fn part2_test() {
        let expected = 525152;
        let result = part2(&get_test_input(InputType::Test));
        assert_eq!(result, expected);
    }
}
