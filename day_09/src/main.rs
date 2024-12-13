use std::{iter, usize};

#[derive(Debug)]
struct File {
    length: usize,
    id: usize,
}

#[derive(Debug)]
enum Type {
    File(File),
    Space(usize),
}
fn main() {
    let input = include_str!("../input/input.txt");

    let data = input
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .scan((true, 0), |(is_file, id), length| {
            let t = match is_file {
                true => Type::File(File { id: *id, length }),
                false => Type::Space(length),
            };
            if *is_file {
                *id += 1;
            }
            *is_file = !*is_file;
            Some(t)
        })
        .collect::<Vec<_>>();

    let mut string = data
        .iter()
        .flat_map(|t| match t {
            Type::File(file) => iter::repeat_n(file.id, file.length),
            Type::Space(length) => iter::repeat_n(usize::MAX, *length),
        })
        .collect::<Vec<_>>();
    let mut s = string.as_mut_slice();
    loop {
        let a = s.iter().position(|c| *c == usize::MAX);
        let b = s.iter().rposition(|c| *c != usize::MAX);
        match (a, b) {
            (Some(a), Some(b)) if a < b => {
                s.swap(a, b);
                s = &mut s[a + 1..b];
            }
            _ => break,
        }
    }

    let checksum = string
        .iter()
        .enumerate()
        .take_while(|(_, c)| **c != usize::MAX)
        .map(|(index, c)| c * index)
        .sum::<usize>();

    println!("The checksum is {}.", checksum);
}
