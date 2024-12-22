use std::{
    ops::{Index, IndexMut},
    str::FromStr,
};

#[derive(Clone, Copy, PartialEq)]
enum Tile {
    Wall,
    WideBox(bool),
    Box,
    Space,
    Robot,
}

#[derive(Clone)]
struct Map {
    tiles: Vec<Tile>,
    height: usize,
    width: usize,
}

#[derive(Clone, Copy, PartialEq)]
enum Move {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Copy, PartialEq)]
struct XY {
    x: usize,
    y: usize,
}

impl XY {
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
}

impl Map {
    fn move_robot(&self, direction: &Move) -> Self {
        let robot_position = self.robot();
        self.move_tile(robot_position, direction)
            .unwrap_or_else(|| self.clone())
    }

    fn move_tile(&self, xy: XY, direction: &Move) -> Option<Self> {
        let tile = self[&xy];
        match tile {
            Tile::Wall => None,
            Tile::Space => Some(self.clone()),
            Tile::WideBox(left_part) => {
                let (left_xy, right_xy) = if left_part {
                    (xy, xy.next(Move::Right))
                } else {
                    (xy.next(Move::Left), xy)
                };
                let (first_xy, second_xy) = if *direction == Move::Left {
                    (left_xy, right_xy)
                } else {
                    (right_xy, left_xy)
                };

                let new_first = first_xy.next(*direction);
                let new_second = second_xy.next(*direction);

                if let Some(mut map) = self.move_tile(new_first, direction) {
                    map[&new_first] = map[&first_xy];
                    map[&first_xy] = Tile::Space;
                    if let Some(mut map) = map.move_tile(new_second, direction) {
                        map[&new_second] = map[&second_xy];
                        map[&second_xy] = Tile::Space;
                        return Some(map);
                    }
                }
                return None;
            }
            tile => {
                let new_spot = xy.next(*direction);
                if let Some(mut map) = self.move_tile(new_spot, direction) {
                    map[&new_spot] = tile;
                    map[&xy] = Tile::Space;
                    Some(map)
                } else {
                    None
                }
            }
        }
    }

    fn sum_of_box_gps(&self) -> usize {
        self.tiles
            .iter()
            .enumerate()
            .filter_map(|(index, tile)| match tile {
                Tile::Box => Some((index / self.width * 100) + (index % self.width)),
                Tile::WideBox(true) => Some((index / self.width * 100) + (index % self.width)),
                _ => None,
            })
            .sum::<usize>()
    }

    fn robot(&self) -> XY {
        let robot_index = self.tiles.iter().position(|t| *t == Tile::Robot).unwrap();
        let x = robot_index % self.width;
        let y = robot_index / self.width;
        XY { x, y }
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

        Ok(Map {
            tiles,
            height,
            width,
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
    let orig_map = Map::from_str(map_str).unwrap();
    let moves = moves(moves_str);

    let map = moves
        .iter()
        .fold(orig_map.clone(), |map, direction| map.move_robot(direction));

    let sum_of_box_gps = map.sum_of_box_gps();
    println!("The sum of the GPS coordinates is {}.", sum_of_box_gps);

    let wide_tiles = orig_map
        .tiles
        .iter()
        .flat_map(|tile| match tile {
            Tile::Box => [Tile::WideBox(true), Tile::WideBox(false)],
            Tile::Robot => [Tile::Robot, Tile::Space],
            t => [*t, *t],
        })
        .collect::<Vec<_>>();
    let wide_map = Map {
        height: orig_map.height,
        width: orig_map.width * 2,
        tiles: wide_tiles,
    };
    let map = moves
        .iter()
        .fold(wide_map.clone(), |map, direction| map.move_robot(direction));

    let sum_of_box_gps = map.sum_of_box_gps();
    println!(
        "The sum of the GPS coordinates on the wide map is {}.",
        sum_of_box_gps
    );
}
