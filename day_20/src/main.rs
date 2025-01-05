use std::{collections::HashMap, iter, ops::RangeInclusive, str::FromStr};

#[derive(PartialEq)]
enum Tile {
    Wall,
    Track,
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn iter() -> impl Iterator<Item = Direction> {
        iter::successors(Some(Direction::Up), |direction| match direction {
            Direction::Up => Some(Direction::Down),
            Direction::Down => Some(Direction::Left),
            Direction::Left => Some(Direction::Right),
            Direction::Right => None,
        })
    }
}
#[derive(Default, Clone, Copy, Eq, Hash, PartialEq)]
struct XY {
    x: usize,
    y: usize,
}

impl XY {
    fn go(&self, direction: &Direction) -> Option<Self> {
        match direction {
            Direction::Up if self.y > 0 => Some(Self {
                y: self.y - 1,
                ..*self
            }),
            Direction::Down => Some(Self {
                y: self.y + 1,
                ..*self
            }),
            Direction::Left if self.x > 0 => Some(Self {
                x: self.x - 1,
                ..*self
            }),
            Direction::Right => Some(Self {
                x: self.x + 1,
                ..*self
            }),
            _ => None,
        }
    }

    fn neighbors(&self) -> impl Iterator<Item = XY> + use<'_> {
        Direction::iter().filter_map(|direction| self.go(&direction))
    }
}

struct Maze {
    start: XY,
    end: XY,
    tiles: Vec<Tile>,
    width: usize,
    _height: usize,
}

impl FromStr for Maze {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let tiles = input
            .chars()
            .filter_map(|c| match c {
                '#' => Some(Tile::Wall),
                '\n' => None,
                _ => Some(Tile::Track),
            })
            .collect::<Vec<_>>();
        let width = input.find('\n').unwrap();
        let height = tiles.len() / width;

        let start_index = input.find('S').unwrap();
        let start_y = start_index / (width + 1);
        let start_x = start_index % (width + 1);
        let start = XY {
            x: start_x,
            y: start_y,
        };

        let end_index = input.find('E').unwrap();
        let end_y = end_index / (width + 1);
        let end_x = end_index % (width + 1);
        let end = XY { x: end_x, y: end_y };
        Ok(Maze {
            start,
            end,
            tiles,
            width,
            _height: height,
        })
    }
}

impl Maze {
    fn get(&self, xy: &XY) -> Option<&Tile> {
        if xy.x >= self.width {
            return None;
        }
        let index = self.width * xy.y + xy.x;
        self.tiles.get(index)
    }
}

fn find_path(maze: &Maze) -> Vec<XY> {
    let mut current = maze.start;
    let mut path = vec![current];
    while current != maze.end {
        let next = current
            .neighbors()
            .filter(|neighbor| maze.get(neighbor).unwrap() == &Tile::Track)
            .filter(|neighbor| !path.contains(neighbor))
            .next()
            .unwrap();
        current = next;
        path.push(current);
    }
    path
}

fn main() {
    let input = include_str!("../input/input.txt");
    let maze = Maze::from_str(&input).unwrap();
    let path = find_path(&maze);
    let length = path.len();
    let distances = path
        .iter()
        .enumerate()
        .map(|(step, xy)| (*xy, length - step - 1))
        .collect::<HashMap<_, _>>();

    let min_save = 100;

    let cheat_count = path
        .iter()
        .take_while(|tile| distances.get(tile).unwrap() >= &min_save)
        .flat_map(|tile| {
            Direction::iter().map({
                |direction| {
                    if let Some(cheat) = tile
                        .go(&direction)
                        .and_then(|neighbor| neighbor.go(&direction))
                        .filter(|xy| maze.get(xy) == Some(&Tile::Track))
                    {
                        let tile_distance = *distances.get(tile).unwrap();
                        let cheat_distance = *distances.get(&cheat).unwrap();
                        if cheat_distance < tile_distance {
                            return Some(tile_distance - cheat_distance - 2);
                        }
                    }
                    None
                }
            })
        })
        .filter_map(|x| x)
        .filter(|size| *size >= min_save)
        .count();
    println!(
        "There are {} cheats through the walls for at least 100ps.",
        cheat_count
    );

    let cheat_count = path
        .iter()
        .take_while(|tile| distances.get(tile).unwrap() >= &min_save)
        .flat_map(|tile| {
            let x_range: RangeInclusive<i32> = -20..=20;

            x_range
                .flat_map(|x_offset| {
                    let y_limit = 20 - x_offset.abs();
                    let y_range = -y_limit..=y_limit;
                    y_range.map(move |y_offset| (x_offset, y_offset))
                })
                .filter_map(|(x_offset, y_offset)| {
                    let x = (x_offset + tile.x as i32).clamp(0, i32::MAX) as usize;
                    let y = (y_offset + tile.y as i32).clamp(0, i32::MAX) as usize;
                    let cheat = XY { x, y };

                    if maze.get(&cheat) == Some(&Tile::Track) {
                        let tile_distance = *distances.get(tile).unwrap();
                        let cheat_distance = *distances.get(&cheat).unwrap();

                        let cheated = x_offset.abs() + y_offset.abs();
                        if cheat_distance < tile_distance {
                            return Some(tile_distance - cheat_distance - cheated as usize);
                        }
                    }
                    None
                })
        })
        .filter(|size| *size >= min_save)
        .count();
    println!(
        "If real cheating is allowed, there are {} cheats for at least 100ps.",
        cheat_count
    );
}
