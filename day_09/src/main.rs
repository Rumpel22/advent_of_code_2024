use std::{iter, usize};

#[derive(Debug, Clone)]
struct File {
    length: usize,
    id: usize,
}

#[derive(Debug, Clone)]
struct Space {
    length: usize,
}

#[derive(Debug, Clone)]
enum Type {
    File(File),
    Space(Space),
}

impl Type {
    fn length(&self) -> usize {
        match self {
            Type::File(file) => file.length,
            Type::Space(space) => space.length,
        }
    }
    fn set_length(&mut self, length: usize) {
        match self {
            Type::File(file) => file.length = length,
            Type::Space(space) => space.length = length,
        };
    }
}

fn defrag_blockwise(data: &[Type]) -> Vec<Type> {
    let mut data = data
        .iter()
        .flat_map(|t| {
            let length = t.length();
            let t = match t {
                Type::File(f) => Type::File(File {
                    length: 1,
                    ..f.clone()
                }),
                Type::Space(s) => Type::Space(Space {
                    length: 1,
                    ..s.clone()
                }),
            };

            iter::repeat_n(t, length)
        })
        .collect::<Vec<_>>();

    let mut d = data.as_mut_slice();
    loop {
        let a = d.iter().position(|t| matches!(t, Type::Space(_)));
        let b = d.iter().rposition(|t| matches!(t, Type::File(_)));
        match (a, b) {
            (Some(a), Some(b)) if a < b => {
                d.swap(a, b);
                d = &mut d[a + 1..b];
            }
            _ => break,
        }
    }
    data
}

fn defrag_filewise(data: &[Type]) -> Vec<Type> {
    let mut data = data.to_vec();
    let mut last_index = data.len();

    while let Some(file_index) = data[..last_index]
        .iter()
        .rposition(|t| matches!(t, Type::File(_)))
    {
        last_index = file_index;
        let file_length = data[file_index].length();

        if let Some(space_index) = data
            .iter()
            .position(|t| matches!(t, Type::Space(_)) && t.length() >= file_length)
        {
            if space_index < file_index {
                let space_length = data[space_index].length();
                if space_length == file_length {
                    data.swap(file_index, space_index);
                } else {
                    data.insert(
                        space_index,
                        Type::Space(Space {
                            length: file_length,
                        }),
                    );
                    data[space_index + 1].set_length(space_length - file_length);
                    data.swap(space_index, file_index + 1);
                }
            }
        }
    }
    data
}

fn calc_checksum(data: &[Type]) -> usize {
    data.iter()
        .scan(0, |index, t| {
            let x = (*index, t);
            *index += t.length();
            Some(x)
        })
        .filter_map(|(index, t)| match t {
            Type::File(file) => Some((index..index + file.length).sum::<usize>() * file.id),
            Type::Space(_) => None,
        })
        .sum::<usize>()
}

fn main() {
    let input = include_str!("../input/input.txt");

    let data = input
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .scan((true, 0), |(is_file, id), length| {
            let t = match is_file {
                true => Type::File(File { id: *id, length }),
                false => Type::Space(Space { length }),
            };
            if *is_file {
                *id += 1;
            }
            *is_file = !*is_file;
            Some(t)
        })
        .collect::<Vec<_>>();

    let data_1 = defrag_blockwise(&data);
    let checksum_1 = calc_checksum(&data_1);

    println!("The first checksum is {}.", checksum_1);

    let data_2 = defrag_filewise(&data);
    let checksum_2 = calc_checksum(&data_2);

    println!("The second checksum is {}.", checksum_2);
}
