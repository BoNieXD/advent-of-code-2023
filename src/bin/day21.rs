use std::collections::HashSet;

use aoc2023::{read_input, InputType};
use timed::timed;

const DAY: u8 = 21;

fn main() {
    let input = read_input(DAY, InputType::Real).unwrap();

    println!("Part1: {}", part1(&input));
    println!("Part2: {}", part2());
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

fn get_starting(map: &Vec<Vec<char>>) -> (usize, usize) {
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if map[y][x] == 'S' {
                return (x, y);
            }
        }
    }
    unreachable!()
}

fn add_point(
    map: &Vec<Vec<char>>,
    set: &mut HashSet<(usize, usize)>,
    point: (usize, usize),
    direction: Direction,
) {
    if let Some(new) = direction.apply_dir(point, map[0].len() - 1, map.len() - 1) {
        if map[new.1][new.0] != '#' {
            set.insert(new);
        }
    }
}

#[timed]
fn part1(input: &str) -> usize {
    let map = parse(input);
    let mut steps = vec![HashSet::from([get_starting(&map)])];
    for _ in 0..64 {
        let last = steps.last().unwrap();
        let mut new_set = HashSet::new();
        for point in last {
            add_point(&map, &mut new_set, *point, Direction::Up);
            add_point(&map, &mut new_set, *point, Direction::Right);
            add_point(&map, &mut new_set, *point, Direction::Down);
            add_point(&map, &mut new_set, *point, Direction::Left);
        }
        steps.push(new_set)
    }

    steps.last().unwrap().len()
}

#[timed]
fn part2() -> usize {
    // Map generator for n repeating
    //
    // let repeat = 1;
    // let input_line = input.lines().map(|line| line.repeat(repeat)).join("\n");
    // let mut input = vec![];
    // for _i in 0..repeat {
    //     input.push(input_line.clone());
    // }
    // let input = input.join("\n");
    // let mut map = parse(&input);
    // let start = (map.len() / 2, map.len() / 2);

    // let mut steps = HashSet::from([start]);
    // for _i in 0..(131 * (repeat / 2) + 65) {
    //     let mut new_set = HashSet::new();
    //     for point in steps {
    //         add_point(&map, &mut new_set, point, Direction::Up);
    //         add_point(&map, &mut new_set, point, Direction::Right);
    //         add_point(&map, &mut new_set, point, Direction::Down);
    //         add_point(&map, &mut new_set, point, Direction::Left);
    //     }
    //     steps = new_set
    // }
    // for (x, y) in steps.iter() {
    //     map[*y][*x] = 'O';
    // }
    // println!("");
    // for line in map {
    //     println!("{}", line.iter().collect::<String>())
    // }
    // println!("{}", steps.len());

    // 202300 * 131 + 65 = 26501365
    let n = 202300 + 1;
    15 * (240 - 969 * n + 977 * n * n)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_input(input_type: InputType) -> String {
        read_input(DAY, input_type).unwrap()
    }

    #[test]
    fn part1_test() {
        let expected = 32000000;
        let result = part1(&get_test_input(InputType::Test));
        assert_eq!(result, expected);
    }
}
