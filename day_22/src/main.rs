use std::ops::BitXor;

fn mix(value: u64, number: u64) -> u64 {
    value.bitxor(number)
}

fn prune(number: u64) -> u64 {
    number % 2_u64.pow(24)
}

fn next(secret_number: u64) -> u64 {
    let number = prune(mix(secret_number, secret_number * 64));
    let number = prune(mix(number, number / 32));
    let number = prune(mix(number, number * 2048));
    number
}
fn main() {
    let input = include_str!("../input/input.txt");
    let numbers = input
        .lines()
        .map(|line| line.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    let x = numbers
        .iter()
        .map(|number| (0..2000).fold(*number, |number, _| next(number)))
        .sum::<u64>();
    println!("{}", x);
}
