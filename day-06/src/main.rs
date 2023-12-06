const INPUT: &str = include_str!("input.txt");
// const INPUT: &str = "Time:      7  15   30
// Distance:  9  40  200";

struct Race {
    time: u32,
    record: u32,
}

impl Race {
    fn count_record_time_configurations(&self) -> u32 {
        (0..=self.time)
            .map(|hold_time| {
                let speed = hold_time;
                let remaining_time = self.time - hold_time;

                speed * remaining_time
            })
            .filter(|distance| distance > &self.record)
            .count() as u32
    }
}

fn main() {
    let parsed = INPUT
        .lines()
        .map(|line| {
            let (_, numbers) = line.split_once(":").expect("invalid input");
            numbers
                .split_whitespace()
                .map(|number| number.trim().parse::<u32>().expect("invalid input"))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let times = parsed.get(0).expect("invalid input");
    let records = parsed.get(1).expect("invalid input");

    let result = times
        .iter()
        .zip(records.iter())
        .map(|(&time, &record)| Race { time, record }.count_record_time_configurations())
        .product::<u32>();

    dbg!(result);
}
