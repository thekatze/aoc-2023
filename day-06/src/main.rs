const INPUT: &str = include_str!("input.txt");
// const INPUT: &str = "Time:      7  15   30
// Distance:  9  40  200";

struct Race {
    time: u64,
    record: u64,
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
                .collect::<String>()
                .parse::<u64>()
                .expect("invalid input")
        })
        .collect::<Vec<_>>();

    let time = *parsed.get(0).expect("invalid input");
    let record = *parsed.get(1).expect("invalid input");

    let result = Race { time, record }.count_record_time_configurations();

    dbg!(result);
}
