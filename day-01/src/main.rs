const INPUT: &str = include_str!("input.txt");

const NUMBER_STRINGS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn get_last_digit(line: &str) -> u32 {
    for index in 0..line.len() {
        let substring = &line[..line.len() - index];
        if let Some(number) = substring
            .chars()
            .last()
            .expect("line ended without a digit")
            .to_digit(10)
        {
            return number;
        }

        if let Some((index, _)) = NUMBER_STRINGS
            .iter()
            .enumerate()
            .filter(|(_, &number_string)| substring.ends_with(number_string))
            .nth(0)
        {
            return (index + 1) as u32;
        }
    }

    unreachable!("No digit in line");
}

fn get_first_digit(line: &str) -> u32 {
    for index in 0..line.len() {
        let substring = &line[index..];
        if let Some(number) = substring
            .chars()
            .next()
            .expect("line ended without a digit")
            .to_digit(10)
        {
            return number;
        }

        if let Some((index, _)) = NUMBER_STRINGS
            .iter()
            .enumerate()
            .filter(|(_, &number_string)| substring.starts_with(number_string))
            .nth(0)
        {
            return (index + 1) as u32;
        }
    }

    unreachable!("No digit in line");
}

fn main() {
    let sum = INPUT
        .lines()
        .map(|line| get_first_digit(line) * 10 + get_last_digit(line))
        .sum::<u32>();

    dbg!(sum);
}
