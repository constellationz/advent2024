use input::read_stdin;

#[path = "../input.rs"]
mod input;

fn main() {
    let reports = read_reports(&read_stdin());
    println!("number of safe reports: {}", count_safe_reports(&reports));
    println!(
        "number of safe reports (dampened): {}",
        count_safe_reports_dampened(&reports)
    );
}

fn count_safe_reports(reports: &[Vec<i32>]) -> i32 {
    reports.iter().fold(0, |sum, report| {
        sum + if report_safe(report) { 1 } else { 0 }
    })
}

fn count_safe_reports_dampened(reports: &[Vec<i32>]) -> i32 {
    reports.iter().fold(0, |sum, report| {
        if report_safe(report) {
            sum + 1
        } else {
            sum + if (0..report.len()).any(|i| {
                let mut new_report = report.clone();
                new_report.remove(i);
                report_safe(&new_report)
            }) {
                1
            } else {
                0
            }
        }
    })
}

fn safe_consecutive(increasing: bool, number_1: i32, number_2: i32) -> bool {
    increasing == (number_2 > number_1) && (1..=3).contains(&number_1.abs_diff(number_2))
}

fn report_safe(report: &[i32]) -> bool {
    let increasing = report[1] > report[0];
    let mut prev_number: i32 = report[0];
    report.iter().skip(1).all(|number| {
        let safe = safe_consecutive(increasing, prev_number, *number);
        prev_number = *number;
        safe
    })
}

fn read_reports(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect()
        })
        .collect()
}
