use std::{collections::HashMap, fmt::Display, iter};

enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
struct XY {
    x: u8,
    y: u8,
}

impl Display for XY {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}
struct Map {
    corrupted: Vec<XY>,
    height: u8,
    width: u8,
}

impl Map {
    fn new(bytes: &[XY]) -> Self {
        let height = if bytes.len() < 100 { 6 } else { 70 } + 1;
        let width = height;
        let corrupted = Vec::from(bytes);
        Map {
            corrupted,
            height,
            width,
        }
    }

    fn neighbors<'a>(&'a self, position: &'a XY) -> impl Iterator<Item = XY> + use<'a> {
        let x = iter::successors(Some(Direction::Left), |direction| match direction {
            Direction::Left => Some(Direction::Right),
            Direction::Right => Some(Direction::Up),
            Direction::Up => Some(Direction::Down),
            Direction::Down => None,
        })
        .filter_map(|direction| match direction {
            Direction::Left if position.x > 0 => Some(XY {
                x: position.x - 1,
                ..*position
            }),
            Direction::Right if position.x < self.width - 1 => Some(XY {
                x: position.x + 1,
                ..*position
            }),
            Direction::Up if position.y > 0 => Some(XY {
                y: position.y - 1,
                ..*position
            }),
            Direction::Down if position.y < self.height - 1 => Some(XY {
                y: position.y + 1,
                ..*position
            }),
            _ => None,
        })
        .filter(|neighbor| !self.corrupted.contains(neighbor));
        x
    }
}

fn parse_input(input: &str) -> Vec<XY> {
    let bytes = input
        .lines()
        .filter_map(|line| line.split_once(','))
        .map(|(c1, c2)| {
            let x = c1.parse::<u8>().unwrap();
            let y = c2.parse::<u8>().unwrap();
            XY { x, y }
        })
        .collect::<Vec<_>>();
    bytes
}

fn h(position: &XY, end: &XY) -> u16 {
    (position.x.abs_diff(end.x) + position.y.abs_diff(end.y)) as u16
}

fn reconstruct_path(came_from: &HashMap<XY, XY>, current: &XY) -> Vec<XY> {
    iter::successors(Some(current), |x| came_from.get(x))
        .cloned()
        .collect()
}

fn find_path(map: &Map) -> Option<Vec<XY>> {
    let start = XY { x: 0, y: 0 };
    let end = XY {
        x: map.width - 1,
        y: map.height - 1,
    };

    let mut open_set = vec![start];
    let mut came_from = HashMap::new();
    let mut g_score = HashMap::from([(start, 0)]);
    let mut f_score = HashMap::from([(start, h(&start, &end))]);

    while let Some(current) = open_set.pop() {
        if current == end {
            return Some(reconstruct_path(&came_from, &current));
        }

        let mut inserted = false;
        for neighbor in map.neighbors(&current) {
            let tentative_g_score = g_score.get(&current).unwrap() + 1;
            if tentative_g_score < *g_score.get(&neighbor).unwrap_or(&u16::MAX) {
                came_from.insert(neighbor, current);
                g_score.insert(neighbor, tentative_g_score);
                f_score.insert(neighbor, tentative_g_score + h(&neighbor, &end));
                if !open_set.contains(&neighbor) {
                    open_set.push(neighbor);
                    inserted = true;
                }
            }
        }
        if inserted {
            open_set.sort_unstable_by_key(|x| f_score.get(x).unwrap());
            open_set.reverse();
        }
    }
    None
}

fn main() {
    let input = include_str!("../input/input.txt");
    let bytes = parse_input(input);
    let nb_bytes = if bytes.len() < 100 { 12 } else { 1024 };
    let mut map = Map::new(&bytes[..nb_bytes]);

    let path = find_path(&map);
    println!(
        "The shortest path after {} fallen bytes is {} steps long.",
        nb_bytes,
        path.unwrap().len() - 1
    );

    for byte in &bytes[nb_bytes..] {
        map.corrupted.push(*byte);
        if find_path(&map).is_none() {
            println!("After byte {}, the end is not reachable anymore.", byte);
            break;
        }
    }
}
