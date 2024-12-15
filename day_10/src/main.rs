use std::{collections::HashSet, str::FromStr};

struct Map {
    data: Vec<u8>,
    height: usize,
    width: usize,
}

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
struct Coordinates {
    x: usize,
    y: usize,
}

impl Coordinates {
    fn up(&self) -> Option<Self> {
        self.y.checked_sub(1).map(|y| Self { y, ..*self })
    }
    fn left(&self) -> Option<Self> {
        self.x.checked_sub(1).map(|x| Self { x, ..*self })
    }
    fn down(&self) -> Option<Self> {
        Some(Self {
            y: self.y + 1,
            ..*self
        })
    }
    fn right(&self) -> Option<Self> {
        Some(Self {
            x: self.x + 1,
            ..*self
        })
    }
}

impl FromStr for Map {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let data = input
            .chars()
            .filter_map(|c| c.to_digit(10).and_then(|c| Some(c as u8)))
            .collect::<Vec<_>>();
        let width = input.find('\n').unwrap();
        let height = data.len() / width;
        Ok(Self {
            data,
            width,
            height,
        })
    }
}

struct MapIterator<'a> {
    map: &'a Map,
    index: usize,
}

impl Iterator for MapIterator<'_> {
    type Item = (Coordinates, u8);

    fn next(&mut self) -> Option<Self::Item> {
        self.map.data.get(self.index).and_then(|value| {
            let x = self.index % self.map.width;
            let y = self.index / self.map.width;
            self.index += 1;
            Some((Coordinates { x, y }, *value))
        })
    }
}

impl Map {
    fn iter(&self) -> MapIterator {
        MapIterator {
            map: self,
            index: 0,
        }
    }
    fn get(&self, coordinates: &Coordinates) -> Option<&u8> {
        if !(0..self.width).contains(&coordinates.x) {
            return None;
        }
        let index = coordinates.y * self.width + coordinates.x;
        self.data.get(index)
    }

    fn find_paths(&self, coordinates: &Coordinates, value: u8) -> Option<Vec<Vec<Coordinates>>> {
        if self.get(coordinates) != Some(&value) {
            return None;
        }
        if value == 9 {
            return Some(vec![vec![*coordinates]]);
        }

        let next_value = value + 1;
        let mut paths = vec![];
        if let Some(new_coordinates) = coordinates.down() {
            if let Some(mut new_paths) = self.find_paths(&new_coordinates, next_value) {
                new_paths
                    .iter_mut()
                    .for_each(|path| path.push(*coordinates));
                paths.append(&mut new_paths);
            };
        }
        if let Some(new_coordinates) = coordinates.up() {
            if let Some(mut new_paths) = self.find_paths(&new_coordinates, next_value) {
                new_paths
                    .iter_mut()
                    .for_each(|path| path.push(*coordinates));
                paths.append(&mut new_paths);
            };
        }
        if let Some(new_coordinates) = coordinates.left() {
            if let Some(mut new_paths) = self.find_paths(&new_coordinates, next_value) {
                new_paths
                    .iter_mut()
                    .for_each(|path| path.push(*coordinates));
                paths.append(&mut new_paths);
            };
        }
        if let Some(new_coordinates) = coordinates.right() {
            if let Some(mut new_paths) = self.find_paths(&new_coordinates, next_value) {
                new_paths
                    .iter_mut()
                    .for_each(|path| path.push(*coordinates));
                paths.append(&mut new_paths);
            };
        }

        if paths.is_empty() {
            None
        } else {
            Some(paths)
        }
    }
}

fn main() {
    let input = include_str!("../input/input.txt");
    let map = Map::from_str(&input).unwrap();
    let scores = map
        .iter()
        .filter(|(_, v)| *v == 0)
        .filter_map(|(coordinates, v)| map.find_paths(&coordinates, v))
        .map(|paths| {
            let x = paths.iter().map(|path| path[0]).collect::<HashSet<_>>();
            x.len()
        })
        .collect::<Vec<_>>();
    let scores = scores.iter().sum::<usize>();
    println!(
        "The sum of the scores of all trailheads on the map is {}.",
        scores
    )
}
