use aoc2023::{read_input, InputType};
use timed::timed;

const DAY: u8 = 17;
fn main() {
    let input = read_input(DAY, InputType::Real).unwrap();

    println!("Part1: {}", part1(&input));
    println!("Part2: {}", part2(&input));
}

fn parse(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| line.chars().map(|x| x.to_digit(10).unwrap()).collect())
        .collect()
}
#[derive(Debug, Clone, Copy, PartialEq)]
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

fn add_next(
    map: &Vec<Vec<u32>>,
    queue: &mut Vec<((usize, usize), Direction, u8, u32)>,
    visited: &mut Vec<((usize, usize), Direction, u8)>,
    prev: ((usize, usize), Direction, u8, u32),
    direction: Direction,
) {
    let ((x, y), prev_direction, subsequential, distance) = prev;
    let subsequential = if prev_direction == direction {
        subsequential + 1
    } else {
        1
    };
    if let Some(next) = direction.apply_dir((x, y), map[0].len() - 1, map.len() - 1) {
        if !visited.contains(&(next, direction, subsequential)) {
            visited.push((next, direction, subsequential));
            queue.push((
                next,
                direction,
                subsequential,
                distance + map[next.1][next.0],
            ));
        }
    }
}
#[allow(unreachable_code)]
#[timed]
fn part1(input: &str) -> u32 {
    let map = parse(input);
    let max_x = map[0].len() - 1;
    let max_y = map.len() - 1;
    let mut queue = vec![];
    let mut visited = vec![];
    queue.push(((0, 0), Direction::Right, 1, 0));
    queue.push(((0, 0), Direction::Down, 1, 0));
    while !queue.is_empty() {
        let prev = queue.pop().unwrap();
        let ((x, y), direction, subsequential, distance) = prev;
        if x == max_x && y == max_y {
            return distance;
        }
        let need_turn = subsequential == 3;
        if !(need_turn && direction == Direction::Up) && direction != Direction::Down {
            add_next(&map, &mut queue, &mut visited, prev, Direction::Up);
        }
        if !(need_turn && direction == Direction::Right) && direction != Direction::Left {
            add_next(&map, &mut queue, &mut visited, prev, Direction::Right);
        }
        if !(need_turn && direction == Direction::Down) && direction != Direction::Up {
            add_next(&map, &mut queue, &mut visited, prev, Direction::Down);
        }
        if !(need_turn && direction == Direction::Left) && direction != Direction::Right {
            add_next(&map, &mut queue, &mut visited, prev, Direction::Left);
        }

        queue.sort_by(|(_, _, _, a), (_, _, _, b)| b.cmp(&a));
    }
    unreachable!();
}
#[allow(unreachable_code)]
#[timed]
fn part2(input: &str) -> u32 {
    let map = parse(input);
    let max_x = map[0].len() - 1;
    let max_y = map.len() - 1;
    let mut queue = vec![];
    let mut visited = vec![];
    queue.push(((0, 0), Direction::Right, 0, 0));
    queue.push(((0, 0), Direction::Down, 0, 0));
    while !queue.is_empty() {
        let prev = queue.pop().unwrap();
        let ((x, y), direction, subsequential, distance) = prev;
        if x == max_x && y == max_y && subsequential >= 4 {
            return distance;
        }
        if subsequential < 10 {
            add_next(&map, &mut queue, &mut visited, prev, direction);
        }
        if subsequential >= 4 {
            if direction != Direction::Down && direction != Direction::Up {
                add_next(&map, &mut queue, &mut visited, prev, Direction::Up);
                add_next(&map, &mut queue, &mut visited, prev, Direction::Down);
            }
            if direction != Direction::Left && direction != Direction::Right {
                add_next(&map, &mut queue, &mut visited, prev, Direction::Right);
                add_next(&map, &mut queue, &mut visited, prev, Direction::Left);
            }
        }

        queue.sort_by(|(_, _, _, a), (_, _, _, b)| b.cmp(&a));
    }
    unreachable!();
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_input(input_type: InputType) -> String {
        read_input(DAY, input_type).unwrap()
    }

    #[test]
    fn part1_test() {
        let expected = 102;
        let result = part1(&get_test_input(InputType::Test));
        assert_eq!(result, expected);
    }

    #[test]
    fn part2_test() {
        let expected = 94;
        let result = part2(&get_test_input(InputType::Test));
        assert_eq!(result, expected);
    }

    #[test]
    fn part2_test1() {
        let expected = 71;
        let result = part2(&get_test_input(InputType::Other("test2")));
        assert_eq!(result, expected);
    }
}
