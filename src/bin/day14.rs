use std::collections::HashMap;

use aoc2023::{read_input, InputType};
use timed::timed;

const DAY: u8 = 14;

fn main() {
    let input = read_input(DAY, InputType::Other("DP")).unwrap();

    println!("Part1: {}", part1(&input));
    println!("Part2: {}", part2(&input));
}

fn parse(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>()
}

fn move_north(map: &mut Vec<Vec<char>>, mut position: (usize, usize)) {
    if position.1 == 0 || map[position.1 - 1][position.0] != '.' {
        return;
    }
    map[position.1][position.0] = '.';
    position.1 -= 1;
    map[position.1][position.0] = 'O';
    move_north(map, position);
}
fn move_west(map: &mut Vec<Vec<char>>, mut position: (usize, usize)) {
    if position.0 == 0 || map[position.1][position.0 - 1] != '.' {
        return;
    }
    map[position.1][position.0] = '.';
    position.0 -= 1;
    map[position.1][position.0] = 'O';
    move_west(map, position);
}
fn move_south(map: &mut Vec<Vec<char>>, mut position: (usize, usize)) {
    if position.1 == map.len() - 1 || map[position.1 + 1][position.0] != '.' {
        return;
    }
    map[position.1][position.0] = '.';
    position.1 += 1;
    map[position.1][position.0] = 'O';
    move_south(map, position);
}
fn move_east(map: &mut Vec<Vec<char>>, mut position: (usize, usize)) {
    if position.0 == map[0].len() - 1 || map[position.1][position.0 + 1] != '.' {
        return;
    }
    map[position.1][position.0] = '.';
    position.0 += 1;
    map[position.1][position.0] = 'O';
    move_east(map, position);
}

#[timed]
fn part1(input: &str) -> usize {
    let mut map = parse(input);
    for y in 1..map.len() {
        for x in 0..map[0].len() {
            if map[y][x] == 'O' {
                move_north(&mut map, (x, y));
            }
        }
    }
    let line_count = map.len();
    map.into_iter().enumerate().fold(0, |acc, (index, x)| {
        acc + x.into_iter().filter(|&x| x == 'O').count() * (line_count - index)
    })
}

fn move_cycle(
    map: &Vec<Vec<char>>,
    mem: &mut HashMap<Vec<Vec<char>>, Vec<Vec<char>>>,
) -> Vec<Vec<char>> {
    if mem.contains_key(map) {
        return mem.get(map).unwrap().clone();
    }
    let mut map_copy = map.clone();
    for y in 1..map_copy.len() {
        for x in 0..map_copy[0].len() {
            if map_copy[y][x] == 'O' {
                move_north(&mut map_copy, (x, y));
            }
        }
    }
    for x in 1..map_copy[0].len() {
        for y in 0..map_copy.len() {
            if map_copy[y][x] == 'O' {
                move_west(&mut map_copy, (x, y));
            }
        }
    }
    for y in (0..map_copy.len() - 1).rev() {
        for x in 0..map_copy[0].len() {
            if map_copy[y][x] == 'O' {
                move_south(&mut map_copy, (x, y));
            }
        }
    }
    for x in (0..map_copy[0].len() - 1).rev() {
        for y in 0..map_copy.len() {
            if map_copy[y][x] == 'O' {
                move_east(&mut map_copy, (x, y));
            }
        }
    }
    mem.insert(map.to_vec(), map_copy.clone());
    return map_copy;
}

#[timed]
fn part2(input: &str) -> usize {
    let mut map = parse(input);
    let mut mem = HashMap::new();
    let line_count = map.len();
    let n = 500;
    let first_n = (1..=n)
        .map(|_| {
            map = move_cycle(&map, &mut mem);
            map.iter().enumerate().fold(0, |acc, (index, x)| {
                acc + x.into_iter().filter(|&&x| x == 'O').count() * (line_count - index)
            })
        })
        .collect::<Vec<_>>();

    let sample_size = 100;
    let samples = first_n[n - sample_size..].to_vec();

    let mut cycle_len = 2;
    let mut cycle = samples[..cycle_len].to_vec();
    while cycle != samples[cycle_len..cycle_len * 2].to_vec() {
        cycle_len += 1;
        cycle = samples[..cycle_len].to_vec();
    }
    let index = (1_000_000_000 - ((n - sample_size) + 1)) % cycle_len;

    cycle[index]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_input(input_type: InputType) -> String {
        read_input(DAY, input_type).unwrap()
    }

    #[test]
    fn part1_test() {
        let expected = 136;
        let result = part1(&get_test_input(InputType::Test));
        assert_eq!(result, expected);
    }

    #[test]
    fn part2_test() {
        let expected = 64;
        let result = part2(&get_test_input(InputType::Test));
        assert_eq!(result, expected);
    }

    #[test]
    fn part1_test_dp() {
        let expected = 107430;
        let result = part1(&get_test_input(InputType::Other("DP")));
        assert_eq!(result, expected);
    }

    #[test]
    fn part2_test_dp() {
        let expected = 96317;
        let result = part2(&get_test_input(InputType::Other("DP")));
        assert_eq!(result, expected);
    }

    #[test]
    fn part1_test_wj() {
        let expected = 106997;
        let result = part1(&get_test_input(InputType::Other("WJ")));
        assert_eq!(result, expected);
    }

    #[test]
    fn part2_test_wj() {
        let expected = 99641;
        let result = part2(&get_test_input(InputType::Other("WJ")));
        assert_eq!(result, expected);
    }
}
