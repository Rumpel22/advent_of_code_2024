use std::collections::HashSet;

struct Map {
    data: &'static str,
    height: i32,
    width: i32,
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coordinates {
    x: i32,
    y: i32,
}

impl Map {
    fn new(data: &'static str) -> Self {
        Self {
            data,
            height: data.lines().count() as i32,
            width: data.find('\n').unwrap() as i32,
        }
    }
    fn start_position(&self) -> Coordinates {
        self.data
            .lines()
            .enumerate()
            .filter_map(|(y, line)| {
                line.find('^').map(|x| Coordinates {
                    x: x as i32,
                    y: y as i32,
                })
            })
            .next()
            .unwrap()
    }
    fn is_blocked(&self, coordinates: &Coordinates) -> bool {
        let index = (coordinates.x + coordinates.y * (self.width + 1)) as usize;
        self.data.bytes().nth(index) == Some(b'#')
    }
    fn on_map(&self, coordinates: &Coordinates) -> bool {
        (0..self.width).contains(&coordinates.x) && (0..self.height).contains(&coordinates.y)
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn turn_right(&self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }
}

impl Coordinates {
    fn go(self, direction: &Direction) -> Self {
        match direction {
            Direction::Up => Self {
                y: self.y - 1,
                ..self
            },
            Direction::Down => Self {
                y: self.y + 1,
                ..self
            },
            Direction::Left => Self {
                x: self.x - 1,
                ..self
            },
            Direction::Right => Self {
                x: self.x + 1,
                ..self
            },
        }
    }
}

fn unique_coordinates(positions: &[Coordinates]) -> HashSet<Coordinates> {
    HashSet::from_iter(positions.iter().cloned())
}

fn take_step(
    map: &Map,
    position: &Coordinates,
    direction: &Direction,
) -> Option<(Coordinates, Direction)> {
    let mut new_position = position.go(&direction);
    let mut new_direction = *direction;
    if map.is_blocked(&new_position) {
        new_position = *position;
        new_direction = direction.turn_right();
    } else {
        if !map.on_map(&new_position) {
            return None;
        }
    }
    Some((new_position, new_direction))
}

fn walk(map: &Map, start: &Coordinates, direction: &Direction) -> Vec<Coordinates> {
    let mut position = *start;
    let mut direction = *direction;
    let mut positions = vec![];
    loop {
        positions.push(position);
        if let Some((new_position, new_direction)) = take_step(map, &position, &direction) {
            position = new_position;
            direction = new_direction;
        } else {
            break;
        }
    }
    positions
}

fn main() {
    let input = include_str!("../input/input.txt");
    let map = Map::new(input);
    let start = map.start_position();

    let positions = walk(&map, &start, &Direction::Up);
    let unique_positions = unique_coordinates(&positions);
    println!(
        "The guard visits {} positions before leaving the mapped area.",
        unique_positions.len()
    );

    let block_positions = positions
        .iter()
        .skip(1)
        .filter(|block| {
            let mut turning_points = HashSet::new();
            let mut position = start;
            let mut direction = Direction::Up;

            loop {
                let new_position = position.go(&direction);
                if !map.on_map(&new_position) {
                    return false;
                }
                if map.is_blocked(&new_position) || new_position == **block {
                    if !turning_points.insert((position, direction)) {
                        return true;
                    }
                    direction = direction.turn_right();
                } else {
                    position = new_position;
                }
            }
        })
        .cloned()
        .collect::<Vec<_>>();

    let unique_block_positions = unique_coordinates(&block_positions);
    println!(
        "There are {} different positions to place an obstruction",
        unique_block_positions.len()
    );
}
