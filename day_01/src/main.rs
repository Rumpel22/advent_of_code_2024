fn get_lists(input: &str) -> (Vec<i32>, Vec<i32>) {
    input
        .lines()
        .map(|line| {
            let mut numbers = line.split_ascii_whitespace();
            let i1: i32 = numbers.next().unwrap().parse().unwrap();
            let i2: i32 = numbers.next().unwrap().parse().unwrap();
            (i1, i2)
        })
        .collect::<(Vec<_>, Vec<_>)>()
}

fn main() {
    let input = include_str!("../input/input.txt");

    let (mut l1, mut l2) = get_lists(input);
    l1.sort();
    l2.sort();

    let total_distance = l1
        .iter()
        .zip(l2.iter())
        .map(|(i1, i2)| i1.abs_diff(*i2))
        .sum::<u32>();

    println!(
        "The total distance between the two lists is {}.",
        total_distance
    );

    let similarity_score = l1
        .iter()
        .map(|i1| l2.iter().filter(|i2| i1 == *i2).count() * *i1 as usize)
        .sum::<usize>();
    println!(
        "The similarity score between the two lists is {}.",
        similarity_score
    )
}
