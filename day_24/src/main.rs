use std::{collections::HashMap, ops::BitXor};

type State = bool;
type Wires = HashMap<String, Option<State>>;

struct Gate {
    i1: String,
    i2: String,
    o: String,
    op: Box<dyn Fn(bool, bool) -> bool>,
}
type Gates = Vec<Gate>;

fn wires(input: &str) -> (Wires, Gates) {
    let (i1, i2) = input.split_once("\n\n").unwrap();

    let mut wires = i1
        .lines()
        .map(|line| {
            let (name, state) = line.split_once(": ").unwrap();
            (
                name.to_string(),
                Some(if state == "1" { true } else { false }),
            )
        })
        .collect::<HashMap<_, _>>();

    let gates = i2
        .lines()
        .map(|line| {
            let mut parts = line.split(' ');
            let i1 = parts.next().unwrap().to_string();
            let op = match parts.next().unwrap() {
                "AND" => |a: bool, b: bool| a & b,
                "OR" => |a: bool, b: bool| a | b,
                "XOR" => |a: bool, b: bool| a ^ b,
                _ => unreachable!(),
            };
            let i2 = parts.next().unwrap().to_string();
            parts.next();
            let o = parts.next().unwrap().to_string();
            Gate {
                i1,
                i2,
                o,
                op: Box::new(op),
            }
        })
        .collect::<Vec<_>>();
    gates
        .iter()
        .flat_map(|gate| [&gate.i1, &gate.i2, &gate.o])
        .for_each(|wire| {
            if !wires.contains_key(wire) {
                wires.insert(wire.to_string(), None);
            }
        });
    (wires, gates)
}

fn get_number(wires: &Wires) -> u64 {
    let mut z_values = wires
        .iter()
        .filter(|(name, _)| name.starts_with('z'))
        .collect::<Vec<_>>();
    z_values.sort_by_key(|(name, _)| *name);
    z_values.reverse();
    let number = z_values.iter().fold(0_u64, |v, (_, state)| {
        (v << 1)
            + match state.unwrap() {
                true => 1,
                false => 0,
            }
    });
    number
}

fn main() {
    let input = include_str!("../input/input.txt");
    let (mut wires, gates) = wires(input);
    while wires.values().any(|state| state.is_none()) {
        let next_gate = gates
            .iter()
            .filter(|gate| {
                wires.get(&gate.i1).unwrap().is_some()
                    && wires.get(&gate.i2).unwrap().is_some()
                    && wires.get(&gate.o).unwrap().is_none()
            })
            .next()
            .unwrap();
        let i1 = wires.get(&next_gate.i1).unwrap().unwrap();
        let i2 = wires.get(&next_gate.i2).unwrap().unwrap();
        let new_state = (next_gate.op)(i1, i2);
        wires.insert(next_gate.o.clone(), Some(new_state));
    }
    let number = get_number(&wires);
    println!("The produces number is {}.", number);
}
