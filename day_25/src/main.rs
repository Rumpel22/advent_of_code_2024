struct Key {
    heights: [u8; 5],
}
struct Lock {
    heights: [u8; 5],
}

fn parse_height(input: &str) -> [u8; 5] {
    let mut heights = [0, 0, 0, 0, 0];
    input
        .chars()
        .filter(|c| c.is_ascii_graphic())
        .enumerate()
        .for_each(|(index, c)| heights[index % 5] += if c == '#' { 1 } else { 0 });
    heights.iter_mut().for_each(|h| *h -= 1);
    heights
}

fn parse(input: &str) -> (Vec<Key>, Vec<Lock>) {
    let mut keys = vec![];
    let mut locks = vec![];
    for i in input.split("\n\n") {
        let heights = parse_height(i);
        if i.starts_with('.') {
            keys.push(Key { heights });
        } else {
            locks.push(Lock { heights });
        }
    }
    (keys, locks)
}

impl Lock {
    fn matches(&self, key: &Key) -> bool {
        self.heights
            .iter()
            .zip(key.heights.iter())
            .all(|(l, k)| l + k <= 5)
    }
}

fn main() {
    let input = include_str!("../input/input.txt");
    let (keys, locks) = parse(input);

    let matches = keys
        .iter()
        .map(|key| locks.iter().filter(|lock| lock.matches(key)).count())
        .sum::<usize>();
    println!("There are {} lock/key pairs fitting together.", matches)
}
