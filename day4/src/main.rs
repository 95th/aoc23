use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("../input.txt");
    let sum = part_1(input);
    println!("Part 1: {sum}");

    let sum = part_2(input);
    println!("Part 2: {sum}");
}

fn part_1(input: &str) -> usize {
    let mut sum = 0;
    for line in input.lines() {
        let card = Card::parse(line).unwrap();
        sum += card.points();
    }
    sum
}

fn part_2(input: &str) -> usize {
    let mut copies = HashMap::<usize, usize>::new();
    let mut card_count = 0;
    for line in input.lines() {
        card_count += 1;
        let card = Card::parse(line).unwrap();
        let matching = card.matching_numbers();
        let card_copies = copies.get(&card.id).copied().unwrap_or(0);
        for i in 0..matching {
            *copies.entry(card.id + i + 1).or_insert(0) += 1 + card_copies;
        }
    }
    card_count + copies.values().sum::<usize>()
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

    pub fn matching_numbers(&self) -> usize {
        self.winning.intersection(&self.received).count()
    }

    pub fn points(&self) -> usize {
        let count = self.matching_numbers();
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let input = r#"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"#;
        let sum = part_2(input);
        assert_eq!(sum, 30);
    }
}
