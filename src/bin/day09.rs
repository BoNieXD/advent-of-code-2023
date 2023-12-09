use aoc2023::{read_input, InputType};
use timed::timed;

const DAY: u8 = 9;

fn main() {
    let input = read_input(DAY, InputType::Real).unwrap();

    println!("Part1: {}", part1(&input));
    println!("Part2: {}", part2(&input));
}

fn generate_sequence_output(mut sequence: Vec<i32>) -> i32 {
    let mut result = 0;
    while !sequence.iter().all(|&x| x == 0) {
        result += sequence.last().unwrap();
        let mut next_sequence = vec![];
        for pair in sequence.windows(2) {
            next_sequence.push(pair[1] - pair[0]);
        }
        sequence = next_sequence;
    }
    result
}

#[timed]
fn part1(input: &str) -> i32 {
    input
        .lines()
        .map(|x| x.split(' ').flat_map(|x| x.parse()).collect::<Vec<_>>())
        .map(generate_sequence_output)
        .sum()
}

#[timed]
fn part2(input: &str) -> i32 {
    input
        .lines()
        .map(|x| {
            x.split(' ')
                .flat_map(|x| x.parse())
                .rev()
                .collect::<Vec<_>>()
        })
        .map(generate_sequence_output)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_input(input_type: InputType) -> String {
        read_input(DAY, input_type).unwrap()
    }

    #[test]
    fn part1_test() {
        let expected = 114;
        let result = part1(&get_test_input(InputType::Test));
        assert_eq!(result, expected);
    }
    #[test]
    fn part2_test() {
        let expected = 2;
        let result = part2(&get_test_input(InputType::Test));
        assert_eq!(result, expected);
    }
}
