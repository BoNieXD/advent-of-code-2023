use std::{collections::HashSet, usize};

use aoc2023::{read_input, InputType};
use timed::timed;

const DAY: u8 = 16;
fn main() {
    let input = read_input(DAY, InputType::Real).unwrap();

    println!("Part1: {}", part1(&input));
    println!("Part2: {}", part2(&input));
}

fn parse(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}
#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}
impl Direction {
    fn move_point(&mut self, mut point: Point, max_x: usize, max_y: usize) -> Option<Point> {
        match &self {
            Direction::Up => {
                if point.y == 0 {
                    return None;
                }
                point.y -= 1;
            }
            Direction::Right => {
                if point.x == max_x {
                    return None;
                }
                point.x += 1;
            }
            Direction::Down => {
                if point.y == max_y {
                    return None;
                }
                point.y += 1;
            }
            Direction::Left => {
                if point.x == 0 {
                    return None;
                }
                point.x -= 1;
            }
        }
        Some(point)
    }
    fn get_next(&self, character: char) -> Direction {
        match character {
            '\\' => match self {
                Direction::Up => Direction::Left,
                Direction::Right => Direction::Down,
                Direction::Down => Direction::Right,
                Direction::Left => Direction::Up,
            },
            '/' => match self {
                Direction::Up => Direction::Right,
                Direction::Right => Direction::Up,
                Direction::Down => Direction::Left,
                Direction::Left => Direction::Down,
            },
            _ => *self,
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
struct Point {
    x: usize,
    y: usize,
}
impl Point {
    fn new(x: usize, y: usize) -> Point {
        Point { x, y }
    }
}

fn propagate(
    map: &Vec<Vec<char>>,
    visited: &mut HashSet<(Point, Direction)>,
    position: Point,
    mut direction: Direction,
) {
    if visited.contains(&(position, direction)) {
        return;
    }
    visited.insert((position, direction));
    let max_x = map[0].len() - 1;
    let max_y = map.len() - 1;
    match map[position.y][position.x] {
        '|' => {
            if matches!(direction, Direction::Left | Direction::Right) {
                direction = Direction::Up;
                if let Some(position) = direction.move_point(position, max_x, max_y) {
                    propagate(map, visited, position, direction)
                }
                direction = Direction::Down;
            }
            if let Some(position) = direction.move_point(position, max_x, max_y) {
                propagate(map, visited, position, direction)
            }
        }
        '-' => {
            if matches!(direction, Direction::Up | Direction::Down) {
                direction = Direction::Left;
                if let Some(position) = direction.move_point(position, max_x, max_y) {
                    propagate(map, visited, position, direction)
                }
                direction = Direction::Right;
            }
            if let Some(position) = direction.move_point(position, max_x, max_y) {
                propagate(map, visited, position, direction)
            }
        }
        _ => {
            direction = direction.get_next(map[position.y][position.x]);
            if let Some(position) = direction.move_point(position, max_x, max_y) {
                propagate(map, visited, position, direction)
            }
        }
    }
}

#[timed]
fn part1(input: &str) -> usize {
    let map = parse(input);
    let mut points_directions = HashSet::new();
    propagate(
        &map,
        &mut points_directions,
        Point::new(0, 0),
        Direction::Right,
    );
    let mut visited = HashSet::new();
    for (p, _) in points_directions.iter() {
        visited.insert(p);
    }
    visited.len()
}

fn calculate(map: &Vec<Vec<char>>, starting_point: Point, starting_direction: Direction) -> usize {
    let mut points_directions = HashSet::new();
    propagate(
        map,
        &mut points_directions,
        starting_point,
        starting_direction,
    );
    let mut visited = HashSet::new();
    for (p, _) in points_directions.iter() {
        visited.insert(p);
    }
    visited.len()
}
#[timed]
fn part2(input: &str) -> usize {
    let map = parse(input);
    let mut result = 0;
    // up / down
    for x in 0..map[0].len() {
        let tmp = calculate(&map, Point::new(x, 0), Direction::Down);
        if tmp > result {
            result = tmp
        }
        let tmp = calculate(&map, Point::new(x, map.len() - 1), Direction::Up);
        if tmp > result {
            result = tmp
        }
    }
    // left / right
    for y in 0..map[0].len() {
        let tmp = calculate(&map, Point::new(0, y), Direction::Right);
        if tmp > result {
            result = tmp
        }
        let tmp = calculate(&map, Point::new(map[0].len() - 1, y), Direction::Left);
        if tmp > result {
            result = tmp
        }
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
        let expected = 46;
        let result = part1(&get_test_input(InputType::Test));
        assert_eq!(result, expected);
    }

    #[test]
    fn part2_test() {
        let expected = 51;
        let result = part2(&get_test_input(InputType::Test));
        assert_eq!(result, expected);
    }
}
