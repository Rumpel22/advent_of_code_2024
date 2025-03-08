use std::collections::HashMap;

trait Operation {
    fn exec(&self, b1: bool, b2: bool) -> bool;
}

enum Op {
    And,
    Or,
    Xor,
}

impl Operation for Op {
    fn exec(&self, b1: bool, b2: bool) -> bool {
        match self {
            Op::And => b1 & b2,
            Op::Or => b1 | b2,
            Op::Xor => b1 ^ b2,
        }
    }
}

enum Wire {
    Gate { i1: String, i2: String, op: Op },
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
            "AND" => Op::And,
            "OR" => Op::Or,
            "XOR" => Op::Xor,
            _ => unreachable!(),
        };
        let i2 = parts.next().unwrap().to_string();
        parts.next();
        let o = parts.next().unwrap().to_string();
        let gate = Wire::Gate { i1, i2, op };
        (o, gate)
    });
    values.chain(gates).collect()
}

fn resolve(name: &str, wires: &HashMap<String, Wire>) -> bool {
    match wires.get(name).unwrap() {
        Wire::Gate { i1, i2, op } => op.exec(resolve(i1, wires), resolve(i2, wires)),
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

fn print_dot_script(wires: &HashMap<String, Wire>) {
    println!("digraph {}", '{');
    for (name, wire) in wires {
        let op = match wire {
            Wire::Gate { i1, i2, op } => {
                println!("{} -> {};\n{} -> {};", i1, name, i2, name);
                Some(op)
            }
            Wire::Value(_) => None,
        };
        let op_string = match op {
            Some(Op::And) => "shape=rect",
            Some(Op::Or) => "shape=diamond",
            Some(Op::Xor) => "shape=star",
            None => "",
        };
        match name.chars().nth(0).unwrap() {
            'x' => println!(
                "{} [color=lightblue,style=filled,group=x,{}];",
                name, op_string
            ),
            'y' => println!(
                "{} [color=lightgreen,style=filled,group=y,{}];",
                name, op_string
            ),
            'z' => println!("{} [color=red,style=filled,group=z,{}];", name, op_string),
            _ => println!("{} [{}];", name, op_string),
        }
    }
    println!("{}", '}');
}

fn main() {
    let input = include_str!("../input/input.txt");
    let wires = parse(input);
    let z_value = z_resolve(&wires);
    println!("The produces number is {}.", z_value);

    // Solution was solved visually:
    // The created dot-script is executed and the image is analysed.
    // - All z-fields (except the last) must be an XOR
    // - Every irregularity indicates a mismatch. Other gate is in proximity.
    // Sorting the affected gates is easy done manually.
    // The solution for the input is "gqp,hsw,jmh,mwk,qgd,z10,z18,z33"
    print_dot_script(&wires);
}
