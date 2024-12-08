use std::collections::HashSet;

struct Map {
    data: &'static str,
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coordinates {
    x: i32,
    y: i32,
}

impl Map {
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
    fn height(&self) -> i32 {
        self.data.lines().count() as i32
    }
    fn width(&self) -> i32 {
        self.data.find('\n').unwrap() as i32
    }
    fn is_blocked(&self, coordinates: &Coordinates) -> bool {
        let index = (coordinates.x + coordinates.y * (self.width() + 1)) as usize;
        self.data.bytes().nth(index) == Some(b'#')
    }
}

#[derive(Debug)]
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

fn main() {
    let input = include_str!("../input/input.txt");
    let map = Map { data: input };
    let mut position = map.start_position();
    let mut direction = Direction::Up;
    let mut positions = HashSet::new();
    loop {
        println!("{:?}", position);

        positions.insert(position);
        let new_position = position.go(&direction);
        if map.is_blocked(&new_position) {
            direction = direction.turn_right();
            println!("New direction: {:?}", direction);
        } else {
            position = new_position;
            if !(0..map.width()).contains(&position.x) || !(0..map.height()).contains(&position.y) {
                break;
            }
        }
    }
    println!(
        "The guard visits {} positions before leaving the mapped area.",
        positions.len()
    );
}
