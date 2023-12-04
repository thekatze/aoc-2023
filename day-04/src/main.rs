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
    fn get_winning_numbers_count(&self) -> u32 {
        self.numbers
            .iter()
            .filter(|&n| self.winning_numbers.contains(n))
            .count() as u32
    }
}

fn main() {
    let sum = INPUT
        .lines()
        .map(|line| {
            ScratchCard::try_from(line)
                .expect("input invalid")
                .get_winning_numbers_count()
        })
        .filter(|&count| count > 0)
        .map(|count| 2_u64.pow(count - 1))
        .sum::<u64>();

    dbg!(sum);
}
