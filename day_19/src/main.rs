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

fn is_valid_pattern(pattern: &Pattern, towels: &Towels) -> bool {
    if pattern.is_empty() {
        return true;
    }

    towels
        .iter()
        .filter(|towel| pattern.starts_with(*towel))
        .any(|towel| is_valid_pattern(&&pattern[towel.len()..], towels))
}

fn main() {
    let input = include_str!("../input/input.txt");
    let (towels, patterns) = parse_input(input);

    let valid_patterns = patterns
        .iter()
        .filter(|pattern| is_valid_pattern(pattern, &towels))
        .collect::<Vec<_>>();
    println!("There are {} valid patterns.", valid_patterns.len());
}
