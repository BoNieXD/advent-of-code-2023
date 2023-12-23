use aoc2023::{read_input, InputType};
use timed::timed;

const DAY: u8 = 23;

fn main() {
    let input = read_input(DAY, InputType::Real).unwrap();

    println!("Part1: {}", part1(&input));
    println!("Part2: {}", part2(&input));
}

fn parse(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}
impl Direction {
    fn from(character: char) -> Option<Direction> {
        match character {
            '^' => Some(Direction::Up),
            '>' => Some(Direction::Right),
            'v' => Some(Direction::Down),
            '<' => Some(Direction::Left),
            _ => None,
        }
    }
    fn apply_dir(
        &self,
        mut point: (usize, usize),
        max_x: usize,
        max_y: usize,
    ) -> Option<(usize, usize)> {
        match self {
            Direction::Up => {
                if point.1 > 0 {
                    point.1 -= 1;
                    return Some(point);
                }
            }
            Direction::Right => {
                if point.0 < max_x {
                    point.0 += 1;
                    return Some(point);
                }
            }
            Direction::Down => {
                if point.1 < max_y {
                    point.1 += 1;
                    return Some(point);
                }
            }
            Direction::Left => {
                if point.0 > 0 {
                    point.0 -= 1;
                    return Some(point);
                }
            }
        }
        None
    }
}

fn calculate(
    map: &mut Vec<Vec<char>>,
    current: (usize, usize),
    end: (usize, usize),
    distance: usize,
    part1: bool,
) -> usize {
    if current == end {
        return distance;
    }
    let mut result = 0;

    let mut directions = vec![
        Direction::Up,
        Direction::Right,
        Direction::Down,
        Direction::Left,
    ];
    if part1 {
        if let Some(direction) = Direction::from(map[current.1][current.0]) {
            directions = vec![direction]
        }
    }

    directions.into_iter().for_each(|dir| {
        if let Some(next) = dir.apply_dir(current, map[0].len() - 1, map.len() - 1) {
            if map[next.1][next.0] != '#' {
                let tmp = map[next.1][next.0];
                if map[next.1][next.0] == '.' || !part1 {
                    map[next.1][next.0] = '#';
                }
                let r = calculate(map, next, end, distance + 1, part1);
                map[next.1][next.0] = tmp;
                if r > result {
                    result = r;
                }
            }
        }
    });
    result
}

#[timed]
fn part1(input: &str) -> usize {
    let mut map = parse(input);
    let max_x = map[0].len() - 1;
    let max_y = map.len() - 1;
    map[0][1] = '#';
    calculate(&mut map, (1, 0), (max_x - 1, max_y), 0, true)
}

#[timed]
fn part2(input: &str) -> usize {
    let mut map = parse(input);
    let max_x = map[0].len() - 1;
    let max_y = map.len() - 1;
    map[0][1] = '#';
    calculate(&mut map, (1, 0), (max_x - 1, max_y), 0, false)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_input(input_type: InputType) -> String {
        read_input(DAY, input_type).unwrap()
    }

    #[test]
    fn part1_test() {
        let expected = 94;
        let result = part1(&get_test_input(InputType::Test));
        assert_eq!(result, expected);
    }

    #[test]
    fn part2_test() {
        let expected = 154;
        let result = part2(&get_test_input(InputType::Test));
        assert_eq!(result, expected);
    }
}
