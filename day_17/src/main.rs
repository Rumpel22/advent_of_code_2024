#![allow(non_snake_case, non_camel_case_types)]

use std::ops::BitXorAssign;

#[derive(Clone, Copy)]
struct Registers {
    A: u64,
    B: u64,
    C: u64,
}

#[derive(Debug)]
enum Register {
    A,
    B,
    C,
}

#[derive(Debug)]
enum Combo {
    Literal(u64),
    Register(Register),
}

#[derive(Debug)]
enum OpCode {
    adv(Combo),
    bxl(u64),
    bst(Combo),
    jnz(u64),
    bxc,
    out(Combo),
    bdv(Combo),
    cdv(Combo),
}

fn parse_register(line: &str) -> u64 {
    line[12..].parse::<u64>().unwrap()
}

fn parse_program(line: &str) -> Vec<OpCode> {
    let numbers = line[9..]
        .split(',')
        .map(|c| c.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    let combo = |number: u64| match number {
        0..=3 => Combo::Literal(number),
        4 => Combo::Register(Register::A),
        5 => Combo::Register(Register::B),
        6 => Combo::Register(Register::C),
        _ => unreachable!(),
    };

    numbers
        .chunks_exact(2)
        .map(|a| match a[0] {
            0 => OpCode::adv(combo(a[1])),
            1 => OpCode::bxl(a[1]),
            2 => OpCode::bst(combo(a[1])),
            3 => OpCode::jnz(a[1]),
            4 => OpCode::bxc,
            5 => OpCode::out(combo(a[1])),
            6 => OpCode::bdv(combo(a[1])),
            7 => OpCode::cdv(combo(a[1])),
            _ => unreachable!(),
        })
        .collect()
}

fn combo_value(combo: &Combo, regs: &Registers) -> u64 {
    let value = match combo {
        Combo::Literal(x) => x,
        Combo::Register(register) => &match register {
            Register::A => regs.A,
            Register::B => regs.B,
            Register::C => regs.C,
        },
    };
    *value
}

fn execute_program(program: &[OpCode], regs: &mut Registers) -> String {
    let mut ip = 0;
    let mut outputs = vec![];
    loop {
        let op_code = match program.get(ip / 2) {
            Some(op_code) => op_code,
            None => break,
        };

        match op_code {
            OpCode::adv(combo) => {
                let denominator = combo_value(combo, &regs) as u32;
                let v = regs.A / 2u64.pow(denominator);
                regs.A = v;
            }
            OpCode::bxl(literal) => {
                regs.B.bitxor_assign(literal);
            }
            OpCode::bst(combo) => {
                let v = combo_value(combo, &regs) % 8;
                regs.B = v;
            }
            OpCode::jnz(literal) => {
                if regs.A != 0 {
                    ip = *literal as usize;
                }
            }
            OpCode::bxc => {
                regs.B.bitxor_assign(regs.C);
            }
            OpCode::out(combo) => {
                let value = combo_value(combo, &regs) % 8;
                outputs.push(value.to_string());
            }
            OpCode::bdv(combo) => {
                let denominator = combo_value(combo, &regs) as u32;
                let v = regs.A / 2u64.pow(denominator);
                regs.B = v;
            }
            OpCode::cdv(combo) => {
                let denominator = combo_value(combo, &regs) as u32;
                let v = regs.A / 2u64.pow(denominator);
                regs.C = v;
            }
        }
        match op_code {
            OpCode::jnz(_) if regs.A != 0 => {}
            _ => {
                ip += 2;
            }
        }
    }
    outputs.join(",")
}

fn find_a(program: &[OpCode], orig_regs: &Registers, program_code: &str) -> u64 {
    let mut a = 0;
    for _ in 0..(2 * program.len()) {
        a *= 8;

        let mut j = 0;
        let i = loop {
            let i = j;
            j += 1;
            let mut regs = Registers {
                A: a + i,
                ..*orig_regs
            };
            let output = execute_program(&program, &mut regs);
            if program_code.ends_with(&output) {
                break i;
            }
        };
        a += i;
    }
    a
}

fn parse_input(input: &str) -> (Registers, Vec<OpCode>) {
    let mut lines = input.lines();
    let A = lines.next().map(parse_register).unwrap();
    let B = lines.next().map(parse_register).unwrap();
    let C = lines.next().map(parse_register).unwrap();
    let regs = Registers { A, B, C };
    let program = lines.skip(1).next().map(parse_program).unwrap();
    (regs, program)
}

fn main() {
    let input = include_str!("../input/input.txt");

    let (orig_regs, program) = parse_input(input);

    let mut regs = orig_regs;
    let output = execute_program(&program, &mut regs);
    println!("The program prints:\n{}", output);

    let program_code = &input.lines().skip(4).next().unwrap()[9..];
    println!("\nThe program itself: {}", program_code);

    let a = find_a(&program, &regs, program_code);
    println!("The program outputs itself if register A is set to {}.", a);
    let mut regs = Registers { A: a, ..orig_regs };
    println!("{}", execute_program(&program, &mut regs))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let program = parse_program("Program: 2,6");
        let mut regs = Registers { A: 0, B: 0, C: 1 };
        execute_program(&program, &mut regs);
        assert_eq!(1, regs.B)
    }

    #[test]
    fn test_2() {
        let program = parse_program("Program: 5,0,5,1,5,4");
        let mut regs = Registers { A: 10, B: 0, C: 0 };
        let output = execute_program(&program, &mut regs);
        assert_eq!("0,1,2", output);
    }

    #[test]
    fn test_3() {
        let program = parse_program("Program: 0,1,5,4,3,0");
        let mut regs = Registers {
            A: 2024,
            B: 0,
            C: 0,
        };
        let output = execute_program(&program, &mut regs);
        assert_eq!("4,2,5,6,7,7,7,7,3,1,0", output);
        assert_eq!(0, regs.A);
    }

    #[test]
    fn test_4() {
        let program = parse_program("Program: 1,7");
        let mut regs = Registers { A: 0, B: 29, C: 0 };
        execute_program(&program, &mut regs);
        assert_eq!(26, regs.B);
    }

    #[test]
    fn test_5() {
        let program = parse_program("Program: 4,0");
        let mut regs = Registers {
            A: 0,
            B: 2024,
            C: 43690,
        };
        execute_program(&program, &mut regs);
        assert_eq!(44354, regs.B);
    }
}
