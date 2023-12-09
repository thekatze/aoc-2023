// const INPUT: &str = include_str!("input.txt");
const INPUT: &str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45
";

#[derive(Debug, Clone)]
struct Sequence(Vec<i32>);

impl Sequence {
    fn get_next_number(self) -> i32 {
        let derivatives = self.get_all_derivatives();

        let prediction = derivatives.iter().fold(0, |prediction, derivative| {
            let last_number = derivative.0.last().expect("list empty, thats no good");
            prediction + last_number
        });

        prediction
    }

    fn get_all_derivatives(self) -> Vec<Sequence> {
        let mut derivatives = vec![self];
        Sequence::get_all_derivatives_rec(&mut derivatives);
        derivatives
    }

    fn get_all_derivatives_rec(sequences: &mut Vec<Sequence>) {
        let last_sequence = sequences.last().expect("list empty, thats no good");
        let derivative = last_sequence.get_derivative();
        let all_zeros = derivative.0.iter().all(|&n| n == 0);

        sequences.push(derivative);

        if !all_zeros {
            Sequence::get_all_derivatives_rec(sequences);
        }
    }

    fn get_derivative(&self) -> Sequence {
        Sequence(
            self.0
                .windows(2)
                .map(|window| window[1] - window[0])
                .collect(),
        )
    }
}

fn main() {
    let sum_of_next_numbers = INPUT
        .lines()
        .map(|line| {
            Sequence(
                line.split_whitespace()
                    .map(|number| number.parse::<i32>().expect("not a number"))
                    .collect(),
            )
            .get_next_number()
        })
        .sum::<i32>();

    dbg!(sum_of_next_numbers);
}
