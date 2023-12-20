use std::collections::HashMap;

use aoc2023::{read_input, InputType};
use timed::timed;

const DAY: u8 = 20;

fn main() {
    let input = read_input(DAY, InputType::Real).unwrap();

    println!("Part1: {}", part1(&input));
    println!("Part2: {}", part2(&input));
}

#[derive(Debug, Clone)]
struct Module<'a> {
    mod_type: char,
    inputs: Vec<&'a str>,
    outputs: Vec<&'a str>,
    state: bool,
}
impl Module<'_> {
    fn new(mod_type: char) -> Module<'static> {
        Module {
            mod_type,
            inputs: vec![],
            outputs: vec![],
            state: false,
        }
    }
    fn update(&mut self, modules: &HashMap<&str, Module>, input: bool) {
        if self.mod_type == '%' {
            if !input {
                self.state = !self.state
            }
        } else if self.mod_type == '&' {
            let mut inputs = self.inputs.iter().map(|name| modules.get(name).unwrap());
            if inputs.all(|x| x.state) {
                self.state = false;
            } else {
                self.state = true;
            }
        }
    }
}

fn parse(input: &str) -> HashMap<&str, Module> {
    input.lines().fold(HashMap::new(), |mut acc, line| {
        let (mut name, output) = line.split_once(" -> ").unwrap();
        let mod_type;
        if name != "broadcaster" {
            let mut name_chars = name.chars();
            mod_type = name_chars.next().unwrap();
            name = name_chars.as_str();
        } else {
            mod_type = 'b'
        }
        let module = acc.entry(name).or_insert(Module::new(mod_type));
        module.mod_type = mod_type;
        let outputs = output.split(", ").collect::<Vec<_>>();
        module.outputs = outputs.clone();
        for output in outputs {
            let module = acc.entry(output).or_insert(Module::new('.'));
            module.inputs.push(name)
        }
        acc
    })
}

fn send_output(modules: &mut HashMap<&str, Module>, name: &str) -> (usize, usize) {
    let modules_copy = modules.clone();
    let current = modules_copy.get(name).unwrap();
    let mut low = 0;
    let mut high = 0;
    for &output in current.outputs.iter() {
        let output_mod = modules.get(output).unwrap();
        low += (!current.state) as usize;
        high += (current.state) as usize;
        if output_mod.mod_type != '%' || !current.state {
            modules
                .get_mut(output)
                .unwrap()
                .update(&modules_copy, current.state);
            let tmp = send_output(modules, output);
            low += tmp.0;
            high += tmp.1;
        }
    }
    (low, high)
}

#[timed]
fn part1(input: &str) -> usize {
    let mut modules = parse(input);
    let mut result = (0, 0);
    for _cycle in 0..1000 {
        let (low, high) = send_output(&mut modules, "broadcaster");
        result.0 += low + 1;
        result.1 += high;
    }
    result.0 * result.1
}

fn send_output_p2(modules: &mut HashMap<&str, Module>, name: &str, to_find: &str) -> bool {
    let modules_copy = modules.clone();
    let current = modules_copy.get(name).unwrap();
    if name == to_find && current.state {
        return true;
    }
    let mut result = false;
    for &output in current.outputs.iter() {
        let output_mod = modules.get(output).unwrap();
        if output_mod.mod_type != '%' || !current.state {
            modules
                .get_mut(output)
                .unwrap()
                .update(&modules_copy, current.state);
            result |= send_output_p2(modules, output, to_find);
            if result {
                return result;
            }
        }
    }
    result
}

fn gcd(mut a: usize, mut b: usize) -> usize {
    while a != b {
        if a > b {
            a -= b;
        } else {
            b -= a;
        }
    }
    a
}
fn lcm(a: usize, b: usize) -> usize {
    (a * b) / gcd(a, b)
}

#[timed]
fn part2(input: &str) -> usize {
    //"lk" 3823, "zv" 4051, "sp" 3929, "xt" 3767
    let modules = parse(input);
    let last = modules.get("rx").unwrap().inputs[0];
    modules
        .get(last)
        .unwrap()
        .inputs
        .clone()
        .iter()
        .fold(1, |acc, x| {
            let mut tmp = 1;
            let mut modules = modules.clone();
            while !send_output_p2(&mut modules, "broadcaster", x) {
                tmp += 1
            }
            lcm(acc, tmp)
        })
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
    #[test]
    fn part1_test2() {
        let expected = 11687500;
        let result = part1(&get_test_input(InputType::Other("test2")));
        assert_eq!(result, expected);
    }
}
