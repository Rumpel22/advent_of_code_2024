use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
    usize,
};

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

#[derive(Eq, Hash, PartialEq, Clone, Copy)]
struct State {
    position: XY,
    direction: Direction,
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
        enum Turn {
            Left,
            Right,
        }

        enum Command {
            Turn(Turn),
            Walk,
        }

        impl Turn {
            fn turn(&self, direction: &Direction) -> Direction {
                match (direction, self) {
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

        while self.count < 3 {
            let command = match self.count {
                0 => Command::Walk,
                1 => Command::Turn(Turn::Left),
                2 => Command::Turn(Turn::Right),
                _ => unreachable!(),
            };
            self.count += 1;
            let new_state = match command {
                Command::Turn(turn) => State {
                    direction: turn.turn(&self.state.direction),
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
    let manhatten = position.x.abs_diff(end.x) + position.y.abs_diff(end.y);
    manhatten
}

fn reconstruct_paths(came_from: &HashMap<State, Vec<State>>, state: State) -> Vec<Vec<XY>> {
    if let Some(prev_states) = came_from.get(&state) {
        let mut paths = vec![];
        for prev_state in prev_states {
            let mut new_paths = reconstruct_paths(came_from, *prev_state);
            for new_path in &mut new_paths {
                new_path.push(state.position);
            }
            paths.extend_from_slice(&mut new_paths);
        }
        paths
    } else {
        vec![vec![state.position]]
    }
}

fn find_paths(maze: &Maze) -> Vec<Vec<XY>> {
    let start = State {
        position: maze.start,
        direction: Direction::East,
    };
    let mut open_set = vec![start];
    let mut came_from = HashMap::new();
    let mut g_score = HashMap::from([(start, 0)]);
    let mut f_score = HashMap::from([(start, h(start, maze.end))]);
    let mut winning_score = None;
    let mut winners = vec![];

    while let Some(state) = open_set.pop() {
        if g_score.get(&state).unwrap() > &winning_score.unwrap_or(usize::MAX) {
            break;
        }

        if state.position == maze.end {
            let score = *g_score.get(&state).unwrap();
            winning_score.get_or_insert_with(|| {
                println!("The total cost from start to end is {}.", score);
                score
            });
            winners.push(state);
        }

        for new_state in maze.next_states(state) {
            let tentative_g_score = g_score.get(&state).unwrap()
                + if new_state.direction == state.direction {
                    1
                } else {
                    1000
                };
            match tentative_g_score.cmp(g_score.get(&new_state).unwrap_or(&usize::MAX)) {
                std::cmp::Ordering::Less => {
                    came_from.insert(new_state, vec![state]);
                    g_score.insert(new_state, tentative_g_score);
                    f_score.insert(new_state, tentative_g_score + h(new_state, maze.end));
                    if !open_set.contains(&new_state) {
                        open_set.push(new_state);
                    }
                }
                std::cmp::Ordering::Equal => came_from.get_mut(&new_state).unwrap().push(state),
                std::cmp::Ordering::Greater => (),
            }
        }
        open_set.sort_unstable_by_key(|x| f_score.get(x).unwrap());
        open_set.reverse();
    }
    winners
        .iter()
        .flat_map(|winner| reconstruct_paths(&came_from, *winner))
        .collect::<Vec<_>>()
}

fn main() {
    let input = include_str!("../input/input.txt");
    let maze = Maze::from_str(&input).unwrap();
    let paths = find_paths(&maze);
    println!(
        "There are {} different paths through the maze.",
        paths.len()
    );
    let best_tiles = paths.iter().flatten().collect::<HashSet<_>>();
    println!("There are {} best tiles on these paths.", best_tiles.len());
}
