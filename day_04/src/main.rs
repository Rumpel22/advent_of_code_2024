struct Field {
    data: &'static str,
    width: i32,
    height: i32,
}

#[derive(Clone, PartialEq)]
enum Direction {
    North,
    East,
    South,
    West,
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
}

impl Direction {
    fn offsets(&self) -> (i32, i32) {
        match self {
            Direction::North => (-1, 0),
            Direction::East => (0, 1),
            Direction::South => (1, 0),
            Direction::West => (0, -1),
            Direction::NorthEast => (-1, 1),
            Direction::NorthWest => (-1, -1),
            Direction::SouthEast => (1, 1),
            Direction::SouthWest => (1, -1),
        }
    }
}

struct DirectionIterator {
    count: usize,
}

impl DirectionIterator {
    fn new() -> Self {
        Self { count: 0 }
    }
    fn diagonals() -> Self {
        Self { count: 4 }
    }
}

impl Iterator for DirectionIterator {
    type Item = Direction;

    fn next(&mut self) -> Option<Self::Item> {
        let n = match self.count {
            0 => Some(Direction::North),
            1 => Some(Direction::East),
            2 => Some(Direction::South),
            3 => Some(Direction::West),
            4 => Some(Direction::NorthEast),
            5 => Some(Direction::NorthWest),
            6 => Some(Direction::SouthEast),
            7 => Some(Direction::SouthWest),
            _ => None,
        };
        self.count += 1;
        n
    }
}

impl Field {
    fn new(data: &'static str) -> Self {
        let width = data.find('\n').unwrap() as i32;
        let height = data.chars().filter(|c| *c != '\n').count() as i32 / width;
        Field {
            data,
            width,
            height,
        }
    }

    fn get(&self, row: i32, column: i32) -> Option<char> {
        if (0..self.width).contains(&column) && (0..self.height).contains(&row) {
            let index = row * (self.width + 1) + column;
            return self.data.chars().nth(index as usize);
        }

        None
    }
}

fn main() {
    let input = include_str!("../input/input.txt");
    let field = Field::new(input);

    let x_iter = input.lines().enumerate().flat_map(|(row, line)| {
        line.chars()
            .enumerate()
            .filter(|(_, c)| *c == 'X')
            .map(move |(column, _)| (row as i32, column as i32))
    });

    let mut count = 0;
    for (row, column) in x_iter {
        for direction in DirectionIterator::new() {
            let offsets = direction.offsets();
            let m = field.get(row + 1 * offsets.0, column + 1 * offsets.1);
            let a = field.get(row + 2 * offsets.0, column + 2 * offsets.1);
            let s = field.get(row + 3 * offsets.0, column + 3 * offsets.1);
            if m.is_some_and(|m| m == 'M')
                && a.is_some_and(|a| a == 'A')
                && s.is_some_and(|s| s == 'S')
            {
                count += 1;
            }
        }
    }
    println!("There are {} XMAS in the filed.", count);

    let a_iter = input.lines().enumerate().flat_map(|(row, line)| {
        line.chars()
            .enumerate()
            .filter(|(_, c)| *c == 'A')
            .map(move |(column, _)| (row as i32, column as i32))
    });
    let mut count = 0;
    for (row, column) in a_iter {
        for direction_1 in DirectionIterator::diagonals() {
            let offsets_1 = direction_1.offsets();
            let m_1 = field.get(row + offsets_1.0, column + offsets_1.1);
            let s_1 = field.get(row - offsets_1.0, column - offsets_1.1);
            for direction_2 in DirectionIterator::diagonals() {
                if direction_1 == direction_2 {
                    continue;
                }
                let offsets_2 = direction_2.offsets();
                let m_2 = field.get(row + offsets_2.0, column + offsets_2.1);
                let s_2 = field.get(row - offsets_2.0, column - offsets_2.1);

                if m_1.is_some_and(|m| m == 'M')
                    && m_2.is_some_and(|m| m == 'M')
                    && s_1.is_some_and(|m| m == 'S')
                    && s_2.is_some_and(|m| m == 'S')
                {
                    count += 1;
                }
            }
        }
    }
    println!("There are {} X-MAS in the filed.", count / 2);
}
