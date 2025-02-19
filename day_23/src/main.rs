use std::collections::{HashMap, HashSet};

type Computer = String;
#[derive(Debug, Eq, PartialEq, Hash, Clone)]
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
    fn password(&self) -> String {
        self.computers.join(",")
    }
    fn len(&self) -> usize {
        self.computers.len()
    }
    fn add(&mut self, computer: &Computer) {
        self.computers.push(computer.to_string());
        self.computers.sort();
    }

    fn intersect(&self, other: &Self) -> Self {
        let mut computers = self.computers.clone();
        computers.retain(|c| other.computers.contains(c));
        Self::new(&computers)
    }
}
type Connections = Vec<Group>;
type Computers = HashMap<Computer, Group>;

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
            .and_modify(|group: &mut Group| group.add(b))
            .or_insert(Group::new(&[b.clone()]));
        computers
            .entry(b.clone())
            .and_modify(|group: &mut Group| group.add(a))
            .or_insert(Group::new(&[a.clone()]));
    });
    computers
}

fn main() {
    let input = include_str!("../input/input.txt");
    let connections = parse(input);
    let computers = computers(&connections);

    let mut sets = HashSet::new();
    for (computer, others) in computers.iter().filter(|(c, _)| c.starts_with('t')) {
        for other in &others.computers {
            let x = computers.get(other).unwrap();
            for y in &x.computers {
                if others.computers.contains(y) {
                    let group = Group::new(&[computer.clone(), other.clone(), y.clone()]);
                    sets.insert(group);
                }
            }
        }
    }
    println!(
        "There are {} sets with three connected computers, where at least one starts with a 't'.",
        sets.len()
    );

    let mut longest_set = Group::new(&[]);
    for (computer, group) in &computers {
        let mut set = group.clone();
        set.add(computer);

        for other in &group.computers {
            if !set.computers.contains(other) {
                continue;
            }
            let others = computers.get(other).unwrap();

            set = set.intersect(others);
            set.add(other);
        }

        if set.len() > longest_set.len() {
            longest_set = set;
        }
    }
    println!("The largest group is {}.", longest_set.password())
}
