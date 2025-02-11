use std::{fmt::Debug, iter};

use itertools::Itertools;

#[derive(PartialEq, Clone, Copy, Debug)]
enum NumericKeypadButton {
    Digit(u8),
    Activate,
}

#[derive(PartialEq, Clone, Copy, Debug)]
enum Command {
    Up,
    Down,
    Left,
    Right,
    A,
}

trait Button: PartialEq + Copy + Debug {
    fn go(&self, command: &Command) -> Option<Self>
    where
        Self: Sized;
    fn start() -> Self;
}

impl Command {
    fn options() -> impl Iterator<Item = Command> {
        iter::successors(Some(Command::A), |direction| match direction {
            Command::A => Some(Command::Up),
            Command::Up => Some(Command::Down),
            Command::Down => Some(Command::Left),
            Command::Left => Some(Command::Right),
            Command::Right => None,
        })
    }
}

impl Button for NumericKeypadButton {
    fn go(&self, command: &Command) -> Option<Self> {
        match (command, self) {
            (Command::Up, Self::Digit(0)) => Some(Self::Digit(2)),
            (Command::Up, Self::Digit(digit)) if *digit <= 6 => Some(Self::Digit(digit + 3)),
            (Command::Up, Self::Activate) => Some(Self::Digit(3)),
            (Command::Down, Self::Digit(2)) => Some(Self::Digit(0)),
            (Command::Down, Self::Digit(3)) => Some(Self::Activate),
            (Command::Down, Self::Digit(digit)) if *digit >= 4 => Some(Self::Digit(digit - 3)),
            (Command::Left, Self::Activate) => Some(Self::Digit(0)),
            (Command::Left, Self::Digit(0)) => None,
            (Command::Left, Self::Digit(digit)) if (digit - 1) % 3 >= 1 => {
                Some(Self::Digit(digit - 1))
            }
            (Command::Right, Self::Digit(0)) => Some(Self::Activate),
            (Command::Right, Self::Activate) => None,
            (Command::Right, Self::Digit(digit)) if *digit % 3 >= 1 => Some(Self::Digit(digit + 1)),
            (Command::A, button) => Some(*button),
            _ => None,
        }
    }
    fn start() -> Self {
        NumericKeypadButton::Activate
    }
}

impl Button for Command {
    fn go(&self, command: &Command) -> Option<Self>
    where
        Self: Sized,
    {
        match (self, command) {
            (command, Command::A) => Some(*command),
            (Command::Up, Command::Down) => Some(Command::Down),
            (Command::Up, Command::Right) => Some(Command::A),
            (Command::Down, command) if *command != Command::Down => Some(*command),
            (Command::Left, Command::Right) => Some(Command::Down),
            (Command::Right, Command::Up) => Some(Command::A),
            (Command::Right, Command::Left) => Some(Command::Down),
            (Command::A, Command::Down) => Some(Command::Right),
            (Command::A, Command::Left) => Some(Command::Up),
            _ => None,
        }
    }

    fn start() -> Self {
        Command::A
    }
}

fn from_to<T: Button>(from: &T, to: &T) -> Vec<Vec<Command>> {
    let mut possibilities: Vec<Vec<Command>> = vec![];
    let mut options = vec![(*from, vec![])];

    while let Some((current, path)) = options.pop() {
        if let Some(first) = possibilities.first() {
            if path.len() > first.len() {
                break;
            }
        }
        for (option, command) in Command::options()
            .filter_map(|command| current.go(&command).and_then(|next| Some((next, command))))
        {
            let mut new_path = path.clone();
            new_path.push(command);
            if option == *to {
                if command != Command::A {
                    new_path.push(Command::A);
                }
                possibilities.push(new_path);
            } else {
                options.push((option, new_path));
            }
        }
        options.sort_by_key(|(_, path)| path.len());
        options.reverse();
    }

    possibilities.into_iter().min_set_by_key(|x| x.len())
}

trait Keypad {
    fn cost<T: Button>(&self, path: &[T]) -> usize;
}

struct DirectInput;

impl Keypad for DirectInput {
    fn cost<T: Button>(&self, path: &[T]) -> usize {
        path.len()
    }
}

struct IndirectInput<T>
where
    T: Keypad,
{
    next: T,
}

impl<K: Keypad> Keypad for IndirectInput<K> {
    fn cost<T: Button>(&self, path: &[T]) -> usize {
        iter::once(&T::start())
            .chain(path.iter())
            .tuple_windows::<(_, _)>()
            .map(|x| {
                let from = x.0;
                let to = x.1;
                let paths = from_to(from, to);
                paths.iter().map(|path| self.next.cost(path)).min().unwrap()
            })
            .sum()
    }
}

fn numeric_part(code: &[NumericKeypadButton]) -> usize {
    let numeric_part = code
        .iter()
        .filter_map(|button| match button {
            NumericKeypadButton::Digit(digit) => Some(*digit),
            NumericKeypadButton::Activate => None,
        })
        .fold(0, |acc, digit| acc * 10 + digit as usize);
    numeric_part
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
    let input = include_str!("../input/input.txt");
    let codes = input.lines().map(parse_code).collect::<Vec<_>>();

    let numeric_keypad = IndirectInput {
        next: IndirectInput {
            next: IndirectInput {
                next: DirectInput {},
            },
        },
    };

    let x = codes
        .iter()
        .map(|code| {
            let cost = numeric_keypad.cost(code);
            let numeric_part = numeric_part(code);
            println!("cost: {cost} * numeric: {numeric_part}");
            cost * numeric_part
        })
        .sum::<usize>();
    println!("{}", x);
}
