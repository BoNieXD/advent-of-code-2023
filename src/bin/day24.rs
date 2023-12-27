use aoc2023::{read_input, InputType};
use timed::timed;

const DAY: u8 = 24;

fn main() {
    let input = read_input(DAY, InputType::Real).unwrap();

    println!("Part1: {}", part1(&input));
}

fn parse(input: &str) -> Vec<((f64, f64, f64), (f64, f64, f64))> {
    input
        .lines()
        .map(|line| {
            let mut line = line
                .split(" @ ")
                .map(|x| x.split(", ").flat_map(|x| x.trim().parse::<f64>()));
            let mut pos = line.next().unwrap();
            let mut vel = line.next().unwrap();
            (
                (
                    pos.next().unwrap(),
                    pos.next().unwrap(),
                    pos.next().unwrap(),
                ),
                (
                    vel.next().unwrap(),
                    vel.next().unwrap(),
                    vel.next().unwrap(),
                ),
            )
        })
        .collect::<Vec<_>>()
}

fn intersection_xy(
    l1: ((f64, f64, f64), (f64, f64, f64)),
    l2: ((f64, f64, f64), (f64, f64, f64)),
) -> Option<(f64, f64)> {
    let ((x1, y1, _), (vx, vy, _)) = l1;
    let x2 = x1 + vx;
    let y2 = y1 + vy;
    let ((x3, y3, _), (vx, vy, _)) = l2;
    let x4 = x3 + vx;
    let y4 = y3 + vy;
    let den = (x1 - x2) * (y3 - y4) - (y1 - y2) * (x3 - x4);
    if den == 0.0 {
        return None;
    }
    let t = ((x1 - x3) * (y3 - y4) - (y1 - y3) * (x3 - x4)) / den;
    let u = ((x1 - x3) * (y1 - y2) - (y1 - y3) * (x1 - x2)) / den;
    if 0.0 <= t && 0.0 <= u {
        let x = x1 + t * (x2 - x1);
        let y = y1 + t * (y2 - y1);
        return Some((x, y));
    }
    None
}

#[timed]
fn part1(input: &str) -> usize {
    let parsed = parse(input);
    let mut result = 0;
    for i in 0..parsed.len() - 1 {
        for j in i + 1..parsed.len() {
            let l1 = parsed[i];
            let l2 = parsed[j];
            if let Some(point) = intersection_xy(l1, l2) {
                let range = 200_000_000_000_000.0..=400_000_000_000_000.0;
                // let range = 7.0..=27.0;
                if range.contains(&point.0) && range.contains(&point.1) {
                    result += 1
                }
            }
        }
    }

    result
}
