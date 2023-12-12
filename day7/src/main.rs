fn main() {
    let input = include_str!("../input.txt");
    let output = part_1(input);
    println!("Output: {output}");

    let output = part_2(input);
    println!("Output: {output}");
}

fn part_1(input: &str) -> usize {
    let mut deck = parse_deck(input);
    deck.sort_by_key(|(hand, _)| *hand);

    let mut total = 0;
    for (i, (_hand, bid)) in deck.into_iter().enumerate() {
        total += bid * (i + 1);
    }
    total
}

fn part_2(input: &str) -> usize {
    let mut deck = parse_deck2(input);
    deck.sort_by_key(|(hand, _)| *hand);

    let mut total = 0;
    for (i, (_hand, bid)) in deck.into_iter().enumerate() {
        total += bid * (i + 1);
    }
    total
}

fn parse_deck(input: &str) -> Vec<(Hand, usize)> {
    input.lines().map(parse_deck_line).collect()
}

fn parse_deck_line(line: &str) -> (Hand, usize) {
    let mut line = line.split_whitespace();
    let hand = Hand::from_str(line.next().unwrap());
    let bid = line.next().unwrap().parse().unwrap();
    (hand, bid)
}

fn parse_deck2(input: &str) -> Vec<(Hand, usize)> {
    input.lines().map(parse_deck_line2).collect()
}

fn parse_deck_line2(line: &str) -> (Hand, usize) {
    let mut line = line.split_whitespace();
    let hand = Hand::from_str_part_2(line.next().unwrap());
    let bid = line.next().unwrap().parse().unwrap();
    (hand, bid)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Hand {
    kind: HandKind,
    cards: [u8; 5],
}

impl Hand {
    fn hand_kind(freq_map: [usize; 6]) -> HandKind {
        if freq_map[5] == 1 {
            HandKind::FiveOfAKind
        } else if freq_map[4] == 1 {
            HandKind::FourOfAKind
        } else if freq_map[3] == 1 && freq_map[2] == 1 {
            HandKind::FullHouse
        } else if freq_map[3] == 1 {
            HandKind::ThreeOfAKind
        } else if freq_map[2] == 2 {
            HandKind::TwoPair
        } else if freq_map[2] == 1 {
            HandKind::OnePair
        } else {
            HandKind::HighCard
        }
    }

    fn kind(cards: [u8; 5]) -> HandKind {
        let mut map = [0; 13];
        for c in cards {
            map[c as usize] += 1;
        }
        let mut freq_map = [0; 6];
        for v in map {
            freq_map[v] += 1;
        }
        Self::hand_kind(freq_map)
    }

    fn kind_part_2(cards: [u8; 5]) -> HandKind {
        let mut map = [0; 13];
        for c in cards {
            map[c as usize] += 1;
        }
        let j = map[0];
        if j == 5 {
            return HandKind::FiveOfAKind;
        }

        let mut freq_map = [0; 6];
        for v in map {
            freq_map[v] += 1;
        }
        let hand_kind = Self::hand_kind(freq_map);
        hand_kind.joker_upgrade(j)
    }

    fn from_str(s: &str) -> Self {
        let mut cards = [0; 5];
        let mut chars = s.char_indices();
        while let Some((i, c)) = chars.next() {
            cards[i] = parse_card(c);
        }
        let kind = Hand::kind(cards);
        Hand { kind, cards }
    }

    fn from_str_part_2(s: &str) -> Self {
        let mut cards = [0; 5];
        let mut chars = s.char_indices();
        while let Some((i, c)) = chars.next() {
            cards[i] = parse_card_part_2(c);
        }
        let kind = Hand::kind_part_2(cards);
        Hand { kind, cards }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
enum HandKind {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl HandKind {
    pub fn joker_upgrade(self, jokers: usize) -> Self {
        if jokers == 0 {
            return self;
        }

        use HandKind::*;
        match self {
            HighCard => OnePair,
            OnePair => ThreeOfAKind,
            TwoPair => {
                if jokers == 2 {
                    FourOfAKind
                } else {
                    FullHouse
                }
            }
            ThreeOfAKind => FourOfAKind,
            FullHouse => FiveOfAKind,
            FourOfAKind => FiveOfAKind,
            FiveOfAKind => FiveOfAKind,
        }
    }
}

fn parse_card(c: char) -> u8 {
    match c {
        '2' => 0,
        '3' => 1,
        '4' => 2,
        '5' => 3,
        '6' => 4,
        '7' => 5,
        '8' => 6,
        '9' => 7,
        'T' => 8,
        'J' => 9,
        'Q' => 10,
        'K' => 11,
        'A' => 12,
        _ => panic!("Unknown card"),
    }
}

fn parse_card_part_2(c: char) -> u8 {
    match c {
        'J' => 0,
        '2' => 1,
        '3' => 2,
        '4' => 3,
        '5' => 4,
        '6' => 5,
        '7' => 6,
        '8' => 7,
        '9' => 8,
        'T' => 9,
        'Q' => 10,
        'K' => 11,
        'A' => 12,
        _ => panic!("Unknown card"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"#;

    #[test]
    fn part1_basic() {
        let output = part_1(INPUT);
        assert_eq!(output, 6440);
    }

    #[test]
    fn part2_basic() {
        let output = part_2(INPUT);
        assert_eq!(output, 5905);
    }
}
