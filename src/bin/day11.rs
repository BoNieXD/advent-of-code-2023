use aoc2023::{read_input, InputType};
use timed::timed;

const DAY: u8 = 11;

fn main() {
    let input = read_input(DAY, InputType::Real).unwrap();

    println!("Part1: {}", part1(&input));
    println!("Part2: {}", part2(&input));
}
#[derive(Debug)]
struct Point {
    x: usize,
    y: usize,
}
impl Point {
    fn new(x: usize, y: usize) -> Point {
        Point { x, y }
    }
    fn distance(p1: &Point, p2: &Point) -> usize {
        p1.x.abs_diff(p2.x) + p1.y.abs_diff(p2.y)
    }
}

fn parse(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|x| x.chars().collect()).collect()
}

fn calculate_result(expand_rate: usize, input: &str) -> usize {
    let parsed = parse(input);
    let mut points = vec![];
    let mut y_offset = 0;
    for (y, line) in parsed.iter().enumerate() {
        let mut x_offset = 0;
        if parsed[y].iter().all(|&x| x == '.') {
            y_offset += expand_rate;
            continue;
        }
        for (x, character) in line.iter().enumerate() {
            if parsed.iter().all(|line| line[x] == '.') {
                x_offset += expand_rate;
                continue;
            }
            if *character == '#' {
                points.push(Point::new(x + x_offset, y + y_offset))
            }
        }
    }
    let mut result = 0;
    for i in 0..points.len() {
        for j in i + 1..points.len() {
            result += Point::distance(&points[i], &points[j])
        }
    }
    result
}

#[timed]
fn part1(input: &str) -> usize {
    calculate_result(1, input)
}

#[timed]
fn part2(input: &str) -> usize {
    calculate_result(1_000_000 - 1, input)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_input(input_type: InputType) -> String {
        read_input(DAY, input_type).unwrap()
    }

    #[test]
    fn part1_test() {
        let expected = 374;
        let result = part1(&get_test_input(InputType::Test));
        assert_eq!(result, expected);
    }
    #[test]
    fn part2_test() {
        let expected = 82000210;
        let result = part2(&get_test_input(InputType::Test));
        assert_eq!(result, expected);
    }
}
