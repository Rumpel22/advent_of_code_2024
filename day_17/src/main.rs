#![allow(non_snake_case, non_camel_case_types)]

use std::ops::BitXorAssign;

struct Registers {
    A: u32,
    B: u32,
    C: u32,
}

enum Register {
    A,
    B,
    C,
}

enum Combo {
    Literal(u32),
    Register(Register),
}

enum OpCode {
    adv(Combo),
    bxl(u32),
    bst(Combo),
    jnz(u32),
    bxc,
    out(Combo),
    bdv(Combo),
    cdv(Combo),
}

fn parse_register(line: &str) -> u32 {
    line[12..].parse::<u32>().unwrap()
}

fn parse_program(line: &str) -> Vec<OpCode> {
    let numbers = line[9..]
        .split(',')
        .map(|c| c.parse::<u32>().unwrap())
        .collect::<Vec<_>>();

    let combo = |number: u32| match number {
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

fn combo_value(combo: &Combo, regs: &Registers) -> u32 {
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

fn execute_program(program: &[OpCode], mut regs: Registers) -> String {
    let mut ip = 0;
    let mut outputs = vec![];
    loop {
        let op_code = match program.get(ip / 2) {
            Some(op_code) => op_code,
            None => break,
        };

        match op_code {
            OpCode::adv(combo) => {
                let denominator = combo_value(combo, &regs);
                let v = regs.A / 2u32.pow(denominator);
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
                let denominator = combo_value(combo, &regs);
                let v = regs.A / 2u32.pow(denominator);
                regs.B = v;
            }
            OpCode::cdv(combo) => {
                let denominator = combo_value(combo, &regs);
                let v = regs.A / 2u32.pow(denominator);
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

fn main() {
    let input = include_str!("../input/demo.txt");
    let mut lines = input.lines();
    let A = lines.next().map(parse_register).unwrap();
    let B = lines.next().map(parse_register).unwrap();
    let C = lines.next().map(parse_register).unwrap();
    let regs = Registers { A, B, C };
    let program = lines.skip(1).next().map(parse_program).unwrap();

    let output = execute_program(&program, regs);
    println!("The program prints:\n{}", output);
}
