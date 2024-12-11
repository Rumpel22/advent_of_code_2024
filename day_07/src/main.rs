#[derive(Debug)]
struct Equation {
    result: i64,
    numbers: Vec<i64>,
}

enum Operator {
    Plus,
    Mult,
}

fn possible_equation(result: i64, numbers: &[i64]) -> Option<Vec<Operator>> {
    if numbers.len() == 1 {
        return match result == numbers[0] {
            true => Some(vec![]),
            false => None,
        };
    }

    let last = numbers.last().unwrap();
    let rest = &numbers[..numbers.len() - 1];

    if let Some(mut symbols) = possible_equation(result - last, rest) {
        symbols.push(Operator::Plus);
        Some(symbols)
    } else if let Some(mut symbols) = possible_equation(result / last, rest) {
        if result % last == 0 {
            symbols.push(Operator::Mult);
            Some(symbols)
        } else {
            None
        }
    } else {
        None
    }
}

fn possible(equation: &&Equation) -> bool {
    if let Some(operations) = possible_equation(equation.result, &equation.numbers) {
        let x = equation.numbers.iter().skip(1).zip(operations.iter()).fold(
            equation.numbers[0],
            |result, (number, operator)| match operator {
                Operator::Mult => result * number,
                Operator::Plus => result + number,
            },
        );
        assert!(x == equation.result);
        return true;
    }
    false
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
