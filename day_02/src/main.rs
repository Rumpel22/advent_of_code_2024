fn valid_report(report: &[i32]) -> bool {
    let mut diffs = report.windows(2).map(|pair| pair[1] - pair[0]);
    let signum = (report[1] - report[0]).signum();

    diffs.all(|diff| diff.abs() >= 1 && diff.abs() <= 3 && diff.signum() == signum)
}

fn main() {
    let input = include_str!("../input/input.txt");
    let reports = input
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|number| number.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let safe_reports = reports.iter().filter(|report| valid_report(report)).count();

    println!("There are {} safe levels.", safe_reports);
}
