use aoc2023::{read_input, InputType};
use timed::timed;

const DAY: u8 = 10;

fn main() {
    let input = read_input(DAY, InputType::Real).unwrap();

    println!("Part1: {}", part1(&input));
    println!("Part2: {}", part2(&input));
}

fn parse(input: &str) -> Vec<Vec<char>> {
    let mut lines = input.lines().collect::<Vec<_>>();
    let binding = ".".repeat(lines[0].len());
    lines.push(&binding);
    lines.push(&binding);
    lines.rotate_right(1);
    lines
        .into_iter()
        .map(|line| {
            let mut characters = line.chars().collect::<Vec<_>>();
            characters.push('.');
            characters.push('.');
            characters.rotate_right(1);
            characters
        })
        .collect()
}

fn find_available_directions(
    map: &Vec<Vec<char>>,
    position: &Position,
) -> (char, Direction, Direction) {
    let if_up = match map[position.y - 1][position.x] {
        '|' => true,
        'F' => true,
        '7' => true,
        _ => false,
    };
    let if_right = match map[position.y][position.x + 1] {
        '-' => true,
        '7' => true,
        'J' => true,
        _ => false,
    };
    let if_down = match map[position.y + 1][position.x] {
        '|' => true,
        'L' => true,
        'J' => true,
        _ => false,
    };
    let if_left = match map[position.y][position.x - 1] {
        '-' => true,
        'F' => true,
        'L' => true,
        _ => false,
    };

    let character: char;
    let first: Direction;
    let second: Direction;
    if if_up {
        first = Direction::Up;
        if if_right {
            character = 'L';
            second = Direction::Right;
        } else if if_down {
            character = '|';
            second = Direction::Down;
        } else if if_left {
            character = 'J';
            second = Direction::Left;
        } else {
            unreachable!()
        }
    } else if if_right {
        first = Direction::Right;
        if if_down {
            character = 'F';
            second = Direction::Down;
        } else if if_left {
            character = '-';
            second = Direction::Left;
        } else {
            unreachable!()
        }
    } else if if_down {
        character = '7';
        first = Direction::Down;
        second = Direction::Left;
    } else {
        unreachable!()
    }
    (character, first, second)
}

fn find_starting(map: &Vec<Vec<char>>) -> Position {
    for (y, line) in map.iter().enumerate() {
        for (x, &pipe) in line.iter().enumerate() {
            if pipe == 'S' {
                return Position::new(x, y);
            }
        }
    }
    unreachable!()
}
#[derive(Debug, Clone, PartialEq)]
struct Position {
    x: usize,
    y: usize,
}
impl Position {
    fn new(x: usize, y: usize) -> Position {
        Position { x, y }
    }
}
#[derive(Debug, Clone, PartialEq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}
impl Direction {
    fn apply_direction(&self, positon: &Position) -> Position {
        match self {
            Direction::Up => Position::new(positon.x, positon.y - 1),
            Direction::Right => Position::new(positon.x + 1, positon.y),
            Direction::Down => Position::new(positon.x, positon.y + 1),
            Direction::Left => Position::new(positon.x - 1, positon.y),
        }
    }
    fn get_next(&self, map: &Vec<Vec<char>>, positon: &Position) -> Direction {
        let next_pipe_position = self.apply_direction(positon);

        match map[next_pipe_position.y][next_pipe_position.x] {
            'L' => match self {
                Direction::Down => Direction::Right,
                Direction::Left => Direction::Up,
                _ => unreachable!(),
            },
            'J' => match self {
                Direction::Down => Direction::Left,
                Direction::Right => Direction::Up,
                _ => unreachable!(),
            },
            '7' => match self {
                Direction::Right => Direction::Down,
                Direction::Up => Direction::Left,
                _ => unreachable!(),
            },
            'F' => match self {
                Direction::Left => Direction::Down,
                Direction::Up => Direction::Right,
                _ => unreachable!(),
            },
            _ => self.clone(),
        }
    }
}

#[timed]
fn part1(input: &str) -> u32 {
    let map = parse(input);
    let starting = find_starting(&map);
    let mut result = 0;
    let (_, mut head1_dir, mut head2_dir) = find_available_directions(&map, &starting);

    let mut head1_pos = starting.clone();
    let mut head2_pos = starting.clone();

    while head1_pos != head2_pos || result == 0 {
        let tmp1 = head1_dir.apply_direction(&head1_pos);
        let tmp2 = head2_dir.apply_direction(&head2_pos);
        head1_dir = head1_dir.get_next(&map, &head1_pos);
        head2_dir = head2_dir.get_next(&map, &head2_pos);
        head1_pos = tmp1;
        head2_pos = tmp2;

        result += 1;
    }

    result
}

fn check_if_inside(map: &Vec<Vec<char>>, positon: Position) -> bool {
    // Left
    let mut count = 0;
    let str = &map[positon.y][..positon.x]
        .iter()
        .collect::<String>()
        .replace('-', "");
    count += str.match_indices("|").count();
    count += str.match_indices("FJ").count();
    count += str.match_indices("L7").count();
    if count % 2 == 0 {
        return false;
    }
    // Right
    let mut count = 0;
    let str = &map[positon.y][positon.x + 1..]
        .iter()
        .collect::<String>()
        .replace('-', "");
    count += str.match_indices("|").count();
    count += str.match_indices("FJ").count();
    count += str.match_indices("L7").count();
    if count % 2 == 0 {
        return false;
    }
    // Up
    let mut count = 0;
    let str = &map.iter().map(|line| line[positon.x]).collect::<Vec<_>>()[..positon.y]
        .iter()
        .collect::<String>()
        .replace('|', "");
    count += str.match_indices("-").count();
    count += str.match_indices("FJ").count();
    count += str.match_indices("7L").count();
    if count % 2 == 0 {
        return false;
    }
    // Down
    let mut count = 0;
    let str = &map.iter().map(|line| line[positon.x]).collect::<Vec<_>>()[positon.y + 1..]
        .iter()
        .collect::<String>()
        .replace('|', "");
    count += str.match_indices("-").count();
    count += str.match_indices("FJ").count();
    count += str.match_indices("7L").count();
    if count % 2 == 0 {
        return false;
    }
    return true;
}
#[timed]
fn part2(input: &str) -> u32 {
    let map = parse(input);
    let starting = find_starting(&map);
    let (start_char, mut head_dir, _) = find_available_directions(&map, &starting);

    let mut head_pos = starting.clone();
    let mut new_map = vec![vec!['.'; map[0].len()]; map.len()];

    new_map[starting.y][starting.x] = start_char;
    loop {
        let tmp = head_dir.apply_direction(&head_pos);
        if tmp == starting {
            break;
        }
        head_dir = head_dir.get_next(&map, &head_pos);
        head_pos = tmp;

        new_map[head_pos.y][head_pos.x] = map[head_pos.y][head_pos.x];
    }

    let mut result = 0;
    for (y, line) in new_map.iter().enumerate() {
        for (x, character) in line.iter().enumerate() {
            if character == &'.' && check_if_inside(&new_map, Position::new(x, y)) {
                result += 1
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
    fn part1_test1() {
        let expected = 4;
        let result = part1(&get_test_input(InputType::Test));
        assert_eq!(result, expected);
    }
    #[test]
    fn part1_test2() {
        let expected = 4;
        let result = part1(&get_test_input(InputType::Other("test2")));
        assert_eq!(result, expected);
    }
    #[test]
    fn part1_test3() {
        let expected = 8;
        let result = part1(&get_test_input(InputType::Other("test3")));
        assert_eq!(result, expected);
    }
    #[test]
    fn part1_test4() {
        let expected = 8;
        let result = part1(&get_test_input(InputType::Other("test4")));
        assert_eq!(result, expected);
    }
    #[test]
    fn part1_test_dp() {
        let expected = 7102;
        let result = part1(&get_test_input(InputType::Other("DP")));
        assert_eq!(result, expected);
    }
    #[test]
    fn part1_test_wj() {
        let expected = 6860;
        let result = part1(&get_test_input(InputType::Other("WJ")));
        assert_eq!(result, expected);
    }
    #[test]
    fn part1_test_reddit() {
        let expected = 6757;
        let result = part1(&get_test_input(InputType::Other("reddit")));
        assert_eq!(result, expected);
    }
    #[test]
    fn part2_test() {
        let expected = 1;
        let result = part2(&get_test_input(InputType::Test));
        assert_eq!(result, expected);
    }
    #[test]
    fn part2_test2() {
        let expected = 1;
        let result = part2(&get_test_input(InputType::Other("test2")));
        assert_eq!(result, expected);
    }
    #[test]
    fn part2_test3() {
        let expected = 1;
        let result = part2(&get_test_input(InputType::Other("test3")));
        assert_eq!(result, expected);
    }
    #[test]
    fn part2_test4() {
        let expected = 1;
        let result = part2(&get_test_input(InputType::Other("test4")));
        assert_eq!(result, expected);
    }
    #[test]
    fn part2_test5() {
        let expected = 4;
        let result = part2(&get_test_input(InputType::Other("test-p2")));
        assert_eq!(result, expected);
    }
    #[test]
    fn part2_test6() {
        let expected = 8;
        let result = part2(&get_test_input(InputType::Other("test2-p2")));
        assert_eq!(result, expected);
    }
    #[test]
    fn part2_test7() {
        let expected = 10;
        let result = part2(&get_test_input(InputType::Other("test3-p2")));
        assert_eq!(result, expected);
    }
    #[test]
    fn part2_test_dp() {
        let expected = 363;
        let result = part2(&get_test_input(InputType::Other("DP")));
        assert_eq!(result, expected);
    }
    #[test]
    fn part2_test_wj() {
        let expected = 343;
        let result = part2(&get_test_input(InputType::Other("WJ")));
        assert_eq!(result, expected);
    }
    #[test]
    fn part2_test_reddit() {
        let expected = 523;
        let result = part2(&get_test_input(InputType::Other("reddit")));
        assert_eq!(result, expected);
    }
}
