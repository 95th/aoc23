use std::str::FromStr;

fn main() {
    let input = include_str!("../input.txt");
    let output = part_1(input);
    println!("Output: {output}");
}

fn part_1(input: &str) -> usize {
    let mut deck = parse_deck(input);
    deck.sort_by_key(|(hand, _bid)| *hand);

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
    let hand = line.next().unwrap().parse().unwrap();
    let bid = line.next().unwrap().parse().unwrap();
    (hand, bid)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Hand {
    kind: HandKind,
    cards: [Card; 5],
}

impl Hand {
    fn kind(cards: [Card; 5]) -> HandKind {
        let mut map = [0; Card::CardA as usize + 1];
        for c in cards {
            map[c as usize] += 1;
        }
        let mut map2 = [0; 6];
        for v in map {
            map2[v] += 1;
        }
        if map2[5] == 1 {
            return HandKind::FiveOfAKind;
        }
        if map2[4] == 1 {
            return HandKind::FourOfAKind;
        }
        if map2[3] == 1 && map2[2] == 1 {
            return HandKind::FullHouse;
        }
        if map2[3] == 1 {
            return HandKind::ThreeOfAKind;
        }
        if map2[2] == 2 {
            return HandKind::TwoPair;
        }
        if map2[2] == 1 {
            return HandKind::OnePair;
        }
        return HandKind::HighCard;
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

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
enum Card {
    Card2,
    Card3,
    Card4,
    Card5,
    Card6,
    Card7,
    Card8,
    Card9,
    CardT,
    CardJ,
    CardQ,
    CardK,
    CardA,
}

impl TryFrom<char> for Card {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        use Card::*;
        let card = match value {
            '2' => Card2,
            '3' => Card3,
            '4' => Card4,
            '5' => Card5,
            '6' => Card6,
            '7' => Card7,
            '8' => Card8,
            '9' => Card9,
            'T' => CardT,
            'J' => CardJ,
            'Q' => CardQ,
            'K' => CardK,
            'A' => CardA,
            _ => return Err("Unknown card"),
        };
        Ok(card)
    }
}

impl FromStr for Hand {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut cards = [Card::Card2; 5];
        let mut chars = s.char_indices();
        while let Some((i, c)) = chars.next() {
            let card = Card::try_from(c)?;
            cards[i] = card;
        }
        let kind = Hand::kind(cards);
        Ok(Hand { kind, cards })
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
}
