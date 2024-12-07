struct Condition {
    before: u32,
    after: u32,
}

type Conditions = Vec<Condition>;

fn is_valid_order(order: &[u32], conditions: &Conditions) -> bool {
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

fn sum_middle_pages(orders: &[Vec<u32>]) -> u32 {
    orders.iter().map(|order| order[order.len() / 2]).sum()
}

fn fix_order(order: &[u32], conditions: &Conditions) -> Vec<u32> {
    let mut new_order = Vec::new();

    for page in order {
        let index_to_insert = conditions
            .iter()
            .filter(|condition| condition.after == *page)
            .filter_map(|condition| new_order.iter().position(|x| *x == condition.before))
            .min()
            .unwrap_or(new_order.len());

        new_order.insert(index_to_insert, *page);
    }
    new_order
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

    let (valid_orders, invalid_orders): (Vec<_>, Vec<_>) = pages
        .iter()
        .partition(|order| is_valid_order(order, &conditions));
    let x = valid_orders
        .iter()
        .map(|order| order.to_owned().clone())
        .collect::<Vec<_>>();

    let sum = sum_middle_pages(&x);
    println!(
        "The sum of the middle pages of the valid orders is {}.",
        sum
    );

    let fixed_orders = invalid_orders
        .iter()
        .map(|order| fix_order(order, &conditions))
        .collect::<Vec<_>>();
    let sum = sum_middle_pages(&fixed_orders);
    println!(
        "The sum of the middle pages of the invalid orders is {}.",
        sum
    );
}
