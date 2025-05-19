use std::time::Instant;

fn solve(lines: Vec<&str>) {
    let important = lines
        .into_iter()
        .filter(|l| l.contains('x') || l.contains('y') || l.contains('z'))
        .collect::<Vec<_>>();

    for l in &important {
        println!("{}", l);
    }

    println!("len: {}", important.len());
}

fn main() {
    let lines = include_str!("input_test3.txt").lines().collect();

    let start = Instant::now();
    solve(lines);
    println!("Took: {:?}", Instant::now().duration_since(start));
}
