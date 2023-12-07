use itertools::Itertools;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum Card {
    Joker(Box<Card>),
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    T,
    J,
    Q,
    K,
    A,
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Card {
    // Cheating, comparing by the enum order, ignoring the value of Joker()
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe { *(self as *const Self as *const u8) }
            .cmp(&unsafe { *(other as *const Self as *const u8) })
    }
}

impl From<char> for Card {
    fn from(value: char) -> Self {
        match value {
            '2' => Card::Two,
            '3' => Card::Three,
            '4' => Card::Four,
            '5' => Card::Five,
            '6' => Card::Six,
            '7' => Card::Seven,
            '8' => Card::Eight,
            '9' => Card::Nine,
            'T' => Card::T,
            'J' => Card::J,
            'Q' => Card::Q,
            'K' => Card::K,
            'A' => Card::A,
            _ => unimplemented!(),
        }
    }
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
pub enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfKind,
    FullHouse,
    FourOfKind,
    FiveOfKind,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Hand {
    cards: [Card; 5],
    wager: i32,
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.grade()
            .cmp(&other.grade())
            .then_with(|| self.cards.cmp(&other.cards))
            .then_with(|| self.wager.cmp(&other.wager))
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Hand {
    fn grade(&self) -> HandType {
        let counts = self
            .cards
            .iter()
            .map(|card| {
                if let Card::Joker(inner) = card {
                    return &**inner;
                }
                card
            })
            .counts();
        let counts = counts.values().sorted().collect_vec();

        match counts {
            x if x == vec![&5] => HandType::FiveOfKind,
            x if x == vec![&1, &4] => HandType::FourOfKind,
            x if x == vec![&2, &3] => HandType::FullHouse,
            x if x == vec![&1, &1, &3] => HandType::ThreeOfKind,
            x if x == vec![&1, &2, &2] => HandType::TwoPair,
            x if x == vec![&1, &1, &1, &2] => HandType::OnePair,
            _ => HandType::HighCard,
        }
    }
}

#[derive(Debug)]
pub struct Hands(Vec<Hand>);

impl Hands {
    fn new() -> Self {
        Self(Vec::new())
    }
}

impl From<(&str, &str)> for Hand {
    fn from((cards, wager): (&str, &str)) -> Self {
        let mut cards = cards.chars().map(|c: char| -> Card { c.into() }).take(5);

        let cards = [
            cards.next().unwrap(),
            cards.next().unwrap(),
            cards.next().unwrap(),
            cards.next().unwrap(),
            cards.next().unwrap(),
        ];

        let wager: i32 = wager.parse().unwrap();

        Hand { cards, wager }
    }
}

pub fn parse<'a>(data: &'a str) -> Hands {
    data.lines().fold(Hands::new(), |mut acc, line| {
        let mut hands = line.split_whitespace();
        let cards = hands.next().unwrap();
        let wager = hands.next().unwrap();
        let hand: Hand = (cards, wager).into();
        acc.0.push(hand);
        acc
    })
}

pub fn part1<'a>(input: Hands) -> i32 {
    input
        .0
        .iter()
        .sorted()
        .enumerate()
        .map(|(idx, hand)| ((idx + 1) as i32) * hand.wager)
        .sum()
}

pub fn part2<'a>(input: Hands) -> i32 {
    input
        .0
        .into_iter()
        .map(|hand| {
            if !hand.cards.contains(&Card::J) {
                return hand;
            }

            let counts = hand.cards.iter().filter(|card| card != &&Card::J).counts();

            if counts.is_empty() {
                return Hand {
                    cards: [
                        Card::Joker(Box::new(Card::A)),
                        Card::Joker(Box::new(Card::A)),
                        Card::Joker(Box::new(Card::A)),
                        Card::Joker(Box::new(Card::A)),
                        Card::Joker(Box::new(Card::A)),
                    ],
                    wager: hand.wager,
                };
            }

            let (_, most_common) = counts
                .into_iter()
                .map(|(card, count)| (count, card))
                .max()
                .unwrap();

            let mut new_cards = hand.cards.clone();
            for card in new_cards.iter_mut() {
                if *card == Card::J {
                    *card = Card::Joker(Box::new(most_common.clone()));
                }
            }

            return Hand {
                cards: new_cards,
                wager: hand.wager,
            };
        })
        .sorted()
        .enumerate()
        .map(|(idx, hand)| ((idx + 1) as i32) * hand.wager)
        .sum()
}

#[cfg(test)]
mod tests {
    use util::generate_test;

    const SAMPLE_INPUT: &str = r#"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"#;

    #[test]
    fn test_sample1() {
        let data = parse(&SAMPLE_INPUT);
        assert_eq!(part1(data), 6440);
    }

    #[test]
    fn test_sample2() {
        let data = parse(&SAMPLE_INPUT);
        assert_eq!(part2(data), 5905);
    }

    const BETTER_SAMPLE_INPUT: &str = r#"2345A 1
Q2KJJ 13
Q2Q2Q 19
T3T3J 17
T3Q33 11
2345J 3
J345A 2
32T3K 5
T55J5 29
KK677 7
KTJJT 34
QQQJA 31
JJJJJ 37
JAAAA 43
AAAAJ 59
AAAAA 61
2AAAA 23
2JJJJ 53
JJJJ2 41"#;

    #[test]
    fn test_better_sample1() {
        let data = parse(&BETTER_SAMPLE_INPUT);
        assert_eq!(part1(data), 6592);
    }

    #[test]
    fn test_better_sample2() {
        let data = parse(&BETTER_SAMPLE_INPUT);
        assert_eq!(part2(data), 6839);
    }

    #[test]
    fn test_part2_joker_sort() {
        let data = parse(
            r#"JJJJJ 37
JJJJ2 41"#,
        );
        assert_eq!(part2(data), 119);
    }

    #[test]
    fn test_part2_joker_cmp() {
        let a = Hand {
            cards: [
                Card::Joker(Box::new(Card::A)),
                Card::Joker(Box::new(Card::A)),
                Card::Joker(Box::new(Card::A)),
                Card::Joker(Box::new(Card::A)),
                Card::Joker(Box::new(Card::A)),
            ],
            wager: 37,
        };

        let b = Hand {
            cards: [
                Card::Joker(Box::new(Card::Two)),
                Card::Joker(Box::new(Card::Two)),
                Card::Joker(Box::new(Card::Two)),
                Card::Joker(Box::new(Card::Two)),
                Card::Two,
            ],
            wager: 41,
        };

        assert_eq!(a.cmp(&b), std::cmp::Ordering::Less);
    }

    #[test]
    fn test_part2_joker_cmp_sort() {
        let a = Hand {
            cards: [
                Card::Joker(Box::new(Card::A)),
                Card::Joker(Box::new(Card::A)),
                Card::Joker(Box::new(Card::A)),
                Card::Joker(Box::new(Card::A)),
                Card::Joker(Box::new(Card::A)),
            ],
            wager: 37,
        };

        let b = Hand {
            cards: [
                Card::Joker(Box::new(Card::Two)),
                Card::Joker(Box::new(Card::Two)),
                Card::Joker(Box::new(Card::Two)),
                Card::Joker(Box::new(Card::Two)),
                Card::Two,
            ],
            wager: 41,
        };

        assert_eq!(
            vec![a.clone(), b.clone()].iter().sorted().next(),
            Some(a.clone()).as_ref()
        );
        assert_eq!(
            vec![b.clone(), a.clone()].iter().sorted().next(),
            Some(a).as_ref()
        );
    }

    generate_test! { 2023, 7, 1, 255048101}
    generate_test! { 2023, 7, 2, 253718286}
}
