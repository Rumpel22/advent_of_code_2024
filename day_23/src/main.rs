use std::collections::{HashMap, HashSet};

type Computer = String;
#[derive(Debug, Eq, PartialEq, Hash)]
struct Group {
    computers: Vec<Computer>,
}

impl Group {
    fn new(computers: &[Computer]) -> Self {
        let mut group = Self {
            computers: computers.to_vec(),
        };
        group.computers.sort();
        group
    }
}
type Connections = Vec<Group>;
type Computers = HashMap<Computer, Vec<Computer>>;

fn parse(input: &str) -> Connections {
    input
        .lines()
        .map(|line| {
            let a = line[..2].to_string();
            let b = line[3..].to_string();
            Group::new(&[a, b])
        })
        .collect()
}
fn computers(connections: &[Group]) -> Computers {
    let mut computers = HashMap::new();
    connections.iter().for_each(|connection| {
        let a = &connection.computers[0];
        let b = &connection.computers[1];
        computers
            .entry(a.clone())
            .and_modify(|list: &mut Vec<String>| list.push(b.clone()))
            .or_insert(vec![b.clone()]);
        computers
            .entry(b.clone())
            .and_modify(|list: &mut Vec<String>| list.push(a.clone()))
            .or_insert(vec![a.clone()]);
    });
    computers
}

fn main() {
    let input = include_str!("../input/input.txt");
    let connections = parse(input);
    let computers = computers(&connections);

    let mut sets = HashSet::new();
    for (computer, others) in computers.iter().filter(|(c, _)| c.starts_with('t')) {
        for other in others {
            let x = computers.get(other).unwrap();
            for y in x {
                if others.contains(y) {
                    let group = Group::new(&[computer.clone(), other.clone(), y.clone()]);
                    sets.insert(group);
                }
            }
        }
    }
    println!("{}", sets.len());
}
