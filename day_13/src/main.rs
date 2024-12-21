use regex::{self, Regex};

struct ClawMachine {
    button_a: (i64, i64),
    button_b: (i64, i64),
    prize: (i64, i64),
}

fn resolve(machine: &ClawMachine) -> Option<(i64, i64)> {
    let denom = machine.button_a.0 * machine.button_b.1 - machine.button_a.1 * machine.button_b.0;
    assert!(denom != 0);
    let nom = machine.prize.1 * machine.button_a.0 - machine.prize.0 * machine.button_a.1;
    let y = nom / denom;

    if y * denom != nom {
        return None;
    }
    let x = (machine.prize.0 - y * machine.button_b.0) / machine.button_a.0;
    if x * machine.button_a.0 != machine.prize.0 - y * machine.button_b.0 {
        return None;
    }
    Some((x, y))
}

fn main() {
    let input = include_str!("../input/input.txt");
    let regex = Regex::new(r"\d+").unwrap();

    let numbers = regex
        .find_iter(input)
        .map(|x| x.as_str().parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    let clawmachines = numbers
        .chunks_exact(6)
        .map(|value| ClawMachine {
            button_a: (value[0], value[1]),
            button_b: (value[2], value[3]),
            prize: (value[4], value[5]),
        })
        .collect::<Vec<_>>();

    let tokens = clawmachines
        .iter()
        .filter_map(|claw_machine| resolve(&claw_machine))
        .map(|(a, b)| a * 3 + b)
        .sum::<i64>();
    println!("To win all prizes, you have to spend {} tokens.", tokens);

    let modified_clawmachines = clawmachines
        .iter()
        .map(|claw_machine| ClawMachine {
            prize: (
                claw_machine.prize.0 + 10000000000000,
                claw_machine.prize.1 + 10000000000000,
            ),
            ..*claw_machine
        })
        .collect::<Vec<_>>();
    let tokens = modified_clawmachines
        .iter()
        .filter_map(|claw_machine| resolve(&claw_machine))
        .map(|(a, b)| a * 3 + b)
        .sum::<i64>();
    println!("To win all prizes, you have to spend {} tokens.", tokens);
}
