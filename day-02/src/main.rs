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
    pulls: Box<[[u32; Colour::AMOUNT_OF_COLOURS as usize]]>,
}

impl TryFrom<&str> for Game {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let (_, pulls) = value.split_once(": ").ok_or("invalid format")?;

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
            pulls: pulls.into_boxed_slice(),
        })
    }
}

impl Game {
    fn calculate_power(&self) -> u64 {
        let max_needed_balls = self.pulls.iter().fold(
            [0; Colour::AMOUNT_OF_COLOURS as usize],
            |mut max_needed_balls, needed_balls| {
                for i in 0..Colour::AMOUNT_OF_COLOURS as usize {
                    if max_needed_balls[i] < needed_balls[i] {
                        max_needed_balls[i] = needed_balls[i];
                    }
                }

                max_needed_balls
            },
        );

        max_needed_balls
            .iter()
            .fold(1_u64, |acc, &curr| acc * curr as u64)
    }
}

fn main() {
    let sum_of_powers = INPUT
        .lines()
        .map(|line| {
            Game::try_from(line)
                .expect("invalid input")
                .calculate_power()
        })
        .sum::<u64>();

    dbg!(sum_of_powers);
}
