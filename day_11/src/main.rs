use std::collections::HashMap;

type Cache = HashMap<(u64, i32), usize>;

fn determine_stone(number: u64, iteration: i32, cache: &mut Cache) -> usize {
    if let Some(count) = cache.get(&(number, iteration)) {
        return *count;
    }

    let result = if iteration == 0 {
        1
    } else {
        if number == 0 {
            determine_stone(1, iteration - 1, cache)
        } else {
            let stone_length = ((number as f64).log10() as u32) + 1;
            if stone_length % 2 == 0 {
                let divisor = 10u64.pow(stone_length / 2);
                let a = number / divisor;
                let b = number % divisor;
                determine_stone(a, iteration - 1, cache) + determine_stone(b, iteration - 1, cache)
            } else {
                determine_stone(number * 2024, iteration - 1, cache)
            }
        }
    };

    cache.insert((number, iteration), result);
    result
}

fn main() {
    let input = include_str!("../input/input.txt");

    let stones = input
        .split_ascii_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    let mut cache = Cache::new();

    let stone_count_25 = stones
        .iter()
        .map(|number| determine_stone(*number, 25, &mut cache))
        .sum::<usize>();
    println!("After 25 blinks there are {} stones.", stone_count_25);
    let stone_count_75 = stones
        .iter()
        .map(|number| determine_stone(*number, 75, &mut cache))
        .sum::<usize>();
    println!("After 75 blinks there are {} stones.", stone_count_75);
}
