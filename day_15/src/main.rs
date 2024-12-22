use std::{
    ops::{Index, IndexMut},
    str::FromStr,
};

#[derive(Clone, Copy, PartialEq)]
enum Tile {
    Wall,
    Box,
    Space,
    Robot,
}

#[derive(Clone)]
struct Map {
    tiles: Vec<Tile>,
    height: usize,
    width: usize,
    robot: XY,
}

#[derive(Clone, Copy)]
enum Move {
    Up,
    Down,
    Left,
    Right,
}

impl Move {
    fn rev(&self) -> Self {
        match self {
            Move::Up => Move::Down,
            Move::Down => Move::Up,
            Move::Left => Move::Right,
            Move::Right => Move::Left,
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
struct XY {
    x: usize,
    y: usize,
}

impl XY {
    fn iter(&self, direction: Move) -> XYIterator {
        XYIterator {
            next: Some(*self),
            direction,
        }
    }
    fn next(&self, direction: Move) -> Self {
        match direction {
            Move::Up => XY {
                y: self.y.saturating_sub(1),
                ..*self
            },
            Move::Down => XY {
                y: self.y + 1,
                ..*self
            },
            Move::Left => XY {
                x: self.x.saturating_sub(1),
                ..*self
            },
            Move::Right => XY {
                x: self.x + 1,
                ..*self
            },
        }
    }
    fn distance(&self, other: &Self) -> usize {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }
}

struct XYIterator {
    next: Option<XY>,
    direction: Move,
}

impl Iterator for XYIterator {
    type Item = XY;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.next?.next(self.direction);
        let current = self.next;
        self.next = Some(next);
        current
    }
}

impl Map {
    fn move_robot(&self, direction: &Move) -> Self {
        let stop = self
            .robot
            .iter(*direction)
            .skip_while(|xy| self[xy] != Tile::Wall && self[xy] != Tile::Space)
            .next()
            .unwrap();
        let steps = stop.distance(&self.robot);
        let map =
            stop.iter(direction.rev())
                .skip(1)
                .take(steps)
                .fold(self.clone(), |mut map, xy| {
                    match map[&xy] {
                        Tile::Wall => panic!("Should not happen"),
                        Tile::Space => (),
                        tile => {
                            let prev = xy.next(*direction);
                            if map[&prev] == Tile::Space {
                                map[&prev] = tile;
                                map[&xy] = Tile::Space;
                                if tile == Tile::Robot {
                                    map.robot = prev;
                                }
                            }
                        }
                    };
                    map
                });
        map
    }
}

impl Index<&XY> for Map {
    type Output = Tile;

    fn index(&self, index: &XY) -> &Self::Output {
        self.tiles.get(index.y * self.width + index.x).unwrap()
    }
}

impl IndexMut<&XY> for Map {
    fn index_mut(&mut self, index: &XY) -> &mut Self::Output {
        self.tiles.get_mut(index.y * self.width + index.x).unwrap()
    }
}

impl FromStr for Map {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tiles = s
            .chars()
            .filter_map(|c| match c {
                '#' => Some(Tile::Wall),
                '.' => Some(Tile::Space),
                'O' => Some(Tile::Box),
                '@' => Some(Tile::Robot),
                '\n' => None,
                _ => panic!("Invalid character"),
            })
            .collect::<Vec<_>>();

        let width = s.find('\n').unwrap();
        let height = tiles.len() / width;
        let robot_index = tiles.iter().position(|t| matches!(t, Tile::Robot)).unwrap();
        let y = robot_index / width;
        let x = robot_index % width;
        let robot = XY { x, y };

        Ok(Map {
            tiles,
            height,
            width,
            robot,
        })
    }
}

fn moves(input: &str) -> Vec<Move> {
    input
        .chars()
        .filter_map(|c| match c {
            '<' => Some(Move::Left),
            '>' => Some(Move::Right),
            'v' => Some(Move::Down),
            '^' => Some(Move::Up),
            '\n' => None,
            _ => panic!("Invalid character"),
        })
        .collect()
}

fn main() {
    let input = include_str!("../input/input.txt");
    let (map_str, moves_str) = input.split_once("\n\n").unwrap();
    let map = Map::from_str(map_str).unwrap();
    let moves = moves(moves_str);

    let map = moves.iter().fold(map, |map, m| map.move_robot(m));

    let sum_of_box_gps = map
        .tiles
        .iter()
        .enumerate()
        .filter_map(|(index, tile)| match tile {
            Tile::Box => Some((index / map.width * 100) + (index % map.width)),
            _ => None,
        })
        .sum::<usize>();
    println!("The sum of the GPS coordinates is {}.", sum_of_box_gps);
}
