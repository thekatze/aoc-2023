const INPUT: &str = include_str!("input.txt");

#[derive(Debug)]
#[repr(u8)]
enum Colour {
    Red = 0,
    Green = 1,
    Blue = 2,
    AMOUNT_OF_COLOURS,
}

struct Game {
    id: u32,
    pulls: Box<[[u32; Colour::AMOUNT_OF_COLOURS as usize]]>,
}

impl TryFrom<&str> for Game {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let (game, pulls) = value.split_once(": ").ok_or("invalid format")?;

        let (_, game_id) = game.split_once(' ').ok_or("invalid format")?;
        let game_id = game_id.parse().map_err(|_| "invalid format")?;

        let pulls = pulls
            .split("; ")
            .map(
                |pull| -> Result<[u32; Colour::AMOUNT_OF_COLOURS as usize], &'static str> {
                    pull.split(", ")
                        .map(|balls| {
                            let (amount, colour) = balls.split_once(' ').ok_or("invalid format")?;
                            let amount: u32 = amount.parse().map_err(|_| "invalid format")?;
                            let colour = match colour {
                                "red" => Colour::Red,
                                "green" => Colour::Green,
                                "blue" => Colour::Blue,
                                _ => return Err("invalid format"),
                            };

                            Ok((colour, amount))
                        })
                        .fold(Ok([0; Colour::AMOUNT_OF_COLOURS as usize]), |acc, curr| {
                            let (mut acc, curr) = (acc?, curr?);
                            acc[curr.0 as usize] = curr.1;

                            Ok(acc)
                        })
                },
            )
            .collect::<Result<Vec<_>, &'static str>>()?;

        Ok(Self {
            id: game_id,
            pulls: pulls.into_boxed_slice(),
        })
    }
}

impl Game {
    fn is_plausible_with(
        &self,
        available_balls: &[u32; Colour::AMOUNT_OF_COLOURS as usize],
    ) -> bool {
        !self.pulls.iter().any(|pull| {
            pull.iter()
                .zip(available_balls.iter())
                .any(|(pulled_balls, available_balls)| pulled_balls > available_balls)
        })
    }
}

fn main() {
    let sum_of_ids = INPUT
        .lines()
        .map(|line| Game::try_from(line).expect("invalid input"))
        .filter(|game| game.is_plausible_with(&[12, 13, 14]))
        .map(|game| game.id)
        .sum::<u32>();

    dbg!(sum_of_ids);
}
