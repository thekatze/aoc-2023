const INPUT: &str = include_str!("input.txt");

fn main() {
    let sum = INPUT
        .lines()
        .map(|line| {
            line.replace("zero", "zero0zero")
                .replace("one", "one1one")
                .replace("two", "two2two")
                .replace("three", "three3three")
                .replace("four", "four4four")
                .replace("five", "five5five")
                .replace("six", "six6six")
                .replace("seven", "seven7seven")
                .replace("eight", "eight8eight")
                .replace("nine", "nine9nine")
        })
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
