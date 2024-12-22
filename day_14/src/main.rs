use std::str::FromStr;

use bmp::Image;

struct XYPair<T> {
    x: T,
    y: T,
}

struct Robot {
    position: XYPair<u32>,
    velocity: XYPair<i32>,
}

impl FromStr for Robot {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let (p, v) = input.split_once(' ').unwrap();
        let p = &p[2..];
        let (px, py) = p.split_once(',').unwrap();
        let v = &v[2..];
        let (vx, vy) = v.split_once(',').unwrap();

        let position = XYPair {
            x: px.parse().unwrap(),
            y: py.parse().unwrap(),
        };
        let velocity = XYPair {
            x: vx.parse().unwrap(),
            y: vy.parse().unwrap(),
        };
        Ok(Robot { position, velocity })
    }
}

fn move_robot(robot: &Robot, room: &XYPair<u32>, steps: u32) -> XYPair<u32> {
    let x = robot.position.x as i32 + steps as i32 * robot.velocity.x;
    let y = robot.position.y as i32 + steps as i32 * robot.velocity.y;

    let x = x.rem_euclid(room.x as i32) as u32;
    let y = y.rem_euclid(room.y as i32) as u32;
    XYPair { x, y }
}

fn quadrants(positions: &[XYPair<u32>], room: &XYPair<u32>) -> [usize; 4] {
    let mid_x = room.x / 2;
    let mid_y = room.y / 2;
    let mut top_left = 0;
    let mut top_right = 0;
    let mut bottom_left = 0;
    let mut bottom_right = 0;
    positions
        .iter()
        .for_each(|position| match (position.x, position.y) {
            (x, y) if x < mid_x && y < mid_y => top_left += 1,
            (x, y) if x < mid_x && y > mid_y => bottom_left += 1,
            (x, y) if x > mid_x && y < mid_y => top_right += 1,
            (x, y) if x > mid_x && y > mid_y => bottom_right += 1,
            _ => {}
        });

    [top_left, top_right, bottom_left, bottom_right]
}

fn show_robots(positions: &[XYPair<u32>], room: &XYPair<u32>, iteration: u32) {
    let mut image = Image::new(room.x as u32, room.y);
    for position in positions {
        image.set_pixel(position.x, position.y, bmp::consts::WHITE);
    }
    image
        .save(format!(
            "/workspaces/rust/day_14/images/it_{}.bmp",
            iteration
        ))
        .unwrap();
}

fn main() {
    let input = include_str!("../input/input.txt");
    let robots = input
        .lines()
        .map(|line| Robot::from_str(line).unwrap())
        .collect::<Vec<_>>();
    let room = if robots.len() < 20 {
        XYPair { x: 11, y: 7 }
    } else {
        XYPair { x: 101, y: 103 }
    };

    let moved_robots = robots
        .iter()
        .map(|robot| move_robot(robot, &room, 100))
        .collect::<Vec<_>>();
    let per_quadrant = quadrants(&moved_robots, &room);

    let safety_factor = per_quadrant.iter().product::<usize>();
    println!("The safety factor after 100 secondes is {}.", safety_factor);

    for iteration in 0..10000 {
        let moved_robots = robots
            .iter()
            .map(|robot| move_robot(robot, &room, iteration))
            .collect::<Vec<_>>();
        show_robots(&moved_robots, &room, iteration);
    }
}
