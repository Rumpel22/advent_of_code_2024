#[derive(Debug)]
struct Equation {
    result: i64,
    numbers: Vec<i64>,
}

fn possible_equation(result: i64, numbers: &[i64]) -> bool {
    if numbers.len() == 1 {
        return result == numbers[0];
    }

    let last = numbers.last().unwrap();
    let rest = &numbers[..numbers.len() - 1];

    possible_equation(result - last, rest)
        || (result % last == 0 && possible_equation(result / last, rest))
}

fn possible(equation: &&Equation) -> bool {
    possible_equation(equation.result, &equation.numbers)
}
fn main() {
    let input = include_str!("../input/input.txt");
    let equations = input
        .lines()
        .map(|line| {
            let (result, rest) = line.split_once(':').unwrap();
            let result = result.parse().unwrap();
            let numbers = rest
                .split_ascii_whitespace()
                .map(|number| number.parse().unwrap())
                .collect();
            Equation { numbers, result }
        })
        .collect::<Vec<_>>();

    let total_calibration_result = equations
        .iter()
        .filter(possible)
        .inspect(|e| println!("{:?}", e))
        .map(|equation| equation.result)
        .sum::<i64>();
    println!(
        "The total calibration result ist {}.",
        total_calibration_result
    );
}
