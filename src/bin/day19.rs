use std::collections::HashMap;

use aoc2023::{read_input, InputType};
use timed::timed;

const DAY: u8 = 19;
const NEWLINE_CHAR: &str = "\n";
fn main() {
    let input = read_input(DAY, InputType::Real).unwrap();

    println!("Part1: {}", part1(&input));
    println!("Part2: {}", part2(&input));
}

fn parse(
    input: &str,
) -> (
    HashMap<&str, Vec<((char, char, usize), &str)>>,
    Vec<HashMap<char, usize>>,
) {
    let split_by = NEWLINE_CHAR.repeat(2);
    let (workflow, categories) = input.split_once(&split_by).unwrap();
    let workflow = workflow
        .lines()
        .map(|line| {
            let (key, rules) = line.split_once('{').unwrap();
            let rules = rules
                .split(',')
                .map(|x| {
                    let tmp = x.split_once(':');
                    match tmp {
                        Some((rule, destination)) => {
                            let mut rule = rule.chars();
                            let key = rule.next().unwrap();
                            let operation = rule.next().unwrap();
                            let number = rule.as_str().parse::<usize>().unwrap();
                            ((key, operation, number), destination)
                        }
                        None => {
                            let mut tmp = x.chars();
                            tmp.next_back();
                            (('.', '=', 1), tmp.as_str())
                        }
                    }
                })
                .collect::<Vec<_>>();
            (key, rules)
        })
        .collect::<HashMap<_, _>>();
    let categories = categories
        .lines()
        .map(|line| {
            let mut line = line.chars();
            line.next();
            line.next_back();
            line.as_str()
                .split(',')
                .map(|x| {
                    let tmp = x.split_once('=').unwrap();
                    (
                        tmp.0.chars().next().unwrap(),
                        tmp.1.parse::<usize>().unwrap(),
                    )
                })
                .collect::<HashMap<_, _>>()
        })
        .collect::<Vec<_>>();
    (workflow, categories)
}

#[timed]
fn part1(input: &str) -> usize {
    let (workflows, to_eval) = parse(input);
    let mut result = 0;

    for categories in to_eval {
        let mut current_workflow = "in";
        while !matches!(current_workflow, "A" | "R") {
            let tests = workflows.get(current_workflow).unwrap();
            for ((key, operation, number), destination) in tests {
                if key == &'.' {
                    current_workflow = destination;
                    break;
                }
                let value = categories.get(&key).unwrap();

                let pass = match operation {
                    '<' => value < &number,
                    '>' => value > &number,
                    _ => false,
                };
                if pass {
                    current_workflow = destination;
                    break;
                }
            }
        }
        if current_workflow == "A" {
            result += categories.values().sum::<usize>();
        }
    }
    result
}

fn calculate(
    workflows: &HashMap<&str, Vec<((char, char, usize), &str)>>,
    mut ranges: Vec<(usize, usize)>,
    workflow: &str,
) -> usize {
    if workflow == "A" {
        return ranges
            .iter()
            .fold(1, |acc, (start, end)| acc * (end - start + 1));
    }
    if workflow == "R" {
        return 0;
    }
    let mut result = 0;
    for ((key, operation, number), destination) in workflows.get(workflow).unwrap() {
        if key == &'.' {
            result += calculate(workflows, ranges, &destination);
            break;
        }
        let index = match key {
            'x' => 0,
            'm' => 1,
            'a' => 2,
            's' => 3,
            _ => unreachable!(),
        };
        let mut new_range = ranges.clone();
        match operation {
            '<' => {
                new_range[index].1 = number - 1;
                ranges[index].0 = *number;
            }
            '>' => {
                new_range[index].0 = number + 1;
                ranges[index].1 = *number;
            }
            _ => unreachable!(),
        }
        result += calculate(workflows, new_range, &destination)
    }
    result
}
#[timed]
fn part2(input: &str) -> usize {
    let (workflows, _) = parse(input);
    let ranges = vec![(1, 4000), (1, 4000), (1, 4000), (1, 4000)];
    calculate(&workflows, ranges, "in")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_input(input_type: InputType) -> String {
        read_input(DAY, input_type).unwrap()
    }

    #[test]
    fn part1_test() {
        let expected = 19114;
        let result = part1(&get_test_input(InputType::Test));
        assert_eq!(result, expected);
    }

    #[test]
    fn part2_test() {
        let expected = 167409079868000;
        let result = part2(&get_test_input(InputType::Test));
        assert_eq!(result, expected);
    }
}
