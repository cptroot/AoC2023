use aoc_runner_derive::aoc_generator;
use aoc_runner_derive::aoc;

use anyhow::Result;
use anyhow::anyhow;

type Data = (Hand, usize);

type Hand = [Card; 5];
type HandWithJokers = [CardWithJoker; 5];

#[derive(Debug, Clone, Copy)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum Card {
    Ace,
    King,
    Queen,
    Jack,
    Ten,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}


#[aoc_generator(day7)]
fn input_generator(input: &str) -> Result<Vec<Data>> {
    let mut result = Vec::new();

    for line in input.lines() {
        if line.is_empty() { continue; }

        let (hand, bid) = line.split_once(" ").map_or(Err(anyhow!("Invalid line")), |a| Ok(a))?;

        let hand = hand.chars().take(5).map(char_to_card).collect::<Result<Vec<Card>>>()?;
        use std::convert::TryInto;
        let hand: [Card; 5] = hand.try_into().unwrap();

        let bid = bid.parse()?;

        result.push((hand, bid));
    }

    Ok(result)
}

fn char_to_card(c: char) -> Result<Card> {
    use Card::*;

    match c {
        'A' => Ok(Ace),
        'K' => Ok(King),
        'Q' => Ok(Queen),
        'J' => Ok(Jack),
        'T' => Ok(Ten),
        '9' => Ok(Nine),
        '8' => Ok(Eight),
        '7' => Ok(Seven),
        '6' => Ok(Six),
        '5' => Ok(Five),
        '4' => Ok(Four),
        '3' => Ok(Three),
        '2' => Ok(Two),
        _ => Err(anyhow!("Invalid card character")),
    }
}

#[aoc(day7, part1)]
fn solve_part1(input: &[Data]) -> usize {
    let mut hands: Vec<_> = input.iter()
        .map(|&(hand, bid)| {
            (hand, hand_type(&hand), bid)
        })
        .collect();

    hands.sort_by(|l, r| {
        l.1.cmp(&r.1).then(l.0.cmp(&r.0)).reverse()
    });

    hands.iter().enumerate()
        .map(|(i, (_hand, _hand_type, bid))| {
            let rank = i + 1;
            rank * bid
        })
        .sum()
}

const CARD_VALUES: [Card; 13] = [
    Card::Ace,
    Card::King,
    Card::Queen,
    Card::Jack,
    Card::Ten,
    Card::Nine,
    Card::Eight,
    Card::Seven,
    Card::Six,
    Card::Five,
    Card::Four,
    Card::Three,
    Card::Two,
];

fn hand_type(hand: &Hand) -> HandType {
    let mut counts: Vec<_> = CARD_VALUES.iter()
        .map(|card_value| {
            hand.iter().filter(|&card| card == card_value).count()
        })
        .filter(|&count| count > 0)
        .collect();

    counts.sort();

    use HandType::*;

    match counts.as_slice() {
        &[5] => FiveOfAKind,
        &[1, 4] => FourOfAKind,
        &[2, 3] => FullHouse,
        &[1, 1, 3] => ThreeOfAKind,
        &[1, 2, 2] => TwoPair,
        &[1, 1, 1, 2] => OnePair,
        &[1, 1, 1, 1, 1] => HighCard,
        _ => unreachable!()
    }
}

#[aoc(day7, part2)]
fn solve_part2(input: &[Data]) -> usize {
    let mut hands: Vec<_> = input.iter()
        .map(|&(hand, bid)| {
            ([
                hand[0].into(),
                hand[1].into(),
                hand[2].into(),
                hand[3].into(),
                hand[4].into(),
            ], bid)
        })
        .map(|(hand, bid): ([CardWithJoker; 5], usize)| {
            (hand, hand_type_with_jokers(&hand), bid)
        })
        .collect();

    hands.sort_by(|l, r| {
        l.1.cmp(&r.1).then(l.0.cmp(&r.0)).reverse()
    });

    hands.iter().enumerate()
        .map(|(i, (_hand, _hand_type, bid))| {
            let rank = i + 1;
            rank * bid
        })
        .sum()
}

#[derive(Debug, Clone, Copy)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum CardWithJoker {
    Ace,
    King,
    Queen,
    Ten,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
    Joker,
}

impl From<Card> for CardWithJoker {
    fn from(value: Card) -> Self {
        use CardWithJoker::*;
        match value {
            Card::Ace => Ace,
            Card::King => King,
            Card::Queen => Queen,
            Card::Ten => Ten,
            Card::Nine => Nine,
            Card::Eight => Eight,
            Card::Seven => Seven,
            Card::Six => Six,
            Card::Five => Five,
            Card::Four => Four,
            Card::Three => Three,
            Card::Two => Two,
            Card::Jack => Joker,
        }
    }
}

const CARD_VALUES_WITH_JOKERS: [CardWithJoker; 12] = [
    CardWithJoker::Ace,
    CardWithJoker::King,
    CardWithJoker::Queen,
    CardWithJoker::Ten,
    CardWithJoker::Nine,
    CardWithJoker::Eight,
    CardWithJoker::Seven,
    CardWithJoker::Six,
    CardWithJoker::Five,
    CardWithJoker::Four,
    CardWithJoker::Three,
    CardWithJoker::Two,
];

fn hand_type_with_jokers(hand: &HandWithJokers) -> HandType {
    let jokers = hand.iter().filter(|&card| card == &CardWithJoker::Joker).count();
    let mut counts: Vec<_> = CARD_VALUES_WITH_JOKERS.iter()
        .map(|card_value| {
            hand.iter().filter(|&card| card == card_value).count()
        })
        .filter(|&count| count > 0)
        .collect();

    counts.sort();

    use HandType::*;

    match (jokers, counts.as_slice()) {
        (0, &[5]) => FiveOfAKind,
        (0, &[1, 4]) => FourOfAKind,
        (0, &[2, 3]) => FullHouse,
        (0, &[1, 1, 3]) => ThreeOfAKind,
        (0, &[1, 2, 2]) => TwoPair,
        (0, &[1, 1, 1, 2]) => OnePair,
        (0, &[1, 1, 1, 1, 1]) => HighCard,
        (1, &[4]) => FiveOfAKind,
        (1, &[1, 3]) => FourOfAKind,
        (1, &[2, 2]) => FullHouse,
        (1, &[1, 1, 2]) => ThreeOfAKind,
        (1, &[1, 1, 1, 1]) => OnePair,
        (2, &[3]) => FiveOfAKind,
        (2, &[1, 2]) => FourOfAKind,
        (2, &[1, 1, 1]) => ThreeOfAKind,
        (3, &[2]) => FiveOfAKind,
        (3, &[1, 1]) => FourOfAKind,
        (4, &[1]) => FiveOfAKind,
        (5, &[]) => FiveOfAKind,
        _ => unreachable!()
    }
}

#[cfg(test)]
mod test {
    const TEST_INPUT: &'static str =
r#"
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
"#;
    #[test]
    fn test_part1_example() {
        let input = super::input_generator(TEST_INPUT).unwrap();
        let result = super::solve_part1(&input);

        assert_eq!(result, 6440);
    }

    #[test]
    fn test_part2_example() {
        let input = super::input_generator(TEST_INPUT).unwrap();
        let result = super::solve_part2(&input);

        assert_eq!(result, 5905);
    }
}
