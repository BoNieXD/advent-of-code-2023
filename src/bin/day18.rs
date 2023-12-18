use aoc2023::{read_input, InputType};
use timed::timed;

const DAY: u8 = 18;
fn main() {
    let input = read_input(DAY, InputType::Real).unwrap();

    println!("Part1: {}", part1(&input));
    println!("Part2: {}", part2(&input));
}

enum Direction {
    Up,
    Right,
    Down,
    Left,
}
impl Direction {
    fn apply_dir(&self, point: (i64, i64), steps: i64) -> (i64, i64) {
        let x = point.0;
        let y = point.1;
        match self {
            Direction::Up => (x, y - 1 * steps),
            Direction::Right => (x + 1 * steps, y),
            Direction::Down => (x, y + 1 * steps),
            Direction::Left => (x - 1 * steps, y),
        }
    }
}

struct Command {
    direction: Direction,
    steps: i64,
}
impl Command {
    fn new_p1(input: &str) -> Command {
        let mut tmp = input.split(' ');
        let direction = match tmp.next().unwrap() {
            "U" => Direction::Up,
            "R" => Direction::Right,
            "D" => Direction::Down,
            "L" => Direction::Left,
            _ => unreachable!(),
        };
        let steps = tmp.next().unwrap().parse::<i64>().unwrap();
        Command { direction, steps }
    }
    fn new_p2(input: &str) -> Command {
        let tmp = input
            .rsplit_once(' ')
            .unwrap()
            .1
            .chars()
            .collect::<Vec<char>>();
        let mut number = tmp[2..8].iter();

        let direction = match number.next_back().unwrap() {
            '3' => Direction::Up,
            '0' => Direction::Right,
            '1' => Direction::Down,
            '2' => Direction::Left,
            _ => unreachable!(),
        };

        let steps = i64::from_str_radix(&number.collect::<String>(), 16).unwrap();
        Command { direction, steps }
    }
}

fn calculate_area(commands: Vec<Command>) -> usize {
    let mut points = vec![(0, 0)];
    let mut b = 0;
    for command in commands {
        let last = points.last().unwrap();
        points.push(command.direction.apply_dir(*last, command.steps));
        b += command.steps as usize;
    }
    let mut area = 0;
    for pair in points.windows(2) {
        area += (pair[0].0 * pair[1].1) - (pair[1].0 * pair[0].1);
    }
    area /= 2;
    area as usize + 1 + b / 2
}

#[timed]
fn part1(input: &str) -> usize {
    let commands: Vec<Command> = input.lines().map(Command::new_p1).collect();
    calculate_area(commands)
}

#[timed]
fn part2(input: &str) -> usize {
    let commands: Vec<Command> = input.lines().map(Command::new_p2).collect();
    calculate_area(commands)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_input(input_type: InputType) -> String {
        read_input(DAY, input_type).unwrap()
    }

    #[test]
    fn part1_test() {
        let expected = 62;
        let result = part1(&get_test_input(InputType::Test));
        assert_eq!(result, expected);
    }

    #[test]
    fn part2_test() {
        let expected = 952408144115;
        let result = part2(&get_test_input(InputType::Test));
        assert_eq!(result, expected);
    }
}
