use aoc2023::{read_input, InputType};
use timed::timed;

const DAY: u8 = 6;

fn main() {
    let input = read_input(DAY, InputType::Real).unwrap();

    println!("Part1: {}", part1(&input));
    println!("Part2: {}", part2(&input));
}

#[timed]
fn part1(input: &str) -> f64 {
    let mut parsed = input.lines().map(|x| {
        x.split_ascii_whitespace()
            .filter_map(|x| x.parse::<f64>().ok())
            .collect::<Vec<_>>()
    });
    let times = parsed.next().unwrap();
    let distances = parsed.next().unwrap();
    let mut result = 1.0;
    for (time, distance) in times.into_iter().zip(distances) {
        let sqrt_of_delta = (time * time - 4.0 * distance).sqrt();
        let x1 = ((time + sqrt_of_delta) / 2.0).ceil();
        let x2 = ((time - sqrt_of_delta) / 2.0).floor();
        result *= x1 - x2 - 1.0;
    }
    result
}

#[timed]
fn part2(input: &str) -> f64 {
    let mut parsed = input
        .lines()
        .flat_map(|x| x.split_once(":").unwrap().1.replace(" ", "").parse::<f64>());
    let time = parsed.next().unwrap();
    let distance = parsed.next().unwrap();
    let sqrt_of_delta = (time * time - 4.0 * distance).sqrt();
    let x1 = ((time + sqrt_of_delta) / 2.0).ceil();
    let x2 = ((time - sqrt_of_delta) / 2.0).floor();
    x1 - x2 - 1.0
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_input(input_type: InputType) -> String {
        read_input(DAY, input_type).unwrap()
    }

    #[test]
    fn part1_test() {
        let expected = 288.0;
        let result = part1(&get_test_input(InputType::Test));
        assert_eq!(result, expected);
    }
    #[test]
    fn part2_test() {
        let expected = 71503.0;
        let result = part2(&get_test_input(InputType::Test));
        assert_eq!(result, expected);
    }
}
