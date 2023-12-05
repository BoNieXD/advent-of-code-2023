use aoc2023::{read_input, InputType};
use rayon::prelude::*;
use timed::timed;

const DAY: u8 = 5;
const NEW_LINE_CHAR: &str = "\r\n";

fn main() {
    let input = read_input(DAY, InputType::Real).unwrap();

    println!("Part1: {}", part1(&input));
    println!("Part2: {}", part2_threaded(&input));
}

struct Range {
    start: i64,
    end: i64,
    offset: i64,
}
impl Range {
    fn num_is_in(&self, num: i64) -> bool {
        return self.start <= num && num <= self.end;
    }
    fn apply_offset(&self, num: i64) -> i64 {
        return num + self.offset;
    }
}
struct Map {
    ranges: Vec<Range>,
}
impl Map {
    fn find_and_apply(&self, num: i64) -> i64 {
        let range = self.ranges.iter().filter(|x| x.num_is_in(num)).next();
        match range {
            Some(range) => range.apply_offset(num),
            None => num,
        }
    }
}

fn parse_data(input: &str) -> (Vec<i64>, Vec<Map>) {
    let split_by = NEW_LINE_CHAR.repeat(2);
    let seeds_maps = input.split(&split_by).collect::<Vec<_>>();
    let seeds = seeds_maps[0]
        .split(' ')
        .filter_map(|x| x.parse::<i64>().ok())
        .collect::<Vec<_>>();
    let maps = seeds_maps[1..]
        .into_iter()
        .map(|x| Map {
            ranges: x
                .lines()
                .skip(1)
                .map(|x| {
                    let mut numbers = x.split(' ').map(|x| x.parse::<i64>().unwrap());
                    let destination_start = numbers.next().unwrap();
                    let source_start = numbers.next().unwrap();
                    let len = numbers.next().unwrap();
                    Range {
                        start: source_start,
                        end: source_start + len - 1,
                        offset: destination_start - source_start,
                    }
                })
                .collect::<Vec<_>>(),
        })
        .collect::<Vec<_>>();
    (seeds, maps)
}

#[timed]
fn part1(input: &str) -> i64 {
    let (seeds, maps) = parse_data(input);

    let mut min_location = i64::MAX;
    for seed in seeds {
        let mut var = seed;
        for map in &maps {
            var = map.find_and_apply(var);
        }
        if var < min_location {
            min_location = var;
        }
    }
    min_location
}

#[timed]
fn part2(input: &str) -> i64 {
    let (seeds, maps) = parse_data(input);
    let mut min_location = i64::MAX;
    for seed_index in (0..seeds.len()).step_by(2) {
        println!("{} / {}", (seed_index / 2) + 1, seeds.len() / 2);
        for seed in seeds[seed_index]..seeds[seed_index] + seeds[seed_index + 1] {
            let mut var = seed;
            for map in &maps {
                var = map.find_and_apply(var);
            }
            if var < min_location {
                min_location = var;
            }
        }
    }
    min_location
}

#[timed]
fn part2_threaded(input: &str) -> i64 {
    let (seeds, maps) = parse_data(input);
    let mut min_location = i64::MAX;
    for seed_index in (0..seeds.len()).step_by(2) {
        println!("{} / {}", (seed_index / 2) + 1, seeds.len() / 2);
        let start_seed = seeds[seed_index];
        let end_seed = seeds[seed_index] + seeds[seed_index + 1];

        let location_vec = (start_seed..end_seed).into_par_iter().map(|seed| {
            let mut var = seed;
            for map in &maps {
                var = map.find_and_apply(var);
            }
            var
        });
        let min = location_vec.min().unwrap();
        if min < min_location {
            min_location = min;
        }
    }
    min_location
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_input(input_type: InputType) -> String {
        read_input(DAY, input_type).unwrap()
    }

    #[test]
    fn part1_test() {
        let expected = 35;
        let result = part1(&get_test_input(InputType::Test));
        assert_eq!(result, expected);
    }
    #[test]
    fn part2_test() {
        let expected = 46;
        let result = part2(&get_test_input(InputType::Test));
        assert_eq!(result, expected);
    }
    #[test]
    fn part2_threaded_test() {
        let expected = 46;
        let result = part2_threaded(&get_test_input(InputType::Test));
        assert_eq!(result, expected);
    }
    // WJ
    #[test]
    fn part1_test_wj() {
        let expected = 31599214;
        let result = part1(&get_test_input(InputType::Other("WJ")));
        assert_eq!(result, expected);
    }
    #[test]
    fn part2_test_wj() {
        let expected = 20358599;
        let result = part2(&get_test_input(InputType::Other("WJ")));
        assert_eq!(result, expected);
    }
    #[test]
    fn part2_threaded_test_wj() {
        let expected = 20358599;
        let result = part2_threaded(&get_test_input(InputType::Other("WJ")));
        assert_eq!(result, expected);
    }
}
