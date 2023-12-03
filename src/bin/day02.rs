use std::collections::HashMap;

use aoc2023::{read_input, InputType};
use timed::timed;

const DAY: u8 = 2;

fn main() {
    let input = read_input(DAY, InputType::Real).unwrap();

    println!("Part1: {}", part1(&input));
    println!("Part2: {}", part2(&input));
}

fn parse_data(input: &str) -> Vec<(u32, Vec<Vec<(u32, &str)>>)> {
    input
        .lines()
        .map(|line: &str| {
            let mut id_sets = line[5..].split(": ");
            let id = id_sets.next().unwrap().parse::<u32>().unwrap();
            let sets = id_sets
                .next()
                .unwrap()
                .split("; ")
                .map(|sets| {
                    sets.split(", ")
                        .map(|set| {
                            let mut num_color = set.split(' ');
                            (
                                num_color.next().unwrap().parse::<u32>().unwrap(),
                                num_color.next().unwrap(),
                            )
                        })
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>();
            (id, sets)
        })
        .collect::<Vec<_>>()
}

fn check_if_posible(sets: Vec<Vec<(u32, &str)>>) -> bool {
    let max_posible_colors = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);
    for set in sets {
        for (cubes_count, color) in set {
            if max_posible_colors.get(color).unwrap() < &cubes_count {
                return false;
            }
        }
    }
    true
}

#[timed]
fn part1(input: &str) -> u32 {
    let parsed_data = parse_data(input);
    let mut result = 0;
    for (id, sets) in parsed_data {
        if check_if_posible(sets) {
            result += id;
        }
    }
    result
}

#[timed]
fn part2(input: &str) -> u32 {
    let parsed_data = parse_data(input);
    let mut result = 0;
    for (_id, sets) in parsed_data {
        let mut max_set = HashMap::from([("red", 0), ("green", 0), ("blue", 0)]);
        for set in sets {
            for (cubes_count, color) in set {
                if max_set.get(color).unwrap() < &cubes_count {
                    max_set.insert(color, cubes_count);
                }
            }
        }
        result += max_set.get("red").unwrap()
            * max_set.get("green").unwrap()
            * max_set.get("blue").unwrap();
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
    fn part1_test() {
        let expected = 8;
        let result = part1(&get_test_input(InputType::Test));
        assert_eq!(result, expected);
    }
    #[test]
    fn part2_test() {
        let expected = 2286;
        let result = part2(&get_test_input(InputType::Test));
        assert_eq!(result, expected);
    }
}
