const INPUT: &str = include_str!("input.txt");

#[derive(Debug, Clone)]
struct Sequence(Vec<i32>);

impl Sequence {
    fn get_next_number(self) -> i32 {
        let mut derivatives = self.get_all_derivatives();
        derivatives.reverse();

        let prediction = derivatives.iter().skip(1).fold(0, |prediction, derivative| {
            let first_number = derivative.0.first().expect("list empty, thats no good");
            first_number - prediction
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
    let sum_of_prev_numbers = INPUT
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

    dbg!(sum_of_prev_numbers);
}
