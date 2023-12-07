const INPUT: &str = include_str!("input.txt");

use card::*;

fn main() {
    let mut games = INPUT
        .lines()
        .map(|line| Game::try_from(line).expect("invalid input"))
        .collect::<Vec<_>>();

    games.sort_by_key(|key| key.cards);

    let total_winnings = games
        .iter()
        .enumerate()
        .map(|(rank, game)| game.bid * (rank + 1) as u64)
        .sum::<u64>();

    dbg!(total_winnings);
}

#[derive(Debug)]
struct Game {
    cards: Cards,
    bid: u64,
}

#[derive(Clone, Copy, Debug, PartialOrd, Ord, PartialEq, Eq)]
enum Hand {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl Hand {
    fn from_cards(cards: &[Card; 5]) -> Hand {
        let mut counts = cards
            .iter()
            .fold([0; (Card::A as usize) + 1], |mut counts, current| {
                counts[*current as usize] += 1;
                counts
            });

        let jokers = counts[Card::J as usize];
        counts[Card::J as usize] = 0;

        counts.sort_by(|a, b| b.cmp(a));

        match (counts[0] + jokers, counts[1]) {
            (5, _) => Hand::FiveOfAKind,
            (4, _) => Hand::FourOfAKind,
            (3, 2) => Hand::FullHouse,
            (3, 1) => Hand::ThreeOfAKind,
            (2, 2) => Hand::TwoPair,
            (2, _) => Hand::OnePair,
            (1, _) => Hand::HighCard,
            _ => unreachable!("no match found"),
        }
    }
}

impl TryFrom<&str> for Game {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let (cards, bid) = value.split_once(' ').ok_or("not separated by space")?;
        let cards = cards
            .chars()
            .map(|c| Card::try_from(c))
            .collect::<Result<Vec<_>, _>>()?;
        let bid = bid.parse().map_err(|_| "bid not a number")?;

        let cards: [Card; 5] = cards.try_into().map_err(|_| "not five cards")?;
        let hand = Hand::from_cards(&cards);

        Ok(Game {
            cards: Cards(hand, cards),
            bid,
        })
    }
}

#[derive(Clone, Copy, Debug, PartialOrd, Ord, PartialEq, Eq)]
struct Cards(Hand, [Card; 5]);

impl Cards {}

mod card {
    #[derive(Clone, Copy, Debug, PartialOrd, Ord, PartialEq, Eq)]
    pub enum Card {
        J,
        Two,
        Three,
        Four,
        Five,
        Six,
        Seven,
        Eight,
        Nine,
        T,
        Q,
        K,
        A,
    }

    impl TryFrom<char> for Card {
        type Error = &'static str;

        fn try_from(value: char) -> Result<Self, Self::Error> {
            match value {
                'J' => Ok(Card::J),
                '2' => Ok(Card::Two),
                '3' => Ok(Card::Three),
                '4' => Ok(Card::Four),
                '5' => Ok(Card::Five),
                '6' => Ok(Card::Six),
                '7' => Ok(Card::Seven),
                '8' => Ok(Card::Eight),
                '9' => Ok(Card::Nine),
                'T' => Ok(Card::T),
                'Q' => Ok(Card::Q),
                'K' => Ok(Card::K),
                'A' => Ok(Card::A),
                _ => Err("Invalid card"),
            }
        }
    }
}
