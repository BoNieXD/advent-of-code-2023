use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashSet},
};

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Node {
    position: (usize, usize),
    direction: Direction,
    subsequential: u8,
}
impl Node {
    fn new(position: (usize, usize), direction: Direction, subsequential: u8) -> Node {
        Node {
            position,
            direction,
            subsequential,
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct DistNode {
    node: Node,
    distance: u32,
}
impl DistNode {
    fn new(node: Node, distance: u32) -> DistNode {
        DistNode { node, distance }
    }
}
impl Ord for DistNode {
    fn cmp(&self, other: &Self) -> Ordering {
        other.distance.cmp(&self.distance)
    }
}
impl PartialOrd for DistNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn add_next(
    map: &Vec<Vec<u32>>,
    queue: &mut BinaryHeap<DistNode>,
    visited: &mut HashSet<Node>,
    prev: DistNode,
    direction: Direction,
) {
    let subsequential = if prev.node.direction == direction {
        prev.node.subsequential + 1
    } else {
        1
    };
    if let Some(next) = direction.apply_dir(prev.node.position, map[0].len() - 1, map.len() - 1) {
        let node = Node::new(next, direction, subsequential);
        if !visited.contains(&node) {
            visited.insert(node);
            queue.push(DistNode::new(node, prev.distance + map[next.1][next.0]));
        }
    }
}
#[timed]
fn part1(input: &str) -> u32 {
    let map = parse(input);
    let max_x = map[0].len() - 1;
    let max_y = map.len() - 1;
    let mut queue = BinaryHeap::new();
    let mut visited = HashSet::new();
    queue.push(DistNode::new(Node::new((0, 0), Direction::Right, 0), 0));
    queue.push(DistNode::new(Node::new((0, 0), Direction::Down, 0), 0));
    let mut result = 0;
    while !queue.is_empty() {
        let prev = queue.pop().unwrap();
        if prev.node.position == (max_x, max_y) {
            result = prev.distance;
            break;
        }
        let need_turn = prev.node.subsequential == 3;
        let direction = prev.node.direction;
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
    }
    result
}

#[timed]
fn part2(input: &str) -> u32 {
    let map = parse(input);
    let max_x = map[0].len() - 1;
    let max_y = map.len() - 1;
    let mut queue = BinaryHeap::new();
    let mut visited = HashSet::new();
    queue.push(DistNode::new(Node::new((0, 0), Direction::Right, 0), 0));
    queue.push(DistNode::new(Node::new((0, 0), Direction::Down, 0), 0));
    let mut result = 0;
    while !queue.is_empty() {
        let prev = queue.pop().unwrap();
        if prev.node.position == (max_x, max_y) && prev.node.subsequential >= 4 {
            result = prev.distance;
            break;
        }
        if prev.node.subsequential < 10 {
            add_next(&map, &mut queue, &mut visited, prev, prev.node.direction);
        }
        if prev.node.subsequential >= 4 {
            if !matches!(prev.node.direction, Direction::Down | Direction::Up) {
                add_next(&map, &mut queue, &mut visited, prev, Direction::Up);
                add_next(&map, &mut queue, &mut visited, prev, Direction::Down);
            }
            if !matches!(prev.node.direction, Direction::Left | Direction::Right) {
                add_next(&map, &mut queue, &mut visited, prev, Direction::Right);
                add_next(&map, &mut queue, &mut visited, prev, Direction::Left);
            }
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
        let expected = 102;
        let result = part1(&get_test_input(InputType::Test));
        assert_eq!(result, expected);
    }

    #[test]
    fn part1_test_kk() {
        let expected = 902;
        let result = part1(&get_test_input(InputType::Other("KK")));
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
