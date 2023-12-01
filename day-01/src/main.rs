// const INPUT: &str = include_str!("input.txt");
const INPUT: &str = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

fn main() {
    let sum = INPUT
        .lines()
        .map(|line| line.chars().filter(|c| c.is_digit(10)).collect::<String>())
        .map(|mut num_str| {
            if num_str.len() > 2 {
                num_str.drain(1..num_str.len() - 1);
                num_str
            } else if num_str.len() == 1 {
                num_str.repeat(2)
            } else {
                num_str
            }
        })
        .inspect(|num_str| eprintln!("{}", num_str))
        .map(|num_str| num_str.parse::<u32>().unwrap())
        .sum::<u32>();

    dbg!(sum);
}
