use aoc2023::{read_input, InputType};
use timed::timed;

const DAY: u8 = 13;
const NEW_LINE_CHAR: &str = "\r\n";

fn main() {
    let input = read_input(DAY, InputType::Other("KB")).unwrap();
    let parsed = parse(&input);

    println!("Part1: {}", part1(&input));
    println!("Part2: {}", part2(&input));
}

fn parse(input: &str) -> Vec<Vec<Vec<char>>> {
    let split_by = NEW_LINE_CHAR.repeat(2);
    input
        .split(&split_by)
        .map(|map| {
            map.lines()
                .map(|line| line.chars().collect::<Vec<_>>())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

fn transpose(vec: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    (0..vec[0].len())
        .map(|i| (0..vec.len()).map(|j| vec[j][i]).collect::<Vec<_>>())
        .collect::<Vec<_>>()
}

fn find_reflection(map: &Vec<Vec<char>>, smudge_count: u8) -> Option<usize> {
    for i in 1..(map.len() / 2) + 1 {
        let up = map[..i].iter().collect::<Vec<_>>();
        let down = map[i..i * 2].iter().rev().collect::<Vec<_>>();
        let mut difference = 0;
        for i in 0..up.len() {
            for j in 0..up[0].len() {
                if up[i][j] != down[i][j] {
                    difference += 1;
                }
            }
        }
        if difference == smudge_count {
            return Some(i);
        }
        let up = map[map.len() - i * 2..map.len() - i]
            .iter()
            .collect::<Vec<_>>();
        let down = map[map.len() - i..].iter().rev().collect::<Vec<_>>();
        let mut difference = 0;
        for i in 0..up.len() {
            for j in 0..up[0].len() {
                if up[i][j] != down[i][j] {
                    difference += 1;
                }
            }
        }
        if difference == smudge_count {
            return Some(map.len() - i);
        }
    }
    None
}

#[timed]
fn part1(input: &str) -> usize {
    let parsed = parse(input);
    let mut result = 0;
    for map in parsed {
        match find_reflection(&map, 0) {
            Some(row) => result += 100 * row,
            None => result += find_reflection(&transpose(&map), 0).unwrap(),
        }
    }
    result
}

#[timed]
fn part2(input: &str) -> usize {
    let parsed = parse(input);
    let mut result = 0;
    for map in parsed {
        match find_reflection(&map, 1) {
            Some(row) => result += 100 * row,
            None => result += find_reflection(&transpose(&map), 1).unwrap(),
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
        let expected = 405;
        let result = part1(&get_test_input(InputType::Test));
        assert_eq!(result, expected);
    }

    #[test]
    fn part2_test() {
        let expected = 400;
        let result = part2(&get_test_input(InputType::Test));
        assert_eq!(result, expected);
    }
}
