use std::collections::{HashMap, HashSet};

enum Wire {
    Gate {
    i1: String,
    i2: String,
    op: Box<dyn Fn(bool, bool) -> bool>,
    },
    Value(bool),
}

fn parse(input: &str) -> HashMap<String, Wire> {
    let (values_input, gates_input) = input.split_once("\n\n").unwrap();

    let values = values_input.lines().map(|line| {
        let (name, value) = line.split_once(": ").unwrap();
        let value = value == "1";
        (name.to_string(), Wire::Value(value))
    });

    let gates = gates_input.lines().map(|line| {
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
        let gate = Wire::Gate {
                i1,
                i2,
                op: Box::new(op),
        };
        (o, gate)
    });
    values.chain(gates).collect()
}

fn resolve(name: &str, wires: &HashMap<String, Wire>) -> bool {
    match wires.get(name).unwrap() {
        Wire::Gate { i1, i2, op } => op(resolve(i1, wires), resolve(i2, wires)),
        Wire::Value(value) => *value,
    }
}

fn z_resolve(wires: &HashMap<String, Wire>) -> u64 {
    (0..)
        .map(|index| format!("z{:02}", index))
        .take_while(|name| wires.contains_key(name))
        .map(|name| if resolve(&name, wires) { 1 } else { 0 })
        .enumerate()
        .fold(0, |total, (index, value)| total + (value << index))
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
    let wires = parse(input);
    let z_value = z_resolve(&wires);
    println!("The produces number is {}.", z_value);
}
