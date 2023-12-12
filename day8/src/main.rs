use std::collections::HashMap;

fn main() {
    let input = include_str!("../input.txt");
    let output = part_1(input);
    println!("{output}");

    let output = part_2(input);
    println!("{output}");
}

fn part_1(input: &str) -> usize {
    let input = Input::parse(input);
    input.calc_steps("AAA", |k| k == "ZZZ")
}

fn part_2(input: &str) -> usize {
    let input = Input::parse(input);

    let mut key_steps = Vec::new();
    for key in input.start.iter() {
        let steps = input.calc_steps(key, |k| k.ends_with('Z'));
        key_steps.push(steps);
    }

    key_steps
        .into_iter()
        .reduce(|a, b| num::integer::lcm(a, b))
        .unwrap()
}

fn parse_directions(line: &str) -> Vec<Direction> {
    line.chars()
        .take_while(|&c| c == 'L' || c == 'R')
        .map(|c| {
            if c == 'L' {
                Direction::Left
            } else {
                Direction::Right
            }
        })
        .collect()
}

fn parse_mapping(line: &str) -> (&str, &str, &str) {
    let mut line = line.split(" = ");
    let key = line.next().unwrap();
    let pair = line.next().unwrap();
    let mut pair = pair[1..pair.len() - 1].split(", ");
    let left = pair.next().unwrap();
    let right = pair.next().unwrap();
    (key, left, right)
}

struct Input<'a> {
    directions: Vec<Direction>,
    left: HashMap<&'a str, &'a str>,
    right: HashMap<&'a str, &'a str>,
    start: Vec<&'a str>,
}

impl<'a> Input<'a> {
    pub fn parse(input: &'a str) -> Self {
        let mut lines = input.lines();
        let directions = parse_directions(lines.next().unwrap());
        lines.next(); // skip the empty line

        let mut start = Vec::new();
        let mut left_map = HashMap::new();
        let mut right_map = HashMap::new();

        while let Some(line) = lines.next() {
            let (key, left, right) = parse_mapping(line);
            left_map.insert(key, left);
            right_map.insert(key, right);
            if key.ends_with('A') {
                start.push(key);
            }
        }

        Self {
            directions,
            left: left_map,
            right: right_map,
            start,
        }
    }

    fn calc_steps(&self, start: &str, stop: impl Fn(&str) -> bool) -> usize {
        let mut steps = 0;
        let mut curr = start;
        for direction in self.directions.iter().cycle() {
            steps += 1;
            let map = direction.choose(&self.left, &self.right);
            curr = map.get(curr).unwrap();
            if stop(curr) {
                break;
            }
        }
        steps
    }
}

#[derive(Clone, Copy)]
enum Direction {
    Left,
    Right,
}

impl Direction {
    fn choose<T>(self, left: T, right: T) -> T {
        match self {
            Self::Left => left,
            Self::Right => right,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_basic() {
        let input = r#"RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)"#;
        let steps = part_1(input);
        assert_eq!(steps, 2);
    }

    #[test]
    fn part_1_multi() {
        let input = r#"LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)"#;
        let steps = part_1(input);
        assert_eq!(steps, 6);
    }

    #[test]
    fn part_2_basic() {
        let input = r#"LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)"#;
        let steps = part_2(input);
        assert_eq!(steps, 6);
    }
}
