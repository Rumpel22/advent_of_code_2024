struct Condition {
    before: u32,
    after: u32,
}

type Conditions = Vec<Condition>;

fn valid_order(order: &[u32], conditions: &Conditions) -> bool {
    for index in 0..order.len() {
        let remaining = &order[index..];
        let page = order[index];

        if conditions
            .iter()
            .filter(|condition| condition.after == page)
            .map(|condition| condition.before)
            .any(|before| remaining.contains(&before))
        {
            return false;
        }
    }
    true
}

fn main() {
    let input = include_str!("../input/input.txt");
    let (upper_part, lower_part) = input.split_once("\n\n").unwrap();
    let conditions = upper_part
        .lines()
        .map(|line| {
            let (a, b) = line.split_once('|').unwrap();
            let before = a.parse().unwrap();
            let after = b.parse().unwrap();
            Condition { before, after }
        })
        .collect::<Vec<_>>();
    let pages = lower_part
        .lines()
        .map(|line| {
            line.split(',')
                .map(|number| number.parse::<u32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let sum = pages
        .iter()
        .filter(|order| valid_order(order, &conditions))
        .map(|pages| pages[pages.len() / 2])
        .sum::<u32>();
    println!("The requested sum is {}.", sum);
}
