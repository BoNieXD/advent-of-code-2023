use std::{cmp::Ordering, collections::HashMap};

use aoc2023::{read_input, InputType};
use itertools::Itertools;
use timed::timed;

const DAY: u8 = 7;

fn main() {
    let input = read_input(DAY, InputType::Real).unwrap();

    println!("Part1: {}", part1(&input));
    println!("Part2: {}", part2(&input));
}
#[derive(Debug)]
struct Card {
    label: char,
    value: u8,
}
impl Card {
    fn new(label: char) -> Card {
        Card {
            label,
            value: match label {
                'A' => 14,
                'K' => 13,
                'Q' => 12,
                'J' => 11,
                'T' => 10,
                'Y' => 1,
                _ => label.to_digit(10).unwrap() as u8,
            },
        }
    }
}

#[derive(Debug)]
struct Hand {
    cards: Vec<Card>,
    value: u8,
    bid: u32,
}

impl Hand {
    fn get_hand_value(
        joker_count: i32,
        distinct_cards_count: usize,
        min_count_of_distinct_cards: i32,
        max_count_of_distinct_cards: i32,
    ) -> u8 {
        let max_count_of_distinct_cards = max_count_of_distinct_cards + joker_count;
        match (
            distinct_cards_count,
            min_count_of_distinct_cards,
            max_count_of_distinct_cards,
        ) {
            (1, _, _) => 7, // Five of a kind
            (2, _, 4) => 6, // Four of a kind
            (2, 2, 3) => 5, // Full house
            (3, _, 3) => 4, // Three of a kind
            (3, 1, 2) => 3, // Two pair
            (4, _, 2) => 2, // One pair
            (5, _, _) => 1, // High card
            _ => unreachable!(),
        }
    }
    fn new(cards_str: &str, bid: u32) -> Hand {
        let mut cards = vec![];
        for card_char in cards_str.chars() {
            cards.push(Card::new(card_char));
        }
        let mut count = cards.iter().fold(HashMap::new(), |mut acc, x| {
            *acc.entry(x.label).or_insert(0) += 1;
            acc
        });
        let joker_count = *count.get(&'Y').unwrap_or(&0);
        count.remove(&'Y');
        let count = count.values().collect::<Vec<_>>();
        let min_count = count.iter().min().unwrap_or(&&0);
        let max_count = count.iter().max().unwrap_or(&&0);
        let value = match joker_count {
            0..=3 => Hand::get_hand_value(joker_count, count.len(), **min_count, **max_count),
            _ => 7,
        };

        Hand { cards, value, bid }
    }
}
impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.value.cmp(&other.value) == Ordering::Equal {
            for i in 0..5 {
                if self.cards[i].value.cmp(&other.cards[i].value) != Ordering::Equal {
                    return self.cards[i].value.cmp(&other.cards[i].value);
                }
            }
            unreachable!()
        } else {
            self.value.cmp(&other.value)
        }
    }
}
impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.value.cmp(&other.value))
    }
}
impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}
impl Eq for Hand {}

#[timed]
fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|x| {
            let mut hand = x.split(' ');
            Hand::new(
                hand.next().unwrap(),
                hand.next().unwrap().parse::<u32>().unwrap(),
            )
        })
        .sorted_by(|a, b| a.cmp(b))
        .enumerate()
        .fold(0, |acc, (index, hand)| acc + hand.bid * (index as u32 + 1))
}

#[timed]
fn part2(input: &str) -> u32 {
    input
        .lines()
        .map(|x| {
            let mut hand = x.split(' ');
            Hand::new(
                &hand.next().unwrap().replace("J", "Y"),
                hand.next().unwrap().parse::<u32>().unwrap(),
            )
        })
        .sorted_by(|a, b| a.cmp(b))
        .enumerate()
        .fold(0, |acc, (index, hand)| acc + hand.bid * (index as u32 + 1))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_input(input_type: InputType) -> String {
        read_input(DAY, input_type).unwrap()
    }

    #[test]
    fn part1_test() {
        let expected = 6440;
        let result = part1(&get_test_input(InputType::Test));
        assert_eq!(result, expected);
    }
    #[test]
    fn part2_test() {
        let expected = 5905;
        let result = part2(&get_test_input(InputType::Test));
        assert_eq!(result, expected);
    }
    #[test]
    fn part1_test2() {
        let expected = 6592;
        let result = part1(&get_test_input(InputType::Other("test2")));
        assert_eq!(result, expected);
    }
    #[test]
    fn part2_test2() {
        let expected = 6839;
        let result = part2(&get_test_input(InputType::Other("test2")));
        assert_eq!(result, expected);
    }
    #[test]
    fn part1_test_wj() {
        let expected = 251927063;
        let result = part1(&get_test_input(InputType::Other("WJ")));
        assert_eq!(result, expected);
    }
    #[test]
    fn part2_test_wj() {
        let expected = 255632664;
        let result = part2(&get_test_input(InputType::Other("WJ")));
        assert_eq!(result, expected);
    }
    #[test]
    fn part1_test_dp() {
        let expected = 251927063;
        let result = part1(&get_test_input(InputType::Other("DP")));
        assert_eq!(result, expected);
    }
    #[test]
    fn part2_test_dp() {
        let expected = 255632664;
        let result = part2(&get_test_input(InputType::Other("DP")));
        assert_eq!(result, expected);
    }
    #[test]
    fn part1_test_kk() {
        let expected = 253205868;
        let result = part1(&get_test_input(InputType::Other("KK")));
        assert_eq!(result, expected);
    }
    #[test]
    fn part2_test_kk() {
        let expected = 253907829;
        let result = part2(&get_test_input(InputType::Other("KK")));
        assert_eq!(result, expected);
    }
}
