use std::collections::HashSet;

fn main() {
    let input = include_str!("../input.txt");
    let sum = part_1(input);
    println!("Part 1: {sum}");
}

fn part_1(input: &str) -> usize {
    let mut sum = 0;
    for line in input.lines() {
        let card = Card::parse(line).unwrap();
        sum += card.points();
    }
    sum
}

struct Card {
    id: usize,
    winning: HashSet<usize>,
    received: HashSet<usize>,
}

impl Card {
    pub fn parse(line: &str) -> Option<Self> {
        let line = line.strip_prefix("Card ")?;
        let mut split = line.trim().split(':');
        let id = split.next()?.parse().ok()?;
        let numbers = split.next()?;
        let mut split = numbers.split('|');
        let winning = split.next()?;
        let received = split.next()?;
        Some(Self {
            id,
            winning: parse_numbers(winning),
            received: parse_numbers(received),
        })
    }

    pub fn points(&self) -> usize {
        let count = self.winning.intersection(&self.received).count();
        if count > 0 {
            2_usize.pow(count as u32 - 1)
        } else {
            0
        }
    }
}

fn parse_numbers(input: &str) -> HashSet<usize> {
    input
        .trim()
        .split(' ')
        .filter(|s| !s.is_empty())
        .map(|s| s.parse().unwrap())
        .collect()
}
