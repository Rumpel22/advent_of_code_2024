use std::{collections::HashMap, str::FromStr};

#[derive(PartialEq, Clone, Copy)]
enum Tile {
    Wall,
    Open,
}

#[derive(PartialEq, Clone, Copy, Eq, Hash, Debug)]
enum Direction {
    North,
    East,
    South,
    West,
}

enum Turn {
    Left,
    Right,
}

enum Command {
    Turn(Turn),
    Walk,
}

#[derive(Eq, Hash, PartialEq, Clone, Copy)]
struct State {
    position: XY,
    direction: Direction,
}

impl Direction {
    fn rev(&self) -> Self {
        match self {
            Direction::North => Direction::South,
            Direction::East => Direction::West,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
        }
    }
    fn turn(&self, turn: &Turn) -> Self {
        match (self, turn) {
            (Direction::North, Turn::Left) => Direction::West,
            (Direction::North, Turn::Right) => Direction::East,
            (Direction::East, Turn::Left) => Direction::North,
            (Direction::East, Turn::Right) => Direction::South,
            (Direction::South, Turn::Left) => Direction::East,
            (Direction::South, Turn::Right) => Direction::West,
            (Direction::West, Turn::Left) => Direction::South,
            (Direction::West, Turn::Right) => Direction::North,
        }
    }
}

#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
struct XY {
    x: usize,
    y: usize,
}

impl XY {
    fn go(&self, direction: Direction) -> Self {
        match direction {
            Direction::North => Self {
                y: self.y - 1,
                ..*self
            },
            Direction::East => Self {
                x: self.x + 1,
                ..*self
            },
            Direction::South => Self {
                y: self.y + 1,
                ..*self
            },
            Direction::West => Self {
                x: self.x - 1,
                ..*self
            },
        }
    }
}

struct Maze {
    tiles: Vec<Tile>,
    start: XY,
    end: XY,
    width: usize,
}

impl FromStr for Maze {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut start = 0;
        let mut end = 0;
        let tiles = input
            .char_indices()
            .filter_map(|(index, c)| match c {
                '#' => Some(Tile::Wall),
                '.' => Some(Tile::Open),
                'S' => {
                    start = index;
                    Some(Tile::Open)
                }
                'E' => {
                    end = index;
                    Some(Tile::Open)
                }
                '\n' => None,
                _ => panic!("Invalid character"),
            })
            .collect::<Vec<_>>();
        let width = input.find('\n').unwrap();
        let start_x = start % (width + 1);
        let start_y = start / (width + 1);
        let end_x = end % (width + 1);
        let end_y = end / (width + 1);
        Ok(Maze {
            tiles,
            start: XY {
                x: start_x,
                y: start_y,
            },
            end: XY { x: end_x, y: end_y },
            width,
        })
    }
}

struct NextStates<'a> {
    maze: &'a Maze,
    state: State,
    count: usize,
}

impl Iterator for NextStates<'_> {
    type Item = State;

    fn next(&mut self) -> Option<Self::Item> {
        while self.count < 3 {
            self.count += 1;
            let command = match self.count {
                1 => Command::Walk,
                2 => Command::Turn(Turn::Left),
                3 => Command::Turn(Turn::Right),
                _ => unreachable!(),
            };
            let new_state = match command {
                Command::Turn(turn) => State {
                    direction: self.state.direction.turn(&turn),
                    ..self.state
                },
                Command::Walk => State {
                    position: self.state.position.go(self.state.direction),
                    ..self.state
                },
            };
            if self.maze.get(new_state.position) == Tile::Open {
                return Some(new_state);
            }
        }
        None
    }
}

impl Maze {
    fn get(&self, position: XY) -> Tile {
        let index = position.y * self.width + position.x;
        *self.tiles.get(index).unwrap()
    }
    fn next_states(&self, state: State) -> NextStates {
        NextStates {
            maze: self,
            state,
            count: 0,
        }
    }
}

fn h(state: State, end: XY) -> usize {
    let position = &state.position;
    let direction = state.direction;
    let manhatten = position.x.abs_diff(end.x) + position.y.abs_diff(end.y);
    let x_turns = match position.x.cmp(&end.x) {
        std::cmp::Ordering::Less if direction == Direction::East => 0,
        std::cmp::Ordering::Less if direction == Direction::West => 2,
        std::cmp::Ordering::Less => 1,
        std::cmp::Ordering::Equal => 0,
        std::cmp::Ordering::Greater if direction == Direction::West => 0,
        std::cmp::Ordering::Greater if direction == Direction::East => 2,
        std::cmp::Ordering::Greater => 1,
    };
    let y_turns = match position.y.cmp(&end.y) {
        std::cmp::Ordering::Less if direction == Direction::North => 0,
        std::cmp::Ordering::Less if direction == Direction::South => 2,
        std::cmp::Ordering::Less => 1,
        std::cmp::Ordering::Equal => 0,
        std::cmp::Ordering::Greater if direction == Direction::South => 0,
        std::cmp::Ordering::Greater if direction == Direction::North => 2,
        std::cmp::Ordering::Greater => 1,
    };

    let turns = (x_turns + y_turns).min(2);
    manhatten + 1000 * turns
}

fn reconstruct_path(came_from: &HashMap<State, State>, mut state: State) -> Vec<Direction> {
    let mut path = vec![state.direction];
    while let Some(previous) = came_from.get(&state) {
        path.push(previous.direction);
        state = *previous;
    }

    path.reverse();
    path
}

fn find_path(maze: &Maze) -> Option<Vec<Direction>> {
    let start = State {
        position: maze.start,
        direction: Direction::East,
    };
    let mut open_set = vec![start];
    let mut came_from = HashMap::new();
    let mut g_score = HashMap::from([(start, 0)]);
    let mut f_score = HashMap::from([(start, h(start, maze.end))]);

    while let Some(state) = open_set.pop() {
        if state.position == maze.end {
            return Some(reconstruct_path(&came_from, state));
        }

        for new_state in maze.next_states(state) {
            let tentative_g_score = g_score.get(&state).unwrap()
                + if new_state.direction == state.direction {
                    1
                } else {
                    1000
                };
            if tentative_g_score < *g_score.get(&new_state).unwrap_or(&usize::MAX) {
                came_from.insert(new_state, state);
                g_score.insert(new_state, tentative_g_score);
                f_score.insert(new_state, tentative_g_score + h(new_state, maze.end));
                if !open_set.contains(&new_state) {
                    open_set.push(new_state);
                    open_set.sort_unstable_by_key(|x| f_score.get(x).unwrap());
                    open_set.reverse();
                }
            }
        }
    }
    None
}

fn calc_cost(path: &[Direction]) -> usize {
    path.windows(2)
        .map(|a| if a[0] == a[1] { 1 } else { 1000 })
        .sum()
}

fn main() {
    let input = include_str!("../input/input.txt");
    let maze = Maze::from_str(&input).unwrap();
    let path = find_path(&maze).unwrap();
    let costs = calc_cost(&path);
    println!("The total cost from start to end is {}.", costs);
}
