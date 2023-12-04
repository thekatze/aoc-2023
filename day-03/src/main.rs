const INPUT: &str = include_str!("input.txt");

#[derive(Debug)]
struct Character {
    at: usize,
}

#[derive(Debug)]
struct Number {
    from: usize,
    to: usize,
    value: u64,
}

#[derive(Debug)]
struct Schematic {
    width: usize,
    numbers: Box<[Number]>,
    symbols: Box<[Character]>,
}

struct NumberBuilder {
    from: usize,
    value: String,
}

impl TryFrom<&str> for Schematic {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut numbers = Vec::new();
        let mut symbols = Vec::new();

        let width = value.lines().next().ok_or("no data")?.len();

        let mut current_number: Option<NumberBuilder> = None;

        for (y, line) in value.lines().enumerate() {
            for (x, character) in line.chars().enumerate() {
                let mut finish_number = || {
                    if let Some(number) = current_number.as_mut() {
                        numbers.push(Number {
                            to: y * width + x - 1,
                            from: number.from,
                            value: number.value.parse().expect("number to be numeric haha"),
                        });
                    };

                    current_number = None;
                };

                match character {
                    '.' => {
                        finish_number();
                    }
                    '0'..='9' => {
                        if let Some(number) = current_number.as_mut() {
                            number.value.push(character);
                        } else {
                            current_number = Some(NumberBuilder {
                                from: y * width + x,
                                value: String::from(character),
                            })
                        }
                    }
                    '*' => {
                        symbols.push(Character { at: y * width + x });

                        finish_number();
                    }
                    _ => {}
                }
            }
        }

        Ok(Self {
            numbers: numbers.into_boxed_slice(),
            symbols: symbols.into_boxed_slice(),
            width,
        })
    }
}

impl Schematic {
    fn sum_gear_ratios(&self) -> u64 {
        let indices_around = |index: usize| -> [usize; 8] {
            [
                index - 1 - self.width,
                index - self.width,
                index + 1 - self.width,
                index - 1,
                index + 1,
                index - 1 + self.width,
                index + self.width,
                index + 1 + self.width,
            ]
        };

        self.symbols
            .iter()
            .filter_map(|symbol| {
                let indices = indices_around(symbol.at);
                let adjacent_numbers = self.numbers.iter().filter(|number| {
                    indices
                        .iter()
                        .any(|&index| index >= number.from && index <= number.to)
                }).collect::<Vec<_>>();

                if adjacent_numbers.len() == 2 {
                    Some(adjacent_numbers.iter().map(|n| n.value).product::<u64>())
                } else {
                    None
                }
            })
            .sum::<u64>()
    }
}

fn main() {
    let schematic = Schematic::try_from(INPUT).expect("input couldnt be parsed");
    dbg!(schematic.sum_gear_ratios());
}
