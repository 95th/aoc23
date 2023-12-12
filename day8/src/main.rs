use std::collections::HashMap;

fn main() {
    let input = include_str!("../input.txt");
    let output = part_1(input);
    println!("{output}");
}

fn part_1(input: &str) -> usize {
    let mut lines = input.lines();
    let directions = parse_directions(lines.next().unwrap());
    lines.next(); // skip the empty line

    let mut mappings = HashMap::new();
    while let Some(line) = lines.next() {
        let (key, left, right) = parse_mapping(line);
        mappings.insert(key, (left, right));
    }

    let mut steps = 0;

    let mut curr = "AAA";
    for dir in directions {
        steps += 1;
        let (l, r) = mappings.get(curr).unwrap();
        curr = dir.choose(l, r);
        if curr == "ZZZ" {
            break;
        }
    }

    steps
}

fn parse_directions(line: &str) -> impl Iterator<Item = Direction> + '_ {
    line.chars()
        .take_while(|&c| c == 'L' || c == 'R')
        .map(|c| {
            if c == 'L' {
                Direction::Left
            } else {
                Direction::Right
            }
        })
        .cycle()
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
}
