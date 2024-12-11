use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

#[derive(Debug)]
struct Map {
    height: i32,
    width: i32,
    antennas: HashMap<char, Vec<(i32, i32)>>,
}

impl FromStr for Map {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let width = input.find('\n').unwrap();
        let height = input.len() / width;
        let antennas = input
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(move |(x, c)| (c, x as i32, y as i32))
            })
            .filter(|(c, _, _)| *c != '.')
            .fold(
                HashMap::new(),
                |mut map: HashMap<char, Vec<(i32, i32)>>, (c, x, y)| {
                    if let Some(entry) = map.get_mut(&c) {
                        entry.push((x, y));
                    } else {
                        map.insert(c, vec![(x, y)]);
                    };
                    map
                },
            );
        Ok(Self {
            height: height as i32,
            width: width as i32,
            antennas,
        })
    }
}

fn antinods(positions: &[(i32, i32)]) -> Vec<(i32, i32)> {
    let pairs = positions.iter().flat_map(|a| {
        positions
            .iter()
            .filter(move |b| a != *b)
            .map(move |b| (a, b))
    });
    let antinodes = pairs
        .flat_map(|((x1, y1), (x2, y2))| {
            let nx1 = x1 - (x2 - x1);
            let nx2 = x2 + (x2 - x1);
            let ny1 = y1 - (y2 - y1);
            let ny2 = y2 + (y2 - y1);
            [(nx1, ny1), (nx2, ny2)]
        })
        .collect::<Vec<_>>();

    antinodes
}

fn main() {
    let input = include_str!("../input/input.txt");
    let map = Map::from_str(&input).unwrap();

    let unique_antinodes = map
        .antennas
        .iter()
        .flat_map(|(_, positions)| antinods(positions))
        .filter(|(x, y)| (0..map.width).contains(x) && (0..map.height).contains(y))
        .collect::<HashSet<_>>();
    println!(
        "There are {} unique locations containing an antinode.",
        unique_antinodes.len()
    );
}
