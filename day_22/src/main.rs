use std::{collections::HashSet, iter, ops::BitXor};

fn mix(value: u64, number: u64) -> u64 {
    value.bitxor(number)
}

fn prune(number: u64) -> u64 {
    number % 2_u64.pow(24)
}

fn last_digit(number: u64) -> i8 {
    (number % 10) as i8
}

fn next(secret_number: u64) -> u64 {
    let number = prune(mix(secret_number, secret_number * 64));
    let number = prune(mix(number, number / 32));
    let number = prune(mix(number, number * 2048));
    number
}

fn main() {
    let input = include_str!("../input/input.txt");
    let init_numbers = input
        .lines()
        .map(|line| line.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    let secret_numbers = init_numbers
        .iter()
        .map(|number| {
            (0..2000)
                .scan(*number, |number, _| {
                    *number = next(*number);
                    Some(*number)
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let sum_secret_numbers = secret_numbers
        .iter()
        .map(|numbers| numbers.last().unwrap())
        .sum::<u64>();
    println!("{}", sum_secret_numbers);

    let last_digits = secret_numbers
        .iter()
        .map(|numbers| {
            numbers
                .iter()
                .map(|number| last_digit(*number))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let differences = init_numbers
        .iter()
        .zip(last_digits.iter())
        .map(|(init, digits)| {
            iter::once(&last_digit(*init))
                .chain(digits.iter())
                .scan(0, |last, digit| {
                    let digit = *digit as i8;
                    let difference = digit - *last;
                    *last = digit;
                    Some(difference)
                })
                .skip(1)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let windows = differences
        .iter()
        .flat_map(|diffs| diffs.windows(4))
        .collect::<HashSet<_>>();

    let max = windows
        .iter()
        .map(|window| {
            differences
                .iter()
                .zip(last_digits.iter())
                .filter_map(|(diff, digits)| {
                    let pos = diff.windows(4).position(|d| d == *window);
                    match pos {
                        Some(pos) => Some(digits[pos + 3] as u64),
                        None => None,
                    }
                })
                .sum::<u64>()
        })
        .max()
        .unwrap();
    println!("{}", max)
}
