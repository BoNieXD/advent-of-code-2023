use aoc2023::{read_input, InputType};
use itertools::Itertools;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use timed::timed;

const DAY: u8 = 12;

fn main() {
    let input = read_input(DAY, InputType::Real).unwrap();

    println!("Part1: {}", part1(&input));
}

fn parse(input: &str) -> Vec<(Vec<char>, Vec<usize>)> {
    input
        .lines()
        .map(|line| {
            let line = line.split_once(' ').unwrap();
            (
                line.0.chars().collect::<Vec<_>>(),
                line.1
                    .split(',')
                    .flat_map(|x| x.parse::<usize>())
                    .collect::<Vec<_>>(),
            )
        })
        .collect::<Vec<_>>()
}

fn check_if_possible(solution: &Vec<char>, numbers: &Vec<usize>) -> bool {
    let mut groups = vec![];
    let mut last_dot = true;
    for i in 0..solution.len() {
        if solution[i] == '.' {
            last_dot = true;
            continue;
        } else {
            if last_dot {
                groups.push(0);
            }
            *groups.last_mut().unwrap() += 1;
            last_dot = false;
        }
    }

    if groups.len() != numbers.len() {
        return false;
    }

    for i in 0..groups.len() {
        if groups[i] != numbers[i] {
            return false;
        }
    }
    true
}

fn generate_possible_solutions(symbols: &Vec<char>, numbers: &Vec<usize>) -> usize {
    if symbols.into_iter().filter(|&&x| x != '.').count() < numbers.into_iter().sum() {
        return 0;
    }
    if symbols.iter().all(|&x| x != '?') {
        if check_if_possible(&symbols, numbers) {
            return 1;
        }
        return 0;
    }
    let mut result = 0;
    let mut symbols = symbols.clone();
    let (index, _) = symbols.iter().find_position(|&&x| x == '?').unwrap();
    symbols[index] = '.';
    result += generate_possible_solutions(&symbols, numbers);
    symbols[index] = '#';
    result += generate_possible_solutions(&symbols, numbers);
    result
}

#[timed]
fn part1(input: &str) -> usize {
    let parsed = parse(input);
    parsed
        .par_iter()
        .map(|(symbols, numbers)| generate_possible_solutions(symbols, numbers))
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
}
