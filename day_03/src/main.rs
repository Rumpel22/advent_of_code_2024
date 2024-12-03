use regex::{self, Regex};

enum RegexMatch {
    Enable,
    Disable,
    Numbers((i32, i32)),
}

fn main() {
    let input = include_str!("../input/input.txt");
    let regex_1 = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    let result = regex_1
        .captures_iter(input)
        .map(|captures| {
            (
                captures.get(1).unwrap().as_str().parse::<i32>().unwrap(),
                captures.get(2).unwrap().as_str().parse::<i32>().unwrap(),
            )
        })
        .map(|(a, b)| a * b)
        .sum::<i32>();
    println!("The result is {}.", result);

    let regex_2 = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don't\(\)").unwrap();
    let result = regex_2
        .captures_iter(input)
        .map(|captures| match captures.get(0).unwrap().as_str() {
            "do()" => RegexMatch::Enable,
            "don't()" => RegexMatch::Disable,
            _ => RegexMatch::Numbers((
                captures.get(1).unwrap().as_str().parse::<i32>().unwrap(),
                captures.get(2).unwrap().as_str().parse::<i32>().unwrap(),
            )),
        })
        .fold((true, 0), |(mut enabled, mut sum), regex_match| {
            match regex_match {
                RegexMatch::Enable => enabled = true,
                RegexMatch::Disable => enabled = false,
                RegexMatch::Numbers((a, b)) => {
                    if enabled {
                        sum += a * b;
                    }
                }
            };
            (enabled, sum)
        })
        .1;
    println!("The second result is {}.", result);
}
