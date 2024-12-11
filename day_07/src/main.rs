#[derive(Debug)]
struct Equation {
    result: i64,
    numbers: Vec<i64>,
}

fn concat(a: i64, b: i64) -> i64 {
    let x = (b as f64).log10();
    let x = x as u32;
    a * 10i64.pow(x + 1) + b
}

fn possible_equation(result: i64, numbers: &[i64]) -> bool {
    if numbers.len() == 1 {
        return result == numbers[0];
    }

    let mut new_numbers = vec![numbers[0] + numbers[1]];
    new_numbers.extend_from_slice(&numbers[2..]);

    if possible_equation(result, &new_numbers) {
        return true;
    }

    new_numbers[0] = numbers[0] * numbers[1];
    if possible_equation(result, &new_numbers) {
        return true;
    }

    new_numbers[0] = concat(numbers[0], numbers[1]);
    if possible_equation(result, &new_numbers) {
        return true;
    }
    false
}

fn possible(equation: &&Equation) -> bool {
    possible_equation(equation.result, &equation.numbers)
}

fn main() {
    let input = include_str!("../input/demo.txt");
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
        .map(|equation| equation.result)
        .sum::<i64>();
    println!(
        "The total calibration result ist {}.",
        total_calibration_result
    );
}
