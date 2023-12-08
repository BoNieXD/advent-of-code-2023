use std::collections::HashMap;

use aoc2023::{read_input, InputType};
use timed::timed;

const DAY: u8 = 8;
const NEW_LINE_CHAR: &str = "\r\n";

fn main() {
    let input = read_input(DAY, InputType::Real).unwrap();

    println!("Part1: {}", part1(&input));
    println!("Part2 LCM: {}", part2_lcm(&input));
    println!("Part2 Bruteforce: {}", part2_bruteforce(&input));
}

fn parse_data(input: &str) -> (Vec<char>, HashMap<&str, (&str, &str)>) {
    let split_by = NEW_LINE_CHAR.repeat(2);
    let mut lines = input.split(&split_by);
    let lr_directions = lines.next().unwrap().chars().collect::<Vec<_>>();
    let node_map = lines
        .next()
        .unwrap()
        .lines()
        .fold(HashMap::new(), |mut acc, line| {
            let (label, lr) = line.split_once(" = ").unwrap();
            let lr: (&str, &str) = lr[1..9].split_once(", ").unwrap();
            acc.insert(label, lr);
            acc
        });
    (lr_directions, node_map)
}

#[timed]
fn part1(input: &str) -> u32 {
    let (lr_directions, node_map) = parse_data(input);
    let mut current = "AAA";
    let mut direction_index = 0;
    let mut steps = 0;
    while current != "ZZZ" {
        let directions = node_map.get(&current).unwrap();
        if lr_directions[direction_index] == 'L' {
            current = directions.0;
        } else {
            current = directions.1;
        }
        steps += 1;
        direction_index = (direction_index + 1) % lr_directions.len()
    }
    steps
}

#[timed]
fn part2_bruteforce(input: &str) -> u128 {
    let (lr_directions, node_map) = parse_data(input);
    let mut current_nodes = node_map
        .keys()
        .filter(|node| node.ends_with('A'))
        .collect::<Vec<_>>();
    let mut direction_index = 0;
    let mut steps = 0;
    while current_nodes
        .iter()
        .filter(|node| !node.ends_with('Z'))
        .count()
        != 0
    {
        for node in current_nodes.iter_mut() {
            let directions = node_map.get(*node).unwrap();
            if lr_directions[direction_index] == 'L' {
                *node = &directions.0;
            } else {
                *node = &directions.1;
            }
        }

        steps += 1;
        direction_index = (direction_index + 1) % lr_directions.len()
    }
    steps
}

fn find_first_occurrence(
    starting: &str,
    lr_directions: &Vec<char>,
    node_map: &HashMap<&str, (&str, &str)>,
) -> u128 {
    let mut current = starting;
    let mut direction_index = 0;
    let mut first_occurrence = 0;
    while !current.ends_with('Z') {
        let directions = node_map.get(&current).unwrap();
        if lr_directions[direction_index] == 'L' {
            current = directions.0;
        } else {
            current = directions.1;
        }
        first_occurrence += 1;
        direction_index = (direction_index + 1) % lr_directions.len();
    }
    first_occurrence
}

fn gcd(mut a: u128, mut b: u128) -> u128 {
    while a != b {
        if a > b {
            a -= b;
        } else {
            b -= a;
        }
    }
    a
}
fn lcm(a: u128, b: u128) -> u128 {
    (a * b) / gcd(a, b)
}

#[timed]
fn part2_lcm(input: &str) -> u128 {
    let (lr_directions, node_map) = parse_data(input);
    node_map
        .keys()
        .filter(|node| node.ends_with('A'))
        .map(|x| find_first_occurrence(x, &lr_directions, &node_map))
        .fold(1, |acc, x| lcm(acc, x))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_input(input_type: InputType) -> String {
        read_input(DAY, input_type).unwrap()
    }

    #[test]
    fn part1_test() {
        let expected = 2;
        let result = part1(&get_test_input(InputType::Test));
        assert_eq!(result, expected);
    }
    #[test]
    fn part1_test2() {
        let expected = 6;
        let result = part1(&get_test_input(InputType::Other("test2")));
        assert_eq!(result, expected);
    }
    #[test]
    fn part2_lcm_test() {
        let expected = 6;
        let result = part2_lcm(&get_test_input(InputType::Other("test3 part2")));
        assert_eq!(result, expected);
    }
    #[test]
    fn part2_bruteforce_test() {
        let expected = 6;
        let result = part2_bruteforce(&get_test_input(InputType::Other("test3 part2")));
        assert_eq!(result, expected);
    }
}
