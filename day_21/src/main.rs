use std::{cell::RefCell, iter, marker::PhantomData};

use itertools::Itertools;

struct Keypad<T>
where
    T: KeypadButton,
{
    _s: PhantomData<T>,
}
trait KeypadButton {
    fn from_to(from: &Self, to: &Self) -> Command;
    fn go(&self, command: &Command) -> Self;
    fn start() -> Self;
}

#[derive(PartialEq, Clone, Copy)]
enum NumericKeypadButton {
    Digit(u8),
    Activate,
}

impl NumericKeypadButton {
    fn col(&self) -> u8 {
        match self {
            NumericKeypadButton::Digit(0) => 1,
            NumericKeypadButton::Digit(digit) => (digit + 2) % 3,
            NumericKeypadButton::Activate => 2,
        }
    }
    fn row(&self) -> u8 {
        match self {
            NumericKeypadButton::Digit(digit) => (digit + 2) / 3,
            NumericKeypadButton::Activate => 0,
        }
    }
}

impl KeypadButton for NumericKeypadButton {
    fn from_to(from: &Self, to: &Self) -> Command {
        let from_col = from.col();
        let from_row = from.row();
        let to_col = to.col();
        let to_row = to.row();
        if to_row > from_row {
            return Command::Up;
        }
        if to_col > from_col {
            return Command::Right;
        }
        if to_col < from_col {
            return Command::Left;
        }
        if to_row < from_row {
            return Command::Down;
        }
        return Command::A;
    }

    fn go(&self, command: &Command) -> Self {
        match (command, self) {
            (Command::Up, Self::Digit(0)) => Self::Digit(2),
            (Command::Up, Self::Digit(digit)) => Self::Digit(digit + 3),
            (Command::Up, Self::Activate) => Self::Digit(3),
            (Command::Down, Self::Digit(2)) => Self::Digit(0),
            (Command::Down, Self::Digit(3)) => Self::Activate,
            (Command::Down, Self::Digit(digit)) => Self::Digit(digit - 3),
            (Command::Left, Self::Activate) => Self::Digit(0),
            (Command::Left, Self::Digit(digit)) => Self::Digit(digit - 1),
            (Command::Right, Self::Digit(0)) => Self::Activate,
            (Command::Right, Self::Digit(digit)) => Self::Digit(digit + 1),
            (Command::A, button) => button.clone(),
            _ => unreachable!(),
        }
    }

    fn start() -> Self {
        Self::Activate
    }
}

impl KeypadButton for Command {
    fn from_to(from: &Self, to: &Self) -> Command {
        match (from, to) {
            (command1, command2) if command1 == command2 => Command::A,
            (Command::Up, Command::A) => Command::Right,
            (Command::Up, _) => Command::Down,
            (Command::A, Command::Up) => Command::Left,
            (Command::A, _) => Command::Down,
            (Command::Left, _) => Command::Right,
            (Command::Right, Command::Left) | (Command::Right, Command::Down) => Command::Left,
            (Command::Right, _) => Command::Up,
            (Command::Down, Command::Up) => Command::Up,
            (Command::Down, Command::Left) => Command::Left,
            (Command::Down, _) => Command::Right,
        }
    }

    fn go(&self, command: &Command) -> Self {
        match (self, command) {
            (command, Command::A) => *command,
            (Command::Up, Command::Down) => Command::Down,
            (Command::Up, Command::Right) => Command::A,
            (Command::Down, command) => *command,
            (Command::Left, Command::Right) => Command::Down,
            (Command::Right, Command::Up) => Command::A,
            (Command::Right, Command::Left) => Command::Down,
            (Command::A, Command::Down) => Command::Right,
            (Command::A, Command::Left) => Command::Up,
            _ => unreachable!(),
        }
    }

    fn start() -> Self {
        Command::A
    }
}

impl<T: KeypadButton> Keypad<T> {
    fn enter_code(code: &[T]) -> Vec<Command> {
        let key = RefCell::new(T::start());
        code.iter()
            .flat_map(|target| {
                iter::repeat(target)
                    .map(|target| {
                        let command = T::from_to(&key.borrow(), target);
                        let next_key = key.borrow().go(&command);
                        *key.borrow_mut() = next_key;
                        command
                    })
                    .take_while_inclusive(|command| command != &Command::A)
            })
            .collect::<Vec<_>>()
    }
}

#[derive(PartialEq, Clone, Copy)]
enum Command {
    Up,
    Down,
    Left,
    Right,
    A,
}

fn to_string(commands: &[Command]) -> String {
    commands
        .iter()
        .map(|command| match command {
            Command::Up => '^',
            Command::Down => 'v',
            Command::Left => '<',
            Command::Right => '>',
            Command::A => 'A',
        })
        .collect()
}

fn complexity(commands: &[Command], code: &[NumericKeypadButton]) -> usize {
    let numeric_part = code
        .iter()
        .filter_map(|button| match button {
            NumericKeypadButton::Digit(digit) => Some(*digit),
            NumericKeypadButton::Activate => None,
        })
        .fold(0, |acc, digit| acc * 10 + digit as usize);
    let length = commands.len();
    numeric_part * length
}

fn parse_code(code: &str) -> Vec<NumericKeypadButton> {
    code.char_indices()
        .map(|(i, _)| match &code[i..=i] {
            "A" => NumericKeypadButton::Activate,
            c => NumericKeypadButton::Digit(c.parse().unwrap()),
        })
        .collect()
}
fn main() {
    let input = include_str!("../input/demo.txt");
    let codes = input.lines().map(parse_code).collect::<Vec<_>>();

    let commands = codes
        .iter()
        .map(|code| Keypad::enter_code(&Keypad::enter_code(&Keypad::enter_code(code))))
        .collect::<Vec<_>>();
    let complexities = commands
        .iter()
        .zip(codes)
        .map(|(commands, code)| complexity(commands, &code))
        .collect::<Vec<_>>();
    let sum = complexities.iter().sum::<usize>();
    println!("The sum of the complexities is {}.", sum);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let testcases = [
            (
                "029A",
                "<vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A",
            ),
            (
                "980A",
                "<v<A>>^AAAvA^A<vA<AA>>^AvAA<^A>A<v<A>A>^AAAvA<^A>A<vA>^A<A>A",
            ),
            (
                "179A",
                "<v<A>>^A<vA<A>>^AAvAA<^A>A<v<A>>^AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A",
            ),
            (
                "456A",
                "<v<A>>^AA<vA<A>>^AAvAA<^A>A<vA>^A<A>A<vA>^A<A>A<v<A>A>^AAvA<^A>A",
            ),
            (
                "379A",
                "<v<A>>^AvA^A<vA<AA>>^AAvA<^A>AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A",
            ),
        ];
        for (code, result) in testcases {
            let code = parse_code(code);
            let commands = Keypad::enter_code(&Keypad::enter_code(&Keypad::enter_code(&code)));
            println!("{}", to_string(&commands));
            assert_eq!(commands.len(), result.len());
        }
    }
}
