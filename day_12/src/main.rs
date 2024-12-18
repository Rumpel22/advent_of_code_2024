use std::{collections::HashMap, str::FromStr, vec};

#[derive(Eq, PartialEq, Hash, Clone, Copy)]
struct Coordinates {
    x: i32,
    y: i32,
}
impl Coordinates {
    fn up(&self) -> Self {
        Self {
            y: self.y - 1,
            ..*self
        }
    }
    fn down(&self) -> Self {
        Self {
            y: self.y + 1,
            ..*self
        }
    }
    fn left(&self) -> Self {
        Self {
            x: self.x - 1,
            ..*self
        }
    }
    fn right(&self) -> Self {
        Self {
            x: self.x + 1,
            ..*self
        }
    }
}

struct Map {
    fields: HashMap<Coordinates, char>,
    height: usize,
    width: usize,
}

type Region = Vec<Coordinates>;

impl FromStr for Map {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let map = input
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.char_indices().map(move |(x, c)| {
                    (
                        Coordinates {
                            x: x as i32,
                            y: y as i32,
                        },
                        c,
                    )
                })
            })
            .collect::<HashMap<_, _>>();
        let width = input.find('\n').unwrap();
        let height = map.len() / width;
        Ok(Self {
            fields: map,
            width,
            height,
        })
    }
}

impl Map {
    fn regions(&self) -> Vec<Region> {
        let mut regions = vec![];
        let mut visited = vec![];

        for x in 0..self.width {
            for y in 0..self.height {
                let coordinates = Coordinates {
                    x: x as i32,
                    y: y as i32,
                };
                if visited.contains(&coordinates) {
                    continue;
                }
                let mut to_visit = vec![coordinates];
                let thing = self.fields.get(&coordinates).unwrap();

                let mut region = Region::new();
                while let Some(field) = to_visit.pop() {
                    if region.contains(&field) {
                        continue;
                    }
                    visited.push(field);
                    region.push(field);
                    if let Some((up_field, up_thing)) = self.fields.get_key_value(&field.up()) {
                        if up_thing == thing {
                            to_visit.push(*up_field);
                        }
                    }
                    if let Some((down_field, down_thing)) = self.fields.get_key_value(&field.down())
                    {
                        if down_thing == thing {
                            to_visit.push(*down_field);
                        }
                    }
                    if let Some((left_field, left_thing)) = self.fields.get_key_value(&field.left())
                    {
                        if left_thing == thing {
                            to_visit.push(*left_field);
                        }
                    }
                    if let Some((right_field, right_thing)) =
                        self.fields.get_key_value(&field.right())
                    {
                        if right_thing == thing {
                            to_visit.push(*right_field);
                        }
                    }
                }
                regions.push(region);
            }
        }
        regions
    }
}

trait Area {
    fn area(&self) -> usize;
}
trait Perimeter {
    fn perimeter(&self) -> usize;
}
impl Area for Region {
    fn area(&self) -> usize {
        self.len()
    }
}

impl Perimeter for Region {
    fn perimeter(&self) -> usize {
        self.iter()
            .map(|coordinates| {
                let mut count = 0;
                if !self.contains(&coordinates.up()) {
                    count += 1;
                }
                if !self.contains(&coordinates.down()) {
                    count += 1;
                }
                if !self.contains(&coordinates.left()) {
                    count += 1;
                }
                if !self.contains(&coordinates.right()) {
                    count += 1;
                }
                count
            })
            .sum()
    }
}

fn main() {
    let input = include_str!("../input/input.txt");
    let map = Map::from_str(input).unwrap();

    let regions = map.regions();
    assert_eq!(
        regions.iter().map(|region| region.len()).sum::<usize>(),
        map.height * map.width
    );
    let price = regions
        .iter()
        .map(|region| region.area() * region.perimeter())
        .sum::<usize>();
    println!("The total price of fencing all regions is {}.", price)
}
