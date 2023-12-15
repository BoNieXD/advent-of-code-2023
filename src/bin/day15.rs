use std::usize;

use aoc2023::{read_input, InputType};
use timed::timed;

const DAY: u8 = 15;
fn main() {
    let input = read_input(DAY, InputType::Test).unwrap();

    println!("Part1: {}", part1(&input));
    println!("Part2: {}", part2(&input));
}

fn hash(input: &str) -> usize {
    input
        .chars()
        .fold(0, |acc, x| ((acc + x as usize) * 17) % 256)
}

#[timed]
fn part1(input: &str) -> usize {
    input.split(',').map(|x| hash(x)).sum()
}

enum Command {
    Equals(usize),
    Minus,
}
struct Step<'a> {
    label: &'a str,
    command: Command,
}
#[derive(Debug, Clone)]
struct Lens<'a> {
    label: &'a str,
    focal_length: usize,
}
impl Lens<'_> {
    fn new(label: &str, focal_length: usize) -> Lens {
        Lens {
            label,
            focal_length,
        }
    }
}

fn parse_part2(input: &str) -> Vec<Step> {
    input
        .split(',')
        .map(|x| {
            let mut tmp = x.split(['-', '=']);
            let label = tmp.next().unwrap();
            let command = match tmp.last().unwrap().parse::<usize>() {
                Ok(focal_length) => Command::Equals(focal_length),
                _ => Command::Minus,
            };
            Step { label, command }
        })
        .collect::<Vec<_>>()
}

#[timed]
fn part2(input: &str) -> usize {
    let parsed = parse_part2(input);
    let mut boxes: Vec<Vec<Lens>> = vec![vec![]; 256];
    for step in parsed.iter() {
        match step.command {
            Command::Minus => {
                boxes[hash(&step.label)] = boxes[hash(&step.label)]
                    .clone()
                    .into_iter()
                    .filter(|lens| lens.label != step.label)
                    .collect::<Vec<_>>()
            }
            Command::Equals(focal_length) => {
                let index = boxes[hash(&step.label)]
                    .iter()
                    .position(|lens| lens.label == step.label);
                match index {
                    Some(index) => boxes[hash(&step.label)][index].focal_length = focal_length,
                    None => boxes[hash(&step.label)].push(Lens::new(step.label, focal_length)),
                }
            }
        }
    }
    boxes
        .into_iter()
        .enumerate()
        .fold(0, |acc, (box_index, lens_box)| {
            acc + lens_box
                .into_iter()
                .enumerate()
                .fold(0, |acc, (lens_index, lens)| {
                    acc + (box_index + 1) * (lens_index + 1) * lens.focal_length
                })
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_input(input_type: InputType) -> String {
        read_input(DAY, input_type).unwrap()
    }

    #[test]
    fn part1_test() {
        let expected = 1320;
        let result = part1(&get_test_input(InputType::Test));
        assert_eq!(result, expected);
    }

    #[test]
    fn part2_test() {
        let expected = 145;
        let result = part2(&get_test_input(InputType::Test));
        assert_eq!(result, expected);
    }
}
