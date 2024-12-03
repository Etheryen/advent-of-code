fn to_reports(lines: Vec<&str>) -> Vec<Vec<u32>> {
    lines
        .into_iter()
        .map(|line| {
            line.split_whitespace()
                .map(|level| level.parse().unwrap())
                .collect()
        })
        .collect()
}

fn is_increasing_or_decreasing(report: &[u32]) -> bool {
    report.is_sorted() || report.is_sorted_by(|a, b| a > b)
}

fn is_differing_slightly(report: &[u32]) -> bool {
    let mut previous = report.first().unwrap();

    for level in report.iter().skip(1) {
        let diff = (*level as i32 - *previous as i32).unsigned_abs();
        if !(1..=3).contains(&diff) {
            return false;
        }
        previous = level;
    }

    true
}

fn is_safe(report: &[u32]) -> bool {
    is_increasing_or_decreasing(report) && is_differing_slightly(report)
}

fn to_safe(reports: Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    reports
        .into_iter()
        .filter(|report| is_safe(report))
        .collect()
}

fn solve(lines: Vec<&str>) -> usize {
    let reports = to_reports(lines);
    to_safe(reports).len()
}

fn main() {
    let lines = include_str!("input.txt")
        .split('\n')
        .filter(|&line| !line.is_empty())
        .collect();
    println!("{}", solve(lines));
}
