use std::collections::HashSet;

use aoc2023::{read_input, InputType};
use timed::timed;

const DAY: u8 = 22;

fn main() {
    let input = read_input(DAY, InputType::Real).unwrap();
    let (p1, p2) = calculate(&input);

    println!("Part1: {}", p1);
    println!("Part2: {}", p2);
}

#[derive(Debug, Clone)]
struct Position {
    x: usize,
    y: usize,
    z: usize,
}
impl Position {
    fn new(x: usize, y: usize, z: usize) -> Position {
        Position { x, y, z }
    }
}

#[derive(Debug, Clone)]
struct Brick {
    start: Position,
    end: Position,
}
impl Brick {
    fn new(start: Position, end: Position) -> Brick {
        Brick { start, end }
    }

    fn intersect(&self, other: &Brick) -> bool {
        if self.start.z.max(other.start.z) <= self.end.z.min(other.end.z) {
            if (self.start.x.max(other.start.x) <= self.end.x.min(other.end.x))
                && (self.start.y.max(other.start.y) <= self.end.y.min(other.end.y))
            {
                return true;
            }
        }
        false
    }

    fn if_fallen(&self) -> Option<Brick> {
        if self.start.z > 1 {
            let mut result = self.clone();
            result.start.z -= 1;
            result.end.z -= 1;
            return Some(result);
        }
        None
    }
}

fn parse(input: &str) -> Vec<Brick> {
    input
        .lines()
        .map(|line| {
            let tmp = line.split_once('~').unwrap();
            let mut start = tmp.0.split(',').flat_map(|x| x.parse::<usize>());
            let mut end = tmp.1.split(',').flat_map(|x| x.parse::<usize>());
            Brick::new(
                Position::new(
                    start.next().unwrap(),
                    start.next().unwrap(),
                    start.next().unwrap(),
                ),
                Position::new(
                    end.next().unwrap(),
                    end.next().unwrap(),
                    end.next().unwrap(),
                ),
            )
        })
        .collect()
}

fn drop(bricks: &mut Vec<Brick>) -> usize {
    let mut changed = HashSet::new();
    'outer: for i in 0..bricks.len() {
        loop {
            let current = &bricks[i];
            match current.if_fallen() {
                Some(fallen) => {
                    for brick in &bricks[..i] {
                        if fallen.intersect(brick) {
                            continue 'outer;
                        }
                    }
                    bricks[i] = fallen;
                    changed.insert(i);
                }
                None => break,
            }
        }
    }
    changed.len()
}

#[timed]
fn calculate(input: &str) -> (usize, usize) {
    let mut bricks = parse(input);
    bricks.sort_by(|a, b| a.start.z.cmp(&b.start.z));
    drop(&mut bricks);
    let mut result_p1 = 0;
    let mut result_p2 = 0;
    for i in 0..bricks.len() {
        let mut bricks_new = bricks.clone();
        bricks_new.remove(i);
        let changed = drop(&mut bricks_new);
        result_p2 += changed;
        if changed == 0 {
            result_p1 += 1;
        }
    }
    (result_p1, result_p2)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_input(input_type: InputType) -> String {
        read_input(DAY, input_type).unwrap()
    }

    #[test]
    fn part1_test() {
        let expected = 5;
        let result = calculate(&get_test_input(InputType::Test)).0;
        assert_eq!(result, expected);
    }

    #[test]
    fn part2_test() {
        let expected = 7;
        let result = calculate(&get_test_input(InputType::Test)).1;
        assert_eq!(result, expected);
    }
}
