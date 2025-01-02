use std::{collections::HashMap, iter};

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

fn h(position: &XY, end: &XY) -> u8 {
    position.x.abs_diff(end.x) + position.y.abs_diff(end.y)
}

fn reconstruct_path(came_from: &HashMap<XY, XY>, current: &XY) -> Vec<XY> {
    iter::successors(Some(current), |x| came_from.get(x))
        .cloned()
        .collect()
}

fn find_path(map: &Map) -> Vec<XY> {
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
            return reconstruct_path(&came_from, &current);
        }

        for neighbor in map.neighbors(&current) {
            let tentative_g_score = g_score.get(&current).unwrap() + 1;
            if tentative_g_score < *g_score.get(&neighbor).unwrap_or(&u8::MAX) {
                came_from.insert(neighbor, current);
                g_score.insert(neighbor, tentative_g_score);
                f_score.insert(neighbor, tentative_g_score + h(&neighbor, &end));
                if !open_set.contains(&neighbor) {
                    open_set.push(neighbor);
                }
            }
        }
        open_set.sort_unstable_by_key(|x| f_score.get(x).unwrap());
        open_set.reverse();
    }
    unreachable!()
}

fn main() {
    let input = include_str!("../input/input.txt");
    let bytes = parse_input(input);
    let nb_bytes = if bytes.len() < 100 { 12 } else { 1024 };
    let map = Map::new(&bytes[..nb_bytes]);

    let path = find_path(&map);
    println!("The shortest path is {} long.", path.len() - 1);
}
