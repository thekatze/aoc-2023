const INPUT: &str = include_str!("input.txt");

const WINNING_NUMBERS_COUNT: usize = 10;
const NUMBERS_COUNT: usize = 25;

struct ScratchCard {
    winning_numbers: [u8; WINNING_NUMBERS_COUNT],
    numbers: [u8; NUMBERS_COUNT],
}

impl TryFrom<&str> for ScratchCard {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let (_, numbers) = value.split_once(": ").ok_or("invalid input")?;
        let (winning_numbers, numbers) = numbers.split_once(" | ").ok_or("invalid input")?;

        let winning_numbers = winning_numbers
            .split_whitespace()
            .map(|n| n.parse())
            .collect::<Result<Vec<u8>, _>>()
            .map_err(|_| "invalid input")?;

        let numbers = numbers
            .split_whitespace()
            .map(|n| n.parse())
            .collect::<Result<Vec<u8>, _>>()
            .map_err(|_| "invalid input")?;

        Ok(Self {
            numbers: numbers.try_into().expect("expected other length"),
            winning_numbers: winning_numbers.try_into().expect("expected other length"),
        })
    }
}

impl ScratchCard {
    fn get_winning_numbers_count(&self) -> u64 {
        self.numbers
            .iter()
            .filter(|&n| self.winning_numbers.contains(n))
            .count() as u64
    }
}

fn main() {
    let winning_amounts = INPUT
        .lines()
        .map(|line| {
            ScratchCard::try_from(line)
                .expect("input invalid")
                .get_winning_numbers_count()
        })
        .collect::<Vec<_>>();

    let mut won_cards = Vec::<u64>::with_capacity(winning_amounts.len());

    for card in winning_amounts.iter().rev() {
        let card_value = won_cards.iter().rev().take(*card as usize).sum::<u64>();
        won_cards.push(card_value + 1);
    }

    let amount_cards = won_cards.iter().sum::<u64>();
    dbg!(amount_cards);
}
