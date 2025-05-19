use std::time::Instant;

fn solve(lines: Vec<&str>) -> u32 {
    todo!()
}

fn main() {
    let lines = include_str!("input0.txt")
        .lines()
        .filter(|&line| !line.is_empty())
        .collect();

    let start = Instant::now();
    println!("{}", solve(lines));
    println!("Took: {:?}", Instant::now().duration_since(start));
}
