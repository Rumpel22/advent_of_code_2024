use std::collections::HashMap;

type Towel<'a> = &'a str;
type Pattern<'a> = &'a str;
type Towels<'a> = Vec<Towel<'a>>;
type Patterns<'a> = Vec<Pattern<'a>>;

fn parse_input(input: &str) -> (Towels, Patterns) {
    let mut lines = input.lines();
    let towels = lines
        .next()
        .unwrap()
        .split(',')
        .map(|pattern| pattern.trim())
        .collect::<Vec<_>>();
    let patterns = lines.skip(1).collect::<Vec<_>>();
    (towels, patterns)
}

fn is_pattern_possible(pattern: &Pattern, towels: &Towels) -> bool {
    if pattern.is_empty() {
        return true;
    }

    towels
        .iter()
        .filter(|towel| pattern.starts_with(*towel))
        .any(|towel| is_pattern_possible(&&pattern[towel.len()..], towels))
}

fn valid_patterns<'a>(
    pattern: &Pattern<'a>,
    towels: &'a Towels<'a>,
    cache: &mut HashMap<&'a str, usize>,
) -> usize {
    if pattern.is_empty() {
        return 1;
    }

    if let Some(cached_value) = cache.get(pattern) {
        return *cached_value;
    }

    let mut value = 0;
    for towel in towels {
        if pattern.starts_with(towel) {
            value += valid_patterns(&&pattern[towel.len()..], towels, cache);
        }
    }
    cache.insert(&pattern, value);
    value
}

fn main() {
    let input = include_str!("../input/input.txt");
    let (towels, patterns) = parse_input(input);

    let possible_patterns = patterns
        .iter()
        .filter(|pattern| is_pattern_possible(pattern, &towels))
        .collect::<Vec<_>>();
    println!("There are {} valid patterns.", possible_patterns.len());

    let mut cache = HashMap::new();

    let different_patterns = patterns
        .iter()
        .map(|pattern| valid_patterns(pattern, &towels, &mut cache))
        .sum::<usize>();
    println!(
        "There are {} different patterns possible.",
        different_patterns
    );
}
